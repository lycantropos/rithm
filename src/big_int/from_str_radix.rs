use crate::traits::FromStrRadix;

use super::constants::{MAX_REPRESENTABLE_BASE, MIN_REPRESENTABLE_BASE};
use super::digits::FromStrDigit;
use super::types::{BigInt, TryFromStringError};

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize> FromStrRadix
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = TryFromStringError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error> {
        if radix != 0
            && (radix < (MIN_REPRESENTABLE_BASE as u32) || radix > (MAX_REPRESENTABLE_BASE as u32))
        {
            Err(TryFromStringError::BaseOutOfBounds(radix))
        } else {
            Self::try_from_string(string, radix as u8)
        }
    }
}
