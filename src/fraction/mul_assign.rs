use std::ops::{Mul, MulAssign};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> MulAssign
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    fn mul_assign(&mut self, other: Self) {
        let (numerator, other_denominator) =
            self.numerator.normalize_moduli(other.denominator);
        let (denominator, other_numerator) =
            self.denominator.normalize_moduli(other.numerator);
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
    }
}

impl<Digit, const DIGIT_BITNESS: usize> MulAssign<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    fn mul_assign(&mut self, other: &Self) {
        let (numerator, other_denominator) =
            self.numerator.normalize_moduli(&other.denominator);
        let (denominator, other_numerator) =
            self.denominator.normalize_moduli(&other.numerator);
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
    }
}

impl<Digit, const DIGIT_BITNESS: usize> MulAssign<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    fn mul_assign(&mut self, other: BigInt<Digit, DIGIT_BITNESS>) {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        self.numerator = &self.numerator * other;
        self.denominator = denominator;
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    MulAssign<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    fn mul_assign(&mut self, other: &BigInt<Digit, DIGIT_BITNESS>) {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        self.numerator = &self.numerator * other;
        self.denominator = denominator;
    }
}

macro_rules! integer_mul_assign_fraction_impl {
    ($($integer:ty)*) => ($(
        impl MulAssign for Fraction<$integer> {
            fn mul_assign(&mut self, other: Self) {
                let (numerator, other_denominator) =
                    self.numerator.normalize_moduli(other.denominator);
                let (other_numerator, denominator) =
                    other.numerator.normalize_moduli(self.denominator);
                self.numerator = numerator * other_numerator;
                self.denominator = denominator * other_denominator;
            }
        }

        impl MulAssign<$integer> for Fraction<$integer>
        {
            fn mul_assign(&mut self, other: $integer) {
                let (other, denominator) =
                    other.normalize_moduli(self.denominator);
                self.numerator = self.numerator * other;
                self.denominator = denominator;
            }
        }
    )*)
}

integer_mul_assign_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
