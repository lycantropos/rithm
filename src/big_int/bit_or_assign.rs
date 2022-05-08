use std::ops::BitOrAssign;

use super::digits::{bitwise_or, BinaryDigit};
use super::types::BigInt;

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOrAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_or::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOrAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_or::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
    }
}
