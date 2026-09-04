//! Module that holds the low-level driver for the LTC2686 and associated types.
//!
//! The low-level driver was implemented using [device-driver](https://device-driver.com).
//! Examples on its usage are given in the repository's `examples` folder.
//! Async and blocking interfaces are implemented based on the
//! [`embedded-hal-async`](https://docs.rs/embedded-hal-async)
//! and [`embedded-hal`](https://docs.rs/embedded-hal) traits.
//!
//! To get an overview of what is possible with the low-level driver,
//! start with the [`Ltc2686Ll`] and branch out from there.

pub use impls::SpanRangeVolts;

#[cfg(feature = "async")]
pub use interface_async::LtcInterfaceAsync;
#[cfg(feature = "blocking")]
pub use interface_blocking::LtcInterfaceBlocking;

#[cfg(feature = "async")]
mod interface_async;
#[cfg(feature = "blocking")]
mod interface_blocking;

#[cfg(any(feature = "async", feature = "blocking"))]
mod crc;
mod impls;

device_driver::compile!(
    manifest: "ltc2686_16bit.ddsl"
);

#[derive(Debug)]
/// Interface error.
///
/// Errors that might be encountered when communicating when using the async or blocking interface.
pub enum InterfaceError {
    /// A communication error with the driver occured.
    CommunicationError,
    /// An error occurred when toggling the chip select pin.
    CsPinError,
    /// An error occurred when toggling the reset pin.
    ResetPinError,
}
