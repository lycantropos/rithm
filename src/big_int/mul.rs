use std::ops::Mul;

use super::digits::{multiply_digits, MultiplicativeDigit};
use super::types::BigInt;

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> Mul
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> Mul<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Mul<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn mul(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> Mul
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}
