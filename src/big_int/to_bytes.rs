use std::convert::{From, TryFrom};

use traiter::numbers::{Endianness, Signed, ToBytes, Zeroable};

use crate::traits::Oppose;

use super::constants::MIDDLE_BYTE;
use super::digits::{negate_digits, BinaryBaseFromBinaryDigits};
use super::types::BigInt;

impl<
        Digit: BinaryBaseFromBinaryDigits<Digit>
            + Copy
            + From<u8>
            + Oppose
            + Zeroable,
        const SEPARATOR: char,
        const SHIFT: usize,
    > ToBytes for BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: TryFrom<Digit>,
{
    type Output = Vec<u8>;

    fn to_bytes(&self, endianness: Endianness) -> Self::Output {
        let mut result = Digit::binary_base_from_binary_digits(
            &self.digits,
            SHIFT,
            u8::BITS as usize,
        )
        .iter()
        .map(|&byte| unsafe { u8::try_from(byte).unwrap_unchecked() })
        .collect::<Vec<u8>>();
        let most_significant_byte = result[result.len() - 1];
        if most_significant_byte >= MIDDLE_BYTE
            && !(most_significant_byte == MIDDLE_BYTE
                && result.iter().rev().skip(1).all(Zeroable::is_zero)
                && self.is_negative())
        {
            result.push(0u8);
        }
        if self.is_negative() {
            negate_digits(&mut result);
        }
        match endianness {
            Endianness::Big => result.reverse(),
            Endianness::Little => {}
        }
        result
    }
}
