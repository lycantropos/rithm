use traiter::numbers::FromStrRadix;

use super::constants::{MAX_REPRESENTABLE_BASE, MIN_REPRESENTABLE_BASE};
use super::try_from_string::TryFromString;
use super::types::{BigInt, TryFromStringError};

impl<Digit, const DIGIT_BITNESS: usize> FromStrRadix
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: TryFromString,
{
    type Error = TryFromStringError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error> {
        if radix != 0
            && (radix < u32::from(MIN_REPRESENTABLE_BASE)
                || radix > u32::from(MAX_REPRESENTABLE_BASE))
        {
            Err(TryFromStringError::BaseOutOfBounds(radix))
        } else {
            Self::try_from_string(string, radix as u8)
        }
    }
}
