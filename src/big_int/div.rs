use std::ops::Div;

use traiter::numbers::CheckedDiv;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Div
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDiv<Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Div<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedDiv<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: &Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Div<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDiv<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Div
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDiv<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
