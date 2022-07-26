use std::ops::{Add, AddAssign, Mul};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> AddAssign
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    fn add_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &other.denominator
            + &self.denominator * other.numerator)
            .normalize_moduli(&self.denominator * other.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> AddAssign<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    fn add_assign(&mut self, other: &Self) {
        (self.numerator, self.denominator) = (&self.numerator
            * &other.denominator
            + &self.denominator * &other.numerator)
            .normalize_moduli(&self.denominator * &other.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    AddAssign<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Add<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    fn add_assign(&mut self, other: BigInt<Digit, SEPARATOR, SHIFT>) {
        (self.numerator, self.denominator) = (&self.numerator
            + &self.denominator * other)
            .normalize_moduli(&self.denominator);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    AddAssign<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Add<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    fn add_assign(&mut self, other: &BigInt<Digit, SEPARATOR, SHIFT>) {
        (self.numerator, self.denominator) = (&self.numerator
            + &self.denominator * other)
            .normalize_moduli(&self.denominator);
    }
}

macro_rules! integer_fraction_add_assign_impl {
    ($($integer:ty)*) => ($(
        impl AddAssign for Fraction<$integer> {
            fn add_assign(&mut self, other: Self) {
                (self.numerator, self.denominator) = (self.numerator
                    * other.denominator
                    + self.denominator * other.numerator)
                    .normalize_moduli(self.denominator * other.denominator);
            }
        }

        impl AddAssign<$integer> for Fraction<$integer> {
            fn add_assign(&mut self, other: $integer) {
                (self.numerator, self.denominator) = (self.numerator
                    + self.denominator * other)
                    .normalize_moduli(self.denominator);
            }
        }
    )*)
}

integer_fraction_add_assign_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
