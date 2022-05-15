use std::convert::{From, TryFrom};

use crate::traits::{Endianness, Oppose, Signed, ToBytes, Zeroable};

use super::constants::MIDDLE_BYTE;
use super::digits::{binary_digits_to_binary_base, negate_digits, BinaryDigitConvertibleToBinary};
use super::types::BigInt;

impl<
        Digit: BinaryDigitConvertibleToBinary<Digit> + From<u8> + Oppose + Zeroable,
        const SEPARATOR: char,
        const SHIFT: usize,
    > ToBytes for &BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: TryFrom<Digit>,
{
    type Output = Vec<u8>;

    fn to_bytes(self, endianness: Endianness) -> Self::Output {
        let mut result =
            binary_digits_to_binary_base::<Digit, Digit>(&self.digits, SHIFT, u8::BITS as usize)
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
