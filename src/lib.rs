//! Driver for the AnalogDevices LTC2686 8 channel DAC.
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
//! │                                                                                              │
//! │ width: 100                                                                                   │
//! │                                                                                              │
//! └──────────────────────────────────────────────────────────────────────────────────────────────┘
//! ```

#![no_std]

pub mod ll;

#[cfg(feature = "measurements")]
mod voltage;

pub use ll::Span;

/// Trait that allows creating your own wrapper around a measurement to send to the DAC.
///
/// This trait is implemented for `u16`, where the value is simply passed through and no
/// calculations take place. If the "measurements" feature is activated, this trait is also
/// implemented for [`measurements::Voltage`], which means that actual voltages can be sent to the
/// device.
pub trait AsDacValue {
    fn as_dac_value(&self, span: Span) -> u16;
}

impl AsDacValue for u16 {
    fn as_dac_value(&self, _span: Span) -> u16 {
        *self
    }
}

/// FIXME: This is just a placeholder to demonstrate generic argument.
pub fn set_channel_dac_code<DAC: AsDacValue>(value: DAC) -> u16 {
    value.as_dac_value(Span::Unip5)
}
