use std::ops::{Div, Mul, Sub};

use traiter::numbers::{Gcd, Signed};

use crate::big_int::BigInt;

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed
            + Sub<Output = Component>,
    > Sub for Fraction<Component>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator * subtrahend.denominator.clone()
                - self.denominator.clone() * subtrahend.numerator,
            self.denominator * subtrahend.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: Clone
            + Div<Output = Component>
            + Eq
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>
            + Sub<Output = Component>,
    > Sub<Component> for Fraction<Component>
{
    type Output = Self;

    fn sub(self, subtrahend: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator - self.denominator.clone() * subtrahend,
            self.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Clone
        + Div<Output = Self>
        + Gcd<Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>,
{
    type Output = Fraction<Self>;

    fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self * subtrahend.denominator.clone() - subtrahend.numerator,
            subtrahend.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

macro_rules! signed_integer_sub_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Sub<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
                let (numerator, denominator) = normalize_components_moduli(
                    self * subtrahend.denominator - subtrahend.numerator,
                    subtrahend.denominator,
                );
                Self::Output {
                    numerator,
                    denominator,
                }
            }
        }
    )*)
}

signed_integer_sub_fraction_impl!(i8 i16 i32 i64 i128 isize);
