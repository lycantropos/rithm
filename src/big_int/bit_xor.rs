use std::ops::BitXor;

use super::digits::{bitwise_xor, BinaryDigit};
use super::types::BigInt;

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXor
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits)
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXor<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitxor(self, other: &Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits.clone())
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits.clone(), self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize>
    BitXor<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitxor(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXor
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
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
        Self::Output { sign, digits }
    }
}
