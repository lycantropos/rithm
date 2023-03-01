use std::convert::TryFrom;

use traiter::numbers::Zeroable;

use crate::traits::UncheckedToInt;

use super::types::BigInt;

impl<Digit: Copy + Zeroable, const DIGIT_BITNESS: usize>
    UncheckedToInt<BigInt<Digit, DIGIT_BITNESS>> for f32
where
    f32: From<Digit> + UncheckedToInt<Digit>,
{
    unsafe fn unchecked_to_int(self) -> BigInt<Digit, DIGIT_BITNESS> {
        BigInt::<Digit, DIGIT_BITNESS>::try_from(self).unwrap_unchecked()
    }
}

impl<Digit: Copy + Zeroable, const DIGIT_BITNESS: usize>
    UncheckedToInt<BigInt<Digit, DIGIT_BITNESS>> for f64
where
    f64: From<Digit> + UncheckedToInt<Digit>,
{
    unsafe fn unchecked_to_int(self) -> BigInt<Digit, DIGIT_BITNESS> {
        BigInt::<Digit, DIGIT_BITNESS>::try_from(self).unwrap_unchecked()
    }
}
