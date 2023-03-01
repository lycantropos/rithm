use traiter::numbers::{CheckedDivRem, DivRem};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> DivRem for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivRem<Output = Option<(Self, Self)>>,
{
    type Output = (Self, Self);

    fn div_rem(self, divisor: Self) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivRem<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedDivRem<&'a Self, Output = Option<(Self, Self)>>,
{
    type Output = (Self, Self);

    fn div_rem(self, divisor: &Self) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivRem<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivRem<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<(
            BigInt<Digit, DIGIT_BITNESS>,
            BigInt<Digit, DIGIT_BITNESS>,
        )>,
    >,
{
    type Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>);

    fn div_rem(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivRem
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivRem<
        Output = Option<(
            BigInt<Digit, DIGIT_BITNESS>,
            BigInt<Digit, DIGIT_BITNESS>,
        )>,
    >,
{
    type Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>);

    fn div_rem(self, divisor: Self) -> Self::Output {
        self.checked_div_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
