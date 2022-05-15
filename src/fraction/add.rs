use std::ops::Add;

use crate::big_int::{AdditiveDigit, BigInt, GcdDigit, MultiplicativeDigit};
use crate::traits::{AdditiveMonoid, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: AdditiveMonoid + Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid,
    > Add for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator * other.denominator.clone() + other.numerator * self.denominator.clone(),
            self.denominator * other.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: AdditiveMonoid + Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid,
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

impl<
        Digit: AdditiveDigit + GcdDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > Add<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn add(self, other: Fraction<Self>) -> Self::Output {
        other + self
    }
}

macro_rules! primitive_add_fraction_impl {
    ($($t:ty)*) => ($(
    impl Add<Fraction<Self>> for $t {
        type Output = Fraction<Self>;

        fn add(self, other: Fraction<Self>) -> Self::Output {
            other + self
        }
    }
    )*)
}

primitive_add_fraction_impl!(i8 i16 i32 i64 i128 isize);
