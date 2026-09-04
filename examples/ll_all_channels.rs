//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! This example shows how to use the commands that will talk to all channels simultaneously.
//! These are all write-only commands, as the status of all channels cannot be simultaneously read.
//!
//! ```text
//! cargo run --example ll_all_channels
//! ```

use std::time::Duration;

use device_driver::Block;
use pico_de_gallo_hal::{Hal, SpiPhase};

use ltc2686::ll::{self, Ltc2686Ll, LtcInterfaceAsync};

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

    // Set up a span of 0-10V.
    ltc.write_settings_all_channels()
        .dispatch_in_async(|data| data.set_span(ll::Span::Unip10))
        .await
        .unwrap();

    // Set all channels to 5V by setting the respecive DAC code and then update all channels.
    //
    // Note that the `update_all` command requires an empty closure in the dispatch, as we need to
    // send "don't care" 2-bytes of data. The empty closure will automatically send 2 bytes of zeros.
    ltc.write_dac_code_all_channels()
        .dispatch_in_async(|data| data.set_code_16_bit(0x7FFF))
        .await
        .unwrap();
    ltc.update_all().dispatch_in_async(|_| {}).await.unwrap();

    // Sleep for 10 seconds (so we can check with a multimeter).
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Now set the output for all channels back to zero. Here we send the code and update in one.
    ltc.write_dac_code_all_channels_update()
        .dispatch_in_async(|data| data.set_code_16_bit(0x0000))
        .await
        .unwrap();
}
