//! Test the low level driver for the LTC2686.

use device_driver::Block;
use embedded_hal_mock::{
    common::Generic,
    eh1::{
        delay::{CheckedDelay, Transaction as DelayTransaction},
        digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction},
        spi::{Mock as SpiMock, Transaction as SpiTransaction},
    },
};
use ltc2686::ll::{self, Channel, Ltc2686Ll, LtcInterface};

type SpiTransactionType = Generic<SpiTransaction<u8>>;
type PinTransactionType = Generic<PinTransaction>;
type Ltc2686Type = Ltc2686Ll<LtcInterface<SpiTransactionType, PinTransactionType, CheckedDelay>>;

fn get_new_ltc2686_blocking() -> (
    Ltc2686Type,
    SpiTransactionType,
    PinTransactionType,
    CheckedDelay,
) {
    let spi = SpiMock::new(&[]);
    let reset_pin = PinMock::new(&[PinTransaction::set(PinState::High)]);
    let delay = CheckedDelay::new(&[]);

    let interface = LtcInterface::try_new(spi.clone(), reset_pin.clone(), delay.clone()).unwrap();

    (Ltc2686Ll::new(interface), spi, reset_pin, delay)
}

#[test]
fn get_new_ltc_blocking() {
    let (_, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();

    spi.done();
    reset_pin.done();
    delay.done();
}

#[test]
fn reset_ltc() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();
    reset_pin.update_expectations(&[
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ]);
    delay.update_expectations(&[DelayTransaction::delay_ns(8)]);

    ltc.interface().reset().unwrap();

    spi.done();
    reset_pin.done();
    delay.done();
}

#[test]
fn set_channel_0_dac_code_to_0xab_cd() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x00),                 // address
        SpiTransaction::write_vec(vec![0xAB, 0xCD]), // data
        SpiTransaction::write(0x00),                 // address
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    ltc.channel_op(Channel::Ch0)
        .channel_dac_code()
        .write(|reg| reg.set_code_16_bit(0xABCD))
        .unwrap();

    spi.done();
    delay.done();
    reset_pin.done();
}

#[test]
fn set_channel_0_dac_code_to_0xab_cd_with_crc() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();
    ltc.interface().set_crc_check(ll::CrcCheck::Enabled);

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x00),                 // address
        SpiTransaction::write_vec(vec![0xAB, 0xCD]), // data
        SpiTransaction::write(0x1A),                 // address
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    ltc.channel_op(Channel::Ch0)
        .channel_dac_code()
        .write(|reg| reg.set_code_16_bit(0xABCD))
        .unwrap();

    spi.done();
    delay.done();
    reset_pin.done();
}

#[test]
fn get_channel_0_dac_code_0xab_cd() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x80),                 // address
        SpiTransaction::write_vec(vec![0x00, 0x00]), // does not matter
        SpiTransaction::write(0x00),                 // address
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::read(0x80), // TEST: Not sure on address
        SpiTransaction::read_vec(vec![0xAB, 0xCD]), // data
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    let ch0_dac_code = ltc
        .channel_op(Channel::Ch0)
        .channel_dac_code()
        .read()
        .unwrap();

    assert_eq!(ch0_dac_code.code_16_bit(), 0xAB_CD);

    spi.done();
    reset_pin.done();
    delay.done();
}

#[test]
fn get_channel_0_dac_code_0xab_cd_with_crc() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();
    ltc.interface().set_crc_check(ll::CrcCheck::Enabled);

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x80),                 // address
        SpiTransaction::write_vec(vec![0x00, 0x00]), // does not matter
        SpiTransaction::write(0x25),                 // address
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::read(0x80), // TEST: Not sure on address
        SpiTransaction::read_vec(vec![0xAB, 0xCD]), // data
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    let ch0_dac_code = ltc
        .channel_op(Channel::Ch0)
        .channel_dac_code()
        .read()
        .unwrap();

    assert_eq!(ch0_dac_code.code_16_bit(), 0xAB_CD);

    spi.done();
    reset_pin.done();
    delay.done();
}

#[test]
fn update_channel_4_dac_output() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_blocking();

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x68), // address
        SpiTransaction::write_vec(vec![0x00, 0x00]),
        SpiTransaction::write(0x00), // address
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    ltc.channel_op(Channel::Ch4)
        .channel_update()
        .dispatch_in(|_| {})
        .unwrap();

    spi.done();
    reset_pin.done();
    delay.done();
}
