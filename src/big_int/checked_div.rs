use crate::traits::CheckedDiv;

use super::digits::{checked_div, DivisibleDigit};
use super::types::BigInt;

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: &Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}
