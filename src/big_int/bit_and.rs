use std::ops::BitAnd;

use super::digits::{bitwise_and, BinaryDigit};
use super::types::BigInt;

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAnd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits)
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize>
    BitAnd<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitand(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAnd
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitand(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
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
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAnd<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitand(self, other: &Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits.clone())
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits.clone(), self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}
