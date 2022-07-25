use std::ops::{Mul, RemAssign};

use traiter::numbers::CheckedRem;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> RemAssign
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> RemAssign<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    RemAssign<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
{
    fn rem_assign(&mut self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) {
        (self.numerator, self.denominator) = self
            .numerator
            .checked_rem(&self.denominator * divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            .normalize_moduli(&self.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    RemAssign<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
{
    fn rem_assign(&mut self, divisor: &BigInt<Digit, SEPARATOR, SHIFT>) {
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
