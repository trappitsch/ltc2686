//! Run the LTC2686 driver against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the low-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using an async interface.
//!
//! This example sets up the following output:
//!
//! - Channel 0 baseline is at 2V with full range 0-5V.
//! - A dither is set up with 32 software toggles per period and an amplitude of 1V.
//! - For one full minute, the software toggled is toggled every `SWT_EDGE_EVERY` * 2.
//!
//! The signal can be observed on an oscilloscope or, alternatively as well, on a voltmeter by
//! looking at the output. The sinusoidal dither is slow enough that the rise and fall can clearly
//! be observed.
//!
//! ```text
//! cargo run --example ll_dither
//! ```

use std::time::Duration;

use device_driver::Block;
use pico_de_gallo_hal::{Hal, SpiPhase};

use ltc2686::ll::{self, Channel, Ltc2686Ll, LtcInterfaceAsync};
use tokio::time::Instant;

static SWT_EDGE_EVERY: Duration = Duration::from_millis(100);

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

    // set the channel up in software-toggle dither mode with 32 steps per period.
    ltc.channel_op(channel)
        .settings()
        .write_async(|reg| {
            reg.set_mode(ll::Mode::Dither);
            reg.set_td_select(ll::TdSelect::Software);
            reg.set_dither_period(ll::DitherPeriod::N32);
        })
        .await
        .unwrap();

    // The central voltage should be 2V.
    ltc.channel_op(channel)
        .dac_code()
        .write_async(|reg| reg.set_code_16_bit(0x66_66))
        .await
        .unwrap();

    // Set the amplitude to 1V.
    //
    // First, switch to right to input register B, then set the voltage.
    ltc.input_register_select()
        .write_async(|reg| reg.set_ab_ch_0(ll::InputRegister::B))
        .await
        .unwrap();

    // Now the complicated part (this is low-level after all!): as we are writing the command code we need to do the following:
    //
    // - select the value we want to send, max 1/4 the span at most.
    // - left shift by 2 so the two LSBs are 0 (thus 1/4 the span at most!)
    // - send the value
    let amplitude = (u16::MAX / 5) << 2;
    println!("Amplitude 1V: code 0x{amplitude:04X}");
    ltc.channel_op(channel)
        .dac_code()
        .write_async(|reg| reg.set_code_16_bit(amplitude))
        .await
        .unwrap();

    // Start the dither mode.
    ltc.toggle_dither_enable()
        .write_async(|reg| reg.set_tde_ch_0(ll::ToggleDitherState::Enabled))
        .await
        .unwrap();

    let run_for = Duration::from_secs(60);
    let run_until = Instant::now() + run_for;

    while Instant::now() < run_until {
        ltc.software_toggle()
            .write_async(|reg| reg.set_swt_ch_0(ll::SoftwareToggleState::Enabled))
            .await
            .unwrap();
        tokio::time::sleep(SWT_EDGE_EVERY).await;

        ltc.software_toggle()
            .write_async(|reg| reg.set_swt_ch_0(ll::SoftwareToggleState::Disabled))
            .await
            .unwrap();
        tokio::time::sleep(SWT_EDGE_EVERY).await;
    }

    // Stop the dither mode.
    ltc.toggle_dither_enable()
        .write_async(|reg| reg.set_tde_ch_0(ll::ToggleDitherState::Enabled))
        .await
        .unwrap();
}
