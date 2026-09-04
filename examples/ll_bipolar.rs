//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! Here, we use a bipolar range from -10V to +10V. Note that a negative power supply must be
//! connected, i.e., `V-` cannot be grounded.
//!
//! This example shows that setting the output to a DAC code of `0x00` will set the output to its
//! most negative value, namely -10V. This is in accordance to the spec sheet, Rev A., Figures 51
//! and 52. Zero is then at `u16::MAX / 2.0`. Note the float division here! The spec sheet has the
//! zero transition at 32768. If we do a float division of `u16::MAX / 2.0`, then round and
//! subsequently transfer to a `u16`, our result ends up being in agreement with the datasheet.
//!
//! ```text
//! cargo run --example ll_bipolar
//! ```

use std::time::Duration;

use device_driver::Block;
use pico_de_gallo_hal::{Hal, SpiPhase};

use ltc2686::ll::{self, Channel, Ltc2686Ll, LtcInterfaceAsync};

#[tokio::main]
async fn main() {
    let mut hal = Hal::new();

    hal.spi_set_config(
        5_000_000,
        SpiPhase::CaptureOnFirstTransition,
        pico_de_gallo_hal::SpiPolarity::IdleLow,
    )
    .unwrap();

    let spi_dev = hal.spi_device(0).unwrap();
    let reset_pin = hal.gpio(1);
    let delay = hal.delay();

    let interface = LtcInterfaceAsync::try_new(spi_dev, reset_pin, delay).unwrap();

    let mut ltc = Ltc2686Ll::new(interface);

    ltc.interface().reset().await.unwrap();

    tokio::time::sleep(Duration::from_millis(250)).await;

    let ch = Channel::Ch0;

    // Set up a bipolar span of +-10V.
    ltc.channel_op(ch)
        .settings()
        .write_async(|reg| reg.set_span(ll::Span::Bip10))
        .await
        .unwrap();

    // Set full output (+10V) to channel 0 and wait 10 seconds.
    ltc.channel_op(ch)
        .write_dac_code_update()
        .dispatch_in_async(|data| data.set_code_16_bit(0xFF_FF))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(10)).await;

    // Set full negative output (-10V) to channel 0 and wait 10 seconds.
    ltc.channel_op(ch)
        .write_dac_code_update()
        .dispatch_in_async(|data| data.set_code_16_bit(0x00_00))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(10)).await;

    // Set zero output (0V) to channel 0.
    ltc.channel_op(ch)
        .write_dac_code_update()
        .dispatch_in_async(|data| data.set_code_16_bit(0x80_00))
        .await
        .unwrap();
}
