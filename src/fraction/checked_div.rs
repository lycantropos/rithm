use std::ops::{Div, Mul, Neg};

use traiter::numbers::{CheckedDiv, Gcd, Signed, Unitary, Zeroable};

use crate::big_int::BigInt;

use super::types::{
    normalize_components_moduli, normalize_components_sign, Fraction,
};

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>
            + Neg<Output = Component>
            + Unitary,
    > CheckedDiv for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend_numerator, divisor_numerator) =
            normalize_components_moduli(self.numerator, divisor.numerator);
        let (dividend_denominator, divisor_denominator) =
            normalize_components_moduli(self.denominator, divisor.denominator);
        let (numerator, denominator) = normalize_components_sign(
            dividend_numerator * divisor_denominator,
            dividend_denominator * divisor_numerator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>
            + Neg<Output = Component>
            + Unitary,
    > CheckedDiv<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Component) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend_numerator, divisor_numerator) =
            normalize_components_moduli(self.numerator, divisor);
        let (numerator, denominator) = normalize_components_sign(
            dividend_numerator,
            self.denominator * divisor_numerator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Clone
        + Div<Output = Self>
        + Gcd<Output = Self>
        + Mul<Output = Self>
        + Neg<Output = Self>
        + Signed
        + Unitary,
{
    type Output = Option<Fraction<Self>>;

    fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend, divisor_numerator) =
            normalize_components_moduli(self, divisor.numerator);
        let (numerator, denominator) = normalize_components_sign(
            dividend * divisor.denominator,
            divisor_numerator,
        );
        Some(Fraction::<Self> {
            numerator,
            denominator,
        })
    }
}

macro_rules! signed_integer_checked_div_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDiv<Fraction<Self>> for $integer {
            type Output = Option<Fraction<Self>>;

            fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
                if divisor.is_zero() {
                    return None;
                }
                let (dividend, divisor_numerator) =
                    normalize_components_moduli(self, divisor.numerator);
                let (numerator, denominator) = normalize_components_sign(
                    dividend * divisor.denominator,
                    divisor_numerator,
                );
                Some(Fraction::<Self> {
                    numerator,
                    denominator,
                })
            }
        }
    )*)
}

signed_integer_checked_div_fraction_impl!(i8 i16 i32 i64 i128 isize);
