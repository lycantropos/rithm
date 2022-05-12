use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::DivEuclid;

use super::digits::{checked_div_euclid, EuclidDivisibleDigit};
use super::types::BigInt;

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div_euclid(self, divisor: &Self) -> Self::Output {
        let (sign, digits) = checked_div_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    DivEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = checked_div_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}
