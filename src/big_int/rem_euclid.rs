use traiter::numbers::{CheckedRemEuclid, RemEuclid};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const DIGIT_BITNESS: usize> RemEuclid
    for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
where
    Self: CheckedRemEuclid<Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const DIGIT_BITNESS: usize> RemEuclid<&Self>
    for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
where
    for<'a> Self: CheckedRemEuclid<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const DIGIT_BITNESS: usize>
    RemEuclid<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
where
    Self: CheckedRemEuclid<
        BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn rem_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const DIGIT_BITNESS: usize> RemEuclid
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
where
    for<'a> Self: CheckedRemEuclid<
        &'a BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}
