use std::ops::{DivAssign, Mul};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli, NormalizeSign};

impl<Digit, const DIGIT_BITNESS: usize> DivAssign
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    fn div_assign(&mut self, divisor: Self) {
        let (numerator, divisor_numerator) =
            self.numerator.normalize_moduli(divisor.numerator);
        let (denominator, divisor_denominator) =
            self.denominator.normalize_moduli(divisor.denominator);
        (self.numerator, self.denominator) = (numerator * divisor_denominator)
            .normalize_sign(denominator * divisor_numerator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivAssign<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    fn div_assign(&mut self, divisor: &Self) {
        let (numerator, divisor_numerator) =
            self.numerator.normalize_moduli(&divisor.numerator);
        let (denominator, divisor_denominator) =
            self.denominator.normalize_moduli(&divisor.denominator);
        (self.numerator, self.denominator) = (numerator * divisor_denominator)
            .normalize_sign(denominator * divisor_numerator);
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivAssign<BigInt<Digit, DIGIT_BITNESS>>
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
    BigInt<Digit, DIGIT_BITNESS>: NormalizeSign<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
{
    fn div_assign(&mut self, divisor: BigInt<Digit, DIGIT_BITNESS>) {
        let (numerator, divisor) = self.numerator.normalize_moduli(divisor);
        (self.numerator, self.denominator) =
            numerator.normalize_sign(&self.denominator * divisor);
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    DivAssign<&BigInt<Digit, DIGIT_BITNESS>>
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
    BigInt<Digit, DIGIT_BITNESS>: NormalizeSign<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
{
    fn div_assign(&mut self, divisor: &BigInt<Digit, DIGIT_BITNESS>) {
        let (numerator, divisor) = self.numerator.normalize_moduli(divisor);
        (self.numerator, self.denominator) =
            numerator.normalize_sign(&self.denominator * divisor);
    }
}

macro_rules! integer_fraction_div_assign_impl {
    ($($integer:ty)*) => ($(
        impl DivAssign for Fraction<$integer> {
            fn div_assign(&mut self, divisor: Self) {
                let (numerator, divisor_numerator) =
                    self.numerator.normalize_moduli(divisor.numerator);
                let (denominator, divisor_denominator) =
                    self.denominator.normalize_moduli(divisor.denominator);
                (self.numerator, self.denominator) = (numerator
                    * divisor_denominator)
                    .normalize_sign(denominator * divisor_numerator);
            }
        }

        impl DivAssign<$integer> for Fraction<$integer> {
            fn div_assign(&mut self, divisor: $integer) {
                let (numerator, divisor) =
                    self.numerator.normalize_moduli(divisor);
                (self.numerator, self.denominator) =
                    numerator.normalize_sign(self.denominator * divisor);
            }
        }
    )*)
}

integer_fraction_div_assign_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
