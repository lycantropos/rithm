use std::ops::{Mul, Sub, SubAssign};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> SubAssign
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    fn sub_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &other.denominator
            - &self.denominator * other.numerator)
            .normalize_moduli(&self.denominator * other.denominator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize> SubAssign<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    fn sub_assign(&mut self, other: &Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &other.denominator
            - &self.denominator * &other.numerator)
            .normalize_moduli(&self.denominator * &other.denominator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize> SubAssign<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + Sub<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    fn sub_assign(&mut self, other: BigInt<Digit, DIGIT_BITNESS>) {
        (self.numerator, self.denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    SubAssign<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Sub<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    fn sub_assign(&mut self, other: &BigInt<Digit, DIGIT_BITNESS>) {
        (self.numerator, self.denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
    }
}

macro_rules! integer_fraction_sub_assign_impl {
    ($($integer:ty)*) => ($(
        impl SubAssign for Fraction<$integer> {
            fn sub_assign(&mut self, other: Self) {
                (self.numerator, self.denominator) = (self.numerator
                    * other.denominator
                    - self.denominator * other.numerator)
                    .normalize_moduli(self.denominator * other.denominator);
            }
        }

        impl SubAssign<$integer> for Fraction<$integer> {
            fn sub_assign(&mut self, other: $integer) {
                (self.numerator, self.denominator) = (self.numerator
                    - self.denominator * other)
                    .normalize_moduli(self.denominator);
            }
        }
    )*)
}

integer_fraction_sub_assign_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
