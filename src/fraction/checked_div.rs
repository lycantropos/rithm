use std::ops::Mul;

use traiter::numbers::{CheckedDiv, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli, NormalizeSign};

impl<
        Component: Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + NormalizeSign<Output = (Component, Component)>,
    > CheckedDiv for Fraction<Component>
where
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend_numerator, divisor_numerator) =
            Component::normalize_moduli(self.numerator, divisor.numerator);
        let (dividend_denominator, divisor_denominator) =
            Component::normalize_moduli(self.denominator, divisor.denominator);
        let (numerator, denominator) = Component::normalize_sign(
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
        Component: Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + NormalizeSign<Output = (Component, Component)>
            + Zeroable,
    > CheckedDiv<Component> for Fraction<Component>
where
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Component) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend_numerator, divisor_numerator) =
            Component::normalize_moduli(self.numerator, divisor);
        let (numerator, denominator) = Component::normalize_sign(
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
        + Mul<Output = Self>
        + NormalizeModuli<Output = (Self, Self)>
        + NormalizeSign<Output = (Self, Self)>,
    Fraction<Self>: Zeroable,
{
    type Output = Option<Fraction<Self>>;

    fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend, divisor_numerator) =
            Self::normalize_moduli(self, divisor.numerator);
        let (numerator, denominator) = Self::normalize_sign(
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
                    <$integer>::normalize_moduli(self, divisor.numerator);
                let (numerator, denominator) = <$integer>::normalize_sign(
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
