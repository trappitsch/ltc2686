//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! The following is done:
//!
//! 1. Setup the interface.
//! 2. Reset the LTC2686.
//! 3. Set input register DAC code of channe 0 to `0xFF_FF`.
//! 4. Update channel 0.
//! 5. Set input register DAC code of channe 0 to `0xAA_AA`.
//! 6. Read back the DAC code in the input register and print it to console.
//!
//! ```text
//! cargo run --example ll_simple
//! ```

use std::time::Duration;

use device_driver::Block;
use pico_de_gallo_hal::{Hal, SpiPhase};

use ltc2686::ll::{Channel, Ltc2686Ll, LtcInterfaceAsync};

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

    let channel = Channel::Ch0;
    ltc.channel_op(channel)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(0xFF_FF))
        .await
        .unwrap();

    ltc.channel_op(channel)
        .channel_update()
        .dispatch_in_async(|_| {})
        .await
        .unwrap();

    ltc.channel_op(channel)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(0xAA_AA))
        .await
        .unwrap();

    let ch0_read = ltc
        .channel_op(channel)
        .channel_dac_code()
        .read_async()
        .await
        .unwrap()
        .code_16_bit();
    println!("Read back DAC code: 0x{ch0_read:04X}");
}
