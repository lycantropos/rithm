use crate::traits::{Endianness, FromBytes, Oppose, Unitary, Zeroable};

use super::constants::MIDDLE_BYTE;
use super::contracts::is_valid_shift;
use super::digits::{
    binary_digits_to_binary_base, negate_digits, to_digits_sign, BinaryDigitConvertibleToBinary,
};
use super::types::{BigInt, Sign};

impl<Digit: Oppose, const SEPARATOR: char, const SHIFT: usize> FromBytes
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: BinaryDigitConvertibleToBinary<Digit>,
{
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        let mut bytes = bytes.to_vec();
        match endianness {
            Endianness::Big => bytes.reverse(),
            Endianness::Little => {}
        }
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        let most_significant_byte = bytes[bytes.len() - 1];
        let sign = if most_significant_byte >= MIDDLE_BYTE {
            negate_digits(&mut bytes);
            -Sign::one()
        } else {
            to_digits_sign(&bytes)
        };
        Self {
            sign,
            digits: binary_digits_to_binary_base::<u8, Digit>(
                &bytes[..bytes.len()
                    - ((bytes.len() > 1 && bytes[bytes.len() - 1].is_zero()) as usize)],
                u8::BITS as usize,
                SHIFT,
            ),
        }
    }
}
