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
use ltc2686::ll::{self, Channel, Ltc2686Ll, LtcInterfaceAsync};

type SpiTransactionType = Generic<SpiTransaction<u8>>;
type PinTransactionType = Generic<PinTransaction>;
type Ltc2686AsyncType =
    Ltc2686Ll<LtcInterfaceAsync<SpiTransactionType, PinTransactionType, CheckedDelay>>;

fn get_new_ltc2686_async() -> (
    Ltc2686AsyncType,
    SpiTransactionType,
    PinTransactionType,
    CheckedDelay,
) {
    let spi = SpiMock::new(&[]);
    let reset_pin = PinMock::new(&[PinTransaction::set(PinState::High)]);
    let delay = CheckedDelay::new(&[]);

    let interface =
        LtcInterfaceAsync::try_new(spi.clone(), reset_pin.clone(), delay.clone()).unwrap();

    (Ltc2686Ll::new(interface), spi, reset_pin, delay)
}

#[test]
fn get_new_ltc_async() {
    let (_, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();

    spi.done();
    reset_pin.done();
    delay.done();
}

#[tokio::test]
async fn reset_ltc() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();
    reset_pin.update_expectations(&[
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ]);
    delay.update_expectations(&[DelayTransaction::delay_ns(8)]);

    ltc.interface().reset().await.unwrap();

    spi.done();
    reset_pin.done();
    delay.done();
}

#[tokio::test]
async fn set_channel_0_dac_code_to_0xab_cd() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x00),                 // address
        SpiTransaction::write_vec(vec![0xAB, 0xCD]), // data
        SpiTransaction::write(0x00),                 // address
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    ltc.channel_op(Channel::Ch0)
        .dac_code()
        .write_async(|reg| reg.set_code_16_bit(0xABCD))
        .await
        .unwrap();

    spi.done();
    delay.done();
    reset_pin.done();
}

#[tokio::test]
async fn set_channel_0_dac_code_to_0xab_cd_with_crc() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();
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
        .dac_code()
        .write_async(|reg| reg.set_code_16_bit(0xABCD))
        .await
        .unwrap();

    spi.done();
    delay.done();
    reset_pin.done();
}

#[tokio::test]
async fn get_channel_0_dac_code_0xab_cd() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();

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
        .dac_code()
        .read_async()
        .await
        .unwrap();

    assert_eq!(ch0_dac_code.code_16_bit(), 0xAB_CD);

    spi.done();
    reset_pin.done();
    delay.done();
}

#[tokio::test]
async fn get_channel_0_dac_code_0xab_cd_with_crc() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();
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
        .dac_code()
        .read_async()
        .await
        .unwrap();

    assert_eq!(ch0_dac_code.code_16_bit(), 0xAB_CD);

    spi.done();
    reset_pin.done();
    delay.done();
}

#[tokio::test]
async fn update_channel_4_dac_output() {
    let (mut ltc, mut spi, mut reset_pin, mut delay) = get_new_ltc2686_async();

    let spi_expected = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write(0x68), // address
        SpiTransaction::write_vec(vec![0x00, 0x00]),
        SpiTransaction::write(0x00), // address
        SpiTransaction::transaction_end(),
    ];
    spi.update_expectations(&spi_expected);

    ltc.channel_op(Channel::Ch4)
        .update()
        .dispatch_in_async(|_| {})
        .await
        .unwrap();

    spi.done();
    reset_pin.done();
    delay.done();
}
