use std::convert::TryFrom;

use traiter::numbers::Zeroable;

use crate::traits::UncheckedToInt;

use super::types::BigInt;

impl<Digit: Copy + Zeroable, const SEPARATOR: char, const SHIFT: usize>
    UncheckedToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f32
where
    f32: From<Digit> + UncheckedToInt<Digit>,
{
    unsafe fn unchecked_to_int(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}

impl<Digit: Copy + Zeroable, const SEPARATOR: char, const SHIFT: usize>
    UncheckedToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f64
where
    f64: From<Digit> + UncheckedToInt<Digit>,
{
    unsafe fn unchecked_to_int(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}
