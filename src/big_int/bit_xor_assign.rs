use std::ops::BitXorAssign;

use super::digits::{bitwise_xor, BinaryDigit};
use super::types::BigInt;

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXorAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXorAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_xor::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
    }
}
