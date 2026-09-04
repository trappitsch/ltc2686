//! Module to handle CRC calculation.

use crate::ll::CrcCheck;

/// Get CRC-6 for a given setup.
///
/// If [`CrcCheck`] is disabled, returns 0x00 (as we don't care about the CRC).
pub fn get_crc(crc_check: &CrcCheck, address: u8, data: &[u8]) -> u8 {
    if crc_check == &CrcCheck::Disabled {
        0
    } else {
        calc_crc(address, data)
    }
}

/// Calculate CRC-6 for communicating with device.
///
/// Polynomial: x**6 + x + 1
///
/// Important: Data must always be 2 bytes long!
/// If data is too short, this will panic.
///
/// All casts in this routine are safe:
/// - u8 -> u32: works always
/// - u32 -> u8: only last cast: but bitmask that is applied is < u8::MAX.
fn calc_crc(address: u8, data: &[u8]) -> u8 {
    // Prepare the start value of the result: 24 bit for value + 2 from 4th byte, last 6 bits for CRC.
    let mut result: u32 = ((address as u32) << 16 | (data[0] as u32) << 8 | (data[1] as u32)) << 8;

    // Prepare the start polynomial: `0b1000011` (7 bit) must be made into 32 bit to match input
    let mut poly: u32 = 0x03 << 25; // 7 bit polynomial + 23bit

    for msbp in (6..32).rev() {
        if result & (1 << msbp) != 0 {
            // so we xor with poly
            result ^= poly;
        }
        poly >>= 1;
    }

    // bit mask for last 6, cast is always valid as mask < u8::MAX
    (result & 0b11_1111) as u8
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0x00, [0x00, 0x00]), 0x00)]
    #[case((0x00, [0x00, 0x43]), 0x00)]
    #[case((0x40, [0x00, 0x00]), 0x33)]
    #[case((0x40, [0x12, 0x34]), 0x16)]
    #[case((0x40, [0x7F, 0x7F]), 0x39)]
    #[case((0x40, [0xAB, 0xCD]), 0x29)]
    #[case((0x40, [0xFF, 0xFF]), 0x38)]
    fn test_calc_crc_read_ch0_dac_code(
        #[case] (address, data): (u8, [u8; 2]),
        #[case] crc_exp: u8,
    ) {
        assert_eq!(calc_crc(address, &data), crc_exp);
    }
}
