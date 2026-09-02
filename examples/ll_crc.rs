//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! This example demonstrates how to set up the CRC check using the low-level driver.
//! The CRC must be set twice: once in the interface and once by sending to the LTC2686 that it
//! should check for the CRC. Missing any of those will lead to problems!
//!
//! After setting up the CRC, the `ll_simple.rs` examples are executed.
//! Finally, we double check if any faults occured by reading them and
//! printing the result.
//!
//! ```text
//! cargo run --example ll_crc
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

    // tell interface to use CRC.
    ltc.interface().set_crc_check(ll::CrcCheck::Enabled);
    // tell the LTC2686 to use CRC as well.
    ltc.configuration()
        .write_async(|reg| reg.set_crc_check(ll::CrcCheck::Enabled))
        .await
        .unwrap();

    // from here on, same as `ll_simple.rs` example
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

    assert_eq!(ch0_read, 0xAA_AA);
    println!("Read back DAC code: 0x{ch0_read:04X}");

    // Let's check for faults and print them. If all went well this should print:
    //
    // `Fault { tsf: Ok, ckf: Ok, cf: Ok }`
    let faults = ltc.fault().read_async().await.unwrap();
    println!("{:?}", faults);
}
