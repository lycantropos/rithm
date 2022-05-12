use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::RemEuclid;

use super::digits::{checked_rem_euclid, EuclidDivisibleDigit};
use super::types::BigInt;

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> RemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> RemEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem_euclid(self, divisor: &Self) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
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
    RemEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> RemEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}
