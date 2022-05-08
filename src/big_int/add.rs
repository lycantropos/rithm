use std::ops::Add;

use super::types::BigInt;
use crate::big_int::digits::{sum_signed_digits, AdditiveDigit};

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize>
    Add<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn add(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}
