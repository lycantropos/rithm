use crate::traits::{CheckedPow, Pow};

use super::digits::ExponentiativeDigit;
use super::types::BigInt;

const NEGATIVE_EXPONENT_MESSAGE: &str = "Exponent should be non-negative.";

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> Pow<Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn pow(self, exponent: Self) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> Pow<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn pow(self, exponent: &Self) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Pow<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn pow(self, exponent: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> Pow<Self>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn pow(self, exponent: Self) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}
