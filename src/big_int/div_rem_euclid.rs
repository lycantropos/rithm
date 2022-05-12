use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{CheckedDivRemEuclid, DivRemEuclid};

use super::digits::EuclidDivisibleDigit;
use super::types::BigInt;

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRemEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    DivRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    );

    fn div_rem_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRemEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    );

    fn div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
