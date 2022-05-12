use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{CheckedDivRem, DivRem};

use super::digits::DivisibleDigit;
use super::types::BigInt;

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem(self, divisor: Self) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRem<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem(self, divisor: &Self) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    DivRem<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    );

    fn div_rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRem
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    );

    fn div_rem(self, divisor: Self) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
