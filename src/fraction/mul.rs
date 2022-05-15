use std::ops::Mul;

use crate::big_int::{BigInt, GcdDigit, MultiplicativeDigit};
use crate::traits::{DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed};

use super::types::{normalize_components_moduli, Fraction};

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid> Mul
    for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (numerator, other_denominator) =
            normalize_components_moduli(self.numerator, other.denominator);
        let (other_numerator, denominator) =
            normalize_components_moduli(other.numerator, self.denominator);
        Self::Output {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid>
    Mul<Component> for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Component) -> Self::Output {
        let (other, denominator) = normalize_components_moduli(other, self.denominator);
        Self::Output {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

impl<Digit: GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Mul<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn mul(self, other: Fraction<Self>) -> Self::Output {
        other * self
    }
}

macro_rules! primitive_mul_fraction_impl {
    ($($t:ty)*) => ($(
    impl Mul<Fraction<Self>> for $t {
        type Output = Fraction<Self>;

        fn mul(self, other: Fraction<Self>) -> Self::Output {
            other * self
        }
    }
    )*)
}

primitive_mul_fraction_impl!(i8 i16 i32 i64 i128 isize);
