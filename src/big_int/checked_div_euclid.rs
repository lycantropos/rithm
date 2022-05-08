use crate::traits::CheckedDivEuclid;

use super::digits::{checked_div_euclid, EuclidDivisibleDigit};
use super::types::BigInt;

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: &Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}
