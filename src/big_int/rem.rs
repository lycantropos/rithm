use std::ops::Rem;

use traiter::numbers::CheckedRem;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> Rem for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRem<Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedRem<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: &Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRem<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn rem(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
