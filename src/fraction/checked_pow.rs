use std::ops::Neg;

use traiter::numbers::{CheckedPow, Signed, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeSign};

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPow<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedPow<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + CheckedPow<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Neg<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Signed,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_pow(
        self,
        exponent: BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPow<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Neg<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedPow<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + CheckedPow<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Signed,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_pow(
        self,
        exponent: &BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPow<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedPow<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + CheckedPow<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Neg<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Signed,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_pow(
        self,
        exponent: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) =
                    (self.denominator.checked_pow(&exponent)?)
                        .normalize_sign(self.numerator.checked_pow(exponent)?);
                Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator: self.numerator.checked_pow(&exponent)?,
                denominator: self.denominator.checked_pow(exponent)?,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPow<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedPow<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + CheckedPow<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Neg<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Signed,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_pow(
        self,
        exponent: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) =
                    (self.denominator.checked_pow(&exponent)?)
                        .normalize_sign(self.numerator.checked_pow(exponent)?);
                Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
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
