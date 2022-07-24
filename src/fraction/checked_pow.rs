use std::ops::Neg;

use traiter::numbers::{CheckedPow, Signed, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeSign};

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedPow<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedPow<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + CheckedPow<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Signed,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_pow(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) =
                    (self.denominator.checked_pow(&exponent)?)
                        .normalize_sign(self.numerator.checked_pow(exponent)?);
                Some(Self {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Self {
                numerator: self.numerator.checked_pow(&exponent)?,
                denominator: self.denominator.checked_pow(exponent)?,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedPow<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedPow<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + CheckedPow<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Signed,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_pow(
        self,
        exponent: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) =
                    (self.denominator.checked_pow(&exponent)?)
                        .normalize_sign(self.numerator.checked_pow(exponent)?);
                Some(Self {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Self {
                numerator: self.numerator.checked_pow(exponent)?,
                denominator: self.denominator.checked_pow(exponent)?,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedPow<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedPow<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + CheckedPow<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Signed,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_pow(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) =
                    (self.denominator.checked_pow(&exponent)?)
                        .normalize_sign(self.numerator.checked_pow(exponent)?);
                Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: self.numerator.checked_pow(&exponent)?,
                denominator: self.denominator.checked_pow(exponent)?,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedPow<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedPow<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + CheckedPow<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Signed,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_pow(
        self,
        exponent: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) =
                    (self.denominator.checked_pow(&exponent)?)
                        .normalize_sign(self.numerator.checked_pow(exponent)?);
                Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: self.numerator.checked_pow(exponent)?,
                denominator: self.denominator.checked_pow(exponent)?,
            })
        }
    }
}

macro_rules! integer_fraction_checked_pow_impl {
    ($($integer:ty)*) => ($(
        impl CheckedPow<u32> for Fraction<$integer> {
            type Output = Option<Self>;

            fn checked_pow(self, exponent: u32) -> Self::Output {
                Some(Self {
                    numerator: self.numerator.checked_pow(exponent)?,
                    denominator: self.denominator.checked_pow(exponent)?,
                })
            }
        }
    )*)
}

integer_fraction_checked_pow_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
