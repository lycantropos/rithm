use std::ops::{Mul, Sub};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + Sub<Output = Component>,
    > Sub for Fraction<Component>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (numerator, denominator) = Component::normalize_moduli(
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
            + Mul<Output = Component>
            + Sub<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > Sub<Component> for Fraction<Component>
{
    type Output = Self;

    fn sub(self, subtrahend: Component) -> Self::Output {
        let (numerator, denominator) = Component::normalize_moduli(
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
        + Mul<Output = Self>
        + Sub<Output = Self>
        + NormalizeModuli<Output = (Self, Self)>,
{
    type Output = Fraction<Self>;

    fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
        let (numerator, denominator) = Self::normalize_moduli(
            self * subtrahend.denominator.clone() - subtrahend.numerator,
            subtrahend.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

macro_rules! integer_sub_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Sub<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
                let (numerator, denominator) = <$integer>::normalize_moduli(
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

integer_sub_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
