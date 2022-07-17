use std::ops::Not;

use crate::big_int::digits::InvertComponents;

use super::types::BigInt;

impl<Digit: InvertComponents, const SEPARATOR: char, const SHIFT: usize> Not
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let (sign, digits) =
            Digit::invert_components::<SHIFT>(self.sign, &self.digits);
        Self { sign, digits }
    }
}

impl<Digit: InvertComponents, const SEPARATOR: char, const SHIFT: usize> Not
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn not(self) -> Self::Output {
        let (sign, digits) =
            Digit::invert_components::<SHIFT>(self.sign, &self.digits);
        Self::Output { sign, digits }
    }
}
