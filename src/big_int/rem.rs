use std::ops::Rem;

use traiter::numbers::CheckedRem;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRem<Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedRem<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: &Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRem<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
