use std::ops::Div;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::digits::checked_div;
use super::digits::DivisibleDigit;
use super::types::BigInt;

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Div
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div::<Digit, SHIFT>(
            self.sign,
            self.digits.as_slice(),
            divisor.sign,
            divisor.digits.as_slice(),
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Div<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div(self, divisor: &Self) -> Self::Output {
        let (sign, digits) = checked_div::<Digit, SHIFT>(
            self.sign,
            self.digits.as_slice(),
            divisor.sign,
            divisor.digits.as_slice(),
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    Div<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = checked_div::<Digit, SHIFT>(
            self.sign,
            self.digits.as_slice(),
            divisor.sign,
            divisor.digits.as_slice(),
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Div
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div::<Digit, SHIFT>(
            self.sign,
            self.digits.as_slice(),
            divisor.sign,
            divisor.digits.as_slice(),
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}
