use std::ops::Mul;

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > Mul for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (numerator, other_denominator) =
            Component::normalize_moduli(self.numerator, other.denominator);
        let (other_numerator, denominator) =
            Component::normalize_moduli(other.numerator, self.denominator);
        Self::Output {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > Mul<Component> for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Component) -> Self::Output {
        let (other, denominator) =
            Component::normalize_moduli(other, self.denominator);
        Self::Output {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Fraction<Self>: Mul<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    fn mul(self, other: Fraction<Self>) -> Self::Output {
        other * self
    }
}

macro_rules! signed_integer_mul_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Mul<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn mul(self, other: Fraction<Self>) -> Self::Output {
                other * self
            }
        }
    )*)
}

signed_integer_mul_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
