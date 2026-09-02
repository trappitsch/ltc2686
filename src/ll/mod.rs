//! Low-level driver for the LTC2686.

#[cfg(feature = "async")]
pub use interface_async::LtcInterfaceAsync;
#[cfg(feature = "blocking")]
pub use interface_blocking::LtcInterface;

#[cfg(feature = "async")]
pub mod interface_async;
#[cfg(feature = "blocking")]
pub mod interface_blocking;

mod helpers;

device_driver::compile!(
    manifest: "ltc2686_16bit.ddsl"
);

#[derive(Debug)]
pub enum InterfaceError {
    /// A communication error with the driver occured.
    CommunicationError,
    /// An error occurred when toggling the chip select pin.
    CsPinError,
    /// An error occurred when toggling the reset pin.
    ResetPinError,
}
