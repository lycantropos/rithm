use std::ops::Div;

use traiter::numbers::CheckedDiv;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> Div for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDiv<Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedDiv<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: &Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDiv<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDiv<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
