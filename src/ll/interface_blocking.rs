//! The blocking interface impl for LTC2686.

use device_driver::{
    CommandInterface, CommandInterfaceBase, RegisterInterface, RegisterInterfaceBase,
};
use embedded_hal::{
    delay::DelayNs,
    digital::OutputPin,
    spi::{Operation, SpiDevice},
};

use crate::ll::InterfaceError;

pub struct LtcInterface<SPI, R, D>
where
    SPI: SpiDevice,
    R: OutputPin,
    D: DelayNs,
{
    /// The SPI device interface used to communicate.
    spi: SPI,
    /// The output pin where the nCLR pin used to reset the chip is connected to.
    reset_pin: R,
    /// A delay provider.
    delay: D,
}

impl<SPI: SpiDevice, R: OutputPin, D: DelayNs> LtcInterface<SPI, R, D> {
    /// Try to create a new blocking interface for the LTC2686.
    ///
    /// Arguments:
    /// - spi: The blocking SPI device to use.
    /// - reset_pin: Pin where nCLR is connected to.
    /// - delay: Some delay provider.
    ///
    /// Error:
    /// - Cannot set nCLR reset pin to high.
    pub fn try_new(spi: SPI, mut reset_pin: R, delay: D) -> Result<Self, InterfaceError> {
        // make sure the reset pin is set to high (no reset)
        reset_pin
            .set_high()
            .map_err(|_| InterfaceError::ResetPinError)?;

        Ok(Self {
            spi,
            reset_pin,
            delay,
        })
    }

    /// Resets the LTC2686 by pulling the reset pin low.
    ///
    /// This clears the device to zero-scale and a 0V to 5V span range. The control registers are
    /// cleared to all default values.
    ///
    /// Error:
    /// - Cannot set nCLR reset pin to high/low.
    pub fn reset(&mut self) -> Result<(), InterfaceError> {
        self.reset_pin
            .set_low()
            .map_err(|_| InterfaceError::ResetPinError)?;

        self.delay.delay_ns(8);

        self.reset_pin
            .set_high()
            .map_err(|_| InterfaceError::ResetPinError)?;

        Ok(())
    }
}

impl<SPI: SpiDevice, R: OutputPin, D: DelayNs> RegisterInterfaceBase for LtcInterface<SPI, R, D> {
    type Error = InterfaceError;
    type AddressType = u8;
}

impl<SPI: SpiDevice, R: OutputPin, D: DelayNs> CommandInterfaceBase for LtcInterface<SPI, R, D> {
    type Error = InterfaceError;
    type AddressType = u8;
}

impl<SPI: SpiDevice, R: OutputPin, D: DelayNs> RegisterInterface for LtcInterface<SPI, R, D> {
    fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        self.spi
            .transaction(&mut [Operation::Write(&[address]), Operation::Write(data)])
            .map_err(|_| Self::Error::CommunicationError)?;

        Ok(())
    }

    fn read_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        let address_read = address + 0x80;

        // buffers for returned address
        let mut address_answer = [0_u8];

        // write the read request
        self.spi
            .transaction(&mut [
                Operation::Write(&[address_read]),
                Operation::Write(&[0x00, 0x00]),
            ])
            .map_err(|_| Self::Error::CommunicationError)?;

        // read back
        self.spi
            .transaction(&mut [Operation::Read(&mut address_answer), Operation::Read(data)])
            .map_err(|_| Self::Error::CommunicationError)?;

        Ok(())
    }
}

impl<SPI: SpiDevice, R: OutputPin, D: DelayNs> CommandInterface for LtcInterface<SPI, R, D> {
    fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        input: &mut [u8],
        _input_metadata: &device_driver::FieldsetMetadata,
        _output: &mut [u8],
        _output_metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        self.spi
            .transaction(&mut [Operation::Write(&[address]), Operation::Write(input)])
            .map_err(|_| Self::Error::CommunicationError)?;
        Ok(())
    }
}
