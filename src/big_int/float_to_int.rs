use std::convert::{FloatToInt, TryFrom};

use super::digits::DigitConvertibleFromFloat;
use super::types::BigInt;

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize>
    FloatToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f32
{
    unsafe fn to_int_unchecked(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize>
    FloatToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f64
{
    unsafe fn to_int_unchecked(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}
