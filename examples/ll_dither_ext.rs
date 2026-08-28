//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! This example sets up the following output:
//!
//! - Channel 0 channel and dither setup:
//!   - Voltage span: 0-5V.
//!   - Baseline voltage (input register A): 2V.
//!   - Amplitude 1V.
//!   - Phase shift: 0°.
//!   - Period: 32 clock signals.
//!   - External trigger on TPG2.
//!
//! - Channel 1 channel and dither setup:
//!   - Voltage span: 0-10V.
//!   - Baseline voltage (input register A): 5V.
//!   - Amplitude 2.5V.
//!   - Phase shift: 90°.
//!   - Period: 32 clock signals.
//!   - External trigger on TPG2.
//!
//! The clock is genrated with a function generator, the traces recorded on an oscilloscope.
//!
//! ```text
//! cargo run --example ll_dither_ext
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

    // setup channel 0
    ltc.channel_op(Channel::Ch0)
        .channel_settings()
        .write_async(|reg| {
            reg.set_mode(ll::Mode::Dither);
            reg.set_td_select(ll::TdSelect::TgpPin2);
            reg.set_dither_period(ll::DitherPeriod::N32);
            reg.set_span_settings(ll::SpanSettings::Sp0To5);
        })
        .await
        .unwrap();

    // setup channel 1:
    ltc.channel_op(Channel::Ch1)
        .channel_settings()
        .write_async(|reg| {
            reg.set_mode(ll::Mode::Dither);
            reg.set_td_select(ll::TdSelect::TgpPin2);
            reg.set_dither_period(ll::DitherPeriod::N32);
            reg.set_dither_phase(ll::DitherPhase::P90);
            reg.set_span_settings(ll::SpanSettings::Sp0To10);
        })
        .await
        .unwrap();

    // baseline of channel 0
    ltc.channel_op(Channel::Ch0)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(0x66_66))
        .await
        .unwrap();

    // baseline of channel 1
    ltc.channel_op(Channel::Ch1)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(0x7F_FF))
        .await
        .unwrap();

    // Switch to control input register B for both channels.
    ltc.input_register_select()
        .write_async(|reg| {
            reg.set_ab_ch_0(ll::InputRegister::B);
            reg.set_ab_ch_1(ll::InputRegister::B);
        })
        .await
        .unwrap();

    // Set the amplitude of channel 0.
    let amplitude = (u16::MAX / 5) << 2;
    println!("Amplitude 1V: code 0x{amplitude:04X}");
    ltc.channel_op(Channel::Ch0)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(amplitude))
        .await
        .unwrap();

    // Set the amplitude of channel 1.
    let amplitude = (u16::MAX / 100 * 25) << 2;
    println!("Amplitude 2.5V: code 0x{amplitude:04X}");
    ltc.channel_op(Channel::Ch1)
        .channel_dac_code()
        .write_async(|reg| reg.set_code_16_bit(amplitude))
        .await
        .unwrap();

    // Start the dither mode on both channels.
    ltc.toggle_dither_enable()
        .write_async(|reg| {
            reg.set_tde_ch_0(ll::ToggleDitherState::Enabled);
            reg.set_tde_ch_1(ll::ToggleDitherState::Enabled);
        })
        .await
        .unwrap();
}
