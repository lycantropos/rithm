use std::ops::{Add, Div, Mul};

use traiter::numbers::{Gcd, Signed};

use crate::big_int::BigInt;

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Add<Output = Component>
            + Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > Add for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
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
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>,
    > Add<Component> for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
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

macro_rules! signed_integer_add_fraction_impl {
    ($($integer:ty)*) => ($(
    impl Add<Fraction<Self>> for $integer {
        type Output = Fraction<Self>;

        fn add(self, other: Fraction<Self>) -> Self::Output {
            other + self
        }
    }
    )*)
}

signed_integer_add_fraction_impl!(i8 i16 i32 i64 i128 isize);
