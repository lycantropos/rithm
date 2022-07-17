use traiter::numbers::{CheckedRemEuclid, RemEuclid};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> RemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRemEuclid<Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> RemEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedRemEuclid<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    RemEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRemEuclid<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> RemEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedRemEuclid<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
