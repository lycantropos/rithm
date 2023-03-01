use traiter::numbers::{CheckedDivEuclid, DivEuclid};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivEuclid<Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedDivEuclid<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
