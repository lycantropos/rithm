use std::ops::{Div, Mul};

use traiter::numbers::{CheckedDivEuclid, DivEuclid, Gcd, Signed};

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > DivEuclid for Fraction<Component>
{
    type Output = Component;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > DivEuclid<Component> for Fraction<Component>
{
    type Output = Component;

    fn div_euclid(self, divisor: Component) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    DivEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivEuclid<Fraction<Self>, Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! signed_integer_div_euclid_fraction_impl {
    ($($integer:ty)*) => ($(
        impl DivEuclid<Fraction<Self>> for $integer {
            type Output = Self;

            fn div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedDivEuclid<Fraction<Self>>>::checked_div_euclid(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

signed_integer_div_euclid_fraction_impl!(i8 i16 i32 i64 i128 isize);
