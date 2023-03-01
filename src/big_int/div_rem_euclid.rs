use traiter::numbers::{CheckedDivRemEuclid, DivRemEuclid};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::digits::CheckedDivRemEuclidComponents;
use super::types::BigInt;

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    DivRemEuclid for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = (Self, Self);

    fn div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    DivRemEuclid<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = (Self, Self);

    fn div_rem_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    DivRemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>);

    fn div_rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    DivRemEuclid for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>);

    fn div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
