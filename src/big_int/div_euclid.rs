use traiter::numbers::{CheckedDivEuclid, DivEuclid};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivEuclid<Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedDivEuclid<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    DivEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivEuclid<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
