use std::convert::TryFrom;

use crate::traits::UncheckedToInt;

use super::digits::DigitConvertibleFromF64;
use super::types::BigInt;

impl<
        Digit: DigitConvertibleFromF64,
        const SEPARATOR: char,
        const SHIFT: usize,
    > UncheckedToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f32
{
    unsafe fn unchecked_to_int(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}

impl<
        Digit: DigitConvertibleFromF64,
        const SEPARATOR: char,
        const SHIFT: usize,
    > UncheckedToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f64
{
    unsafe fn unchecked_to_int(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}
