//! Low-level driver for the LTC2686.

#[cfg(feature = "async")]
pub use interface_async::LtcInterfaceAsync;
#[cfg(feature = "blocking")]
pub use interface_blocking::LtcInterface;

#[cfg(feature = "async")]
pub mod interface_async;
#[cfg(feature = "blocking")]
pub mod interface_blocking;

device_driver::compile!(
    manifest: "ltc2686_16bit.ddsl"
);

#[derive(Debug)]
pub enum InterfaceError {
    CommunicationError,
    CsPinError,
    ResetPinError,
}
