use std::ops::BitAndAssign;

use super::digits::{bitwise_and_components, BitwiseConjunctiveDigit};
use super::types::BigInt;

impl<Digit: BitwiseConjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitAndAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = bitwise_and_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}

impl<Digit: BitwiseConjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitAndAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: Self) {
        (self.sign, self.digits) = bitwise_and_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}
