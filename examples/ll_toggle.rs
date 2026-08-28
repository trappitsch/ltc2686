//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! This example sets up toggle mode for channel 0 between 2V (off) and 3V (on).
//! The software toggle will be used.
//! Then we will toggle the channel between the two states every second (500 ms between toggles) for
//! a total of 10 seconds.
//! Finally, we will turn the toggle mode off again, which will leave the output in its register A
//! state, here 2V.
//!
//! ```text
//! cargo run --example ll_toggle
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

    let channel = Channel::Ch0;

    // Configure channel 0 settings for toggle mode with software toggle "pin".
    // Note that we leave the default span (0 - 5 V).
    ltc.channel_op(channel)
        .channel_settings()
        .write_async(|reg| {
            reg.set_mode(ll::Mode::Toggle);
            reg.set_td_select(ll::TdSelect::Software);
        })
        .await
        .unwrap();

    // Set input register A to 2V.
    ltc.channel_op(channel)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(0x66_66))
        .await
        .unwrap();

    // Select input register B as the one to write to.
    ltc.input_register_select()
        .write_async(|reg| reg.set_ab_ch_0(ll::InputRegister::B))
        .await
        .unwrap();
    // Write 3V into input register B.
    ltc.channel_op(channel)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(0x99_99))
        .await
        .unwrap();

    // Enable toggle mode.
    ltc.toggle_dither_enable()
        .write_async(|reg| reg.set_tde_ch_0(ll::ToggleDitherState::Enabled))
        .await
        .unwrap();

    // Toggle on/off every second for 10 seconds.
    for _ in 0..10 {
        ltc.software_toggle()
            .write_async(|reg| reg.set_swt_ch_0(ll::SoftwareToggleState::Enabled))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;

        ltc.software_toggle()
            .write_async(|reg| reg.set_swt_ch_0(ll::SoftwareToggleState::Disabled))
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Disable toggle mode.
    ltc.toggle_dither_enable()
        .write_async(|reg| reg.set_tde_ch_0(ll::ToggleDitherState::Enabled))
        .await
        .unwrap();
}
