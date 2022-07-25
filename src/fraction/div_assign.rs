use std::ops::{DivAssign, Mul};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli, NormalizeSign};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivAssign
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivAssign<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    DivAssign<BigInt<Digit, SEPARATOR, SHIFT>>
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
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeSign<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
{
    fn div_assign(&mut self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) {
        let (numerator, divisor) = self.numerator.normalize_moduli(divisor);
        (self.numerator, self.denominator) =
            numerator.normalize_sign(&self.denominator * divisor);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    DivAssign<&BigInt<Digit, SEPARATOR, SHIFT>>
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
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeSign<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
{
    fn div_assign(&mut self, divisor: &BigInt<Digit, SEPARATOR, SHIFT>) {
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
