use std::ops::{Mul, Sub, SubAssign};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> SubAssign
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    fn sub_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &other.denominator
            - &self.denominator * other.numerator)
            .normalize_moduli(&self.denominator * other.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> SubAssign<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    fn sub_assign(&mut self, other: &Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &other.denominator
            - &self.denominator * &other.numerator)
            .normalize_moduli(&self.denominator * &other.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    SubAssign<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Sub<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    fn sub_assign(&mut self, other: BigInt<Digit, SEPARATOR, SHIFT>) {
        (self.numerator, self.denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    SubAssign<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Sub<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    fn sub_assign(&mut self, other: &BigInt<Digit, SEPARATOR, SHIFT>) {
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
