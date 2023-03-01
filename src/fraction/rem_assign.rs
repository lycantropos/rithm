use std::ops::{Mul, RemAssign};

use traiter::numbers::CheckedRem;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> RemAssign
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
    BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    fn rem_assign(&mut self, divisor: Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &divisor.denominator)
            .checked_rem(&self.denominator * divisor.numerator)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            .normalize_moduli(&self.denominator * divisor.denominator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemAssign<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    fn rem_assign(&mut self, divisor: &Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &divisor.denominator)
            .checked_rem(&self.denominator * &divisor.numerator)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            .normalize_moduli(&self.denominator * &divisor.denominator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemAssign<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedRem<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
{
    fn rem_assign(&mut self, divisor: BigInt<Digit, DIGIT_BITNESS>) {
        (self.numerator, self.denominator) = self
            .numerator
            .checked_rem(&self.denominator * divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            .normalize_moduli(&self.denominator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    RemAssign<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedRem<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
{
    fn rem_assign(&mut self, divisor: &BigInt<Digit, DIGIT_BITNESS>) {
        (self.numerator, self.denominator) = self
            .numerator
            .checked_rem(&self.denominator * divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            .normalize_moduli(&self.denominator);
    }
}

macro_rules! integer_fraction_rem_assign_impl {
    ($($integer:ty)*) => ($(
        impl RemAssign for Fraction<$integer> {
            fn rem_assign(&mut self, divisor: Self) {
                (self.numerator, self.denominator) = (self.numerator
                    * divisor.denominator)
                    .checked_rem(self.denominator * divisor.numerator)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
                    .normalize_moduli(self.denominator * divisor.denominator);
            }
        }

        impl RemAssign<$integer> for Fraction<$integer> {
            fn rem_assign(&mut self, divisor: $integer) {
                (self.numerator, self.denominator) = self
                    .numerator
                    .checked_rem(self.denominator * divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
                    .normalize_moduli(self.denominator);
            }
        }
    )*)
}

integer_fraction_rem_assign_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
