//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! Here we first the input register of channel 0 with a full output. As the chip was just reset,
//! the output span is by default 0 - 5V, thus we should measure 5V once updated.
//! We then set up channel 1 for a voltage range of 0 to 10V. The "full output" DAC code is then
//! written to the input register and all channels are updated in one command.
//!
//! For demonstration purposes (i.e., such that you can follow on a volt meter), we add some delays
//! and print statements.
//!
//! ```text
//! cargo run --example ll_span
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

    // Write full output (5V) to channel 0.
    ltc.channel_op(Channel::Ch0)
        .dac_code()
        .write_async(|reg| reg.set_code_16_bit(0xFF_FF))
        .await
        .unwrap();

    println!("Written DAC0 code, waiting 3 seconds...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("The wait is over!");

    // Set up channel 1 with a voltage range (span) of 0 - 10V.
    ltc.channel_op(Channel::Ch1)
        .settings()
        .write_async(|reg| reg.set_span(ll::Span::Unip10))
        .await
        .unwrap();

    // Write full output (10V) into channel 1 and update all DAC channels (will also set channel 0).
    ltc.channel_op(Channel::Ch1)
        .write_dac_code_update_all()
        .dispatch_in_async(|data| data.set_code_16_bit(0xFF_FF))
        .await
        .unwrap();
}
