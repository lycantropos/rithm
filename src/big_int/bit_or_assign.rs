use std::ops::BitOrAssign;

use super::digits::{bitwise_or_components, BitwiseDisjunctiveDigit};
use super::types::BigInt;

impl<Digit: BitwiseDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitOrAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = bitwise_or_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}

impl<Digit: BitwiseDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitOrAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = bitwise_or_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}
