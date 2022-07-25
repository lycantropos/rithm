use std::ops::{Mul, MulAssign};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> MulAssign
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> MulAssign<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    MulAssign<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    fn mul_assign(&mut self, other: BigInt<Digit, SEPARATOR, SHIFT>) {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        self.numerator = &self.numerator * other;
        self.denominator = denominator;
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    MulAssign<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    fn mul_assign(&mut self, other: &BigInt<Digit, SEPARATOR, SHIFT>) {
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
