use traiter::numbers::{CheckedRemEuclid, RemEuclid};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRemEuclid<Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedRemEuclid<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRemEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedRemEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
