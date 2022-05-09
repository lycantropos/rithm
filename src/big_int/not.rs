use std::ops::Not;

use super::digits::{invert_components, InvertibleDigit};
use super::types::BigInt;

impl<Digit: InvertibleDigit, const SEPARATOR: char, const SHIFT: usize> Not
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let (sign, digits) = invert_components::<Digit, SHIFT>(self.sign, &self.digits);
        Self { sign, digits }
    }
}

impl<Digit: InvertibleDigit, const SEPARATOR: char, const SHIFT: usize> Not
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn not(self) -> Self::Output {
        let (sign, digits) = invert_components::<Digit, SHIFT>(self.sign, &self.digits);
        Self::Output { sign, digits }
    }
}
