use std::ops::BitAndAssign;

use super::digits::{bitwise_and, BinaryDigit};
use super::types::BigInt;

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAndAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_and::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAndAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
    }
}
