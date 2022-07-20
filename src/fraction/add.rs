use std::ops::{Add, Mul};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Add<Output = Component>
            + Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > Add for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (numerator, denominator) = Component::normalize_moduli(
            self.numerator * other.denominator.clone()
                + other.numerator * self.denominator.clone(),
            self.denominator * other.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: Add<Output = Component>
            + Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > Add<Component> for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Component) -> Self::Output {
        let (numerator, denominator) = Component::normalize_moduli(
            self.numerator + other * self.denominator.clone(),
            self.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Fraction<Self>: Add<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    fn add(self, other: Fraction<Self>) -> Self::Output {
        other + self
    }
}

macro_rules! integer_add_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Add<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn add(self, other: Fraction<Self>) -> Self::Output {
                other + self
            }
        }
    )*)
}

integer_add_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
