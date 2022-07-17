use std::ops::{Div, Mul};

use traiter::numbers::{CheckedRemEuclid, Gcd, RemEuclid, Signed};

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > RemEuclid for Fraction<Component>
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>,
    > RemEuclid<Component> for Fraction<Component>
{
    type Output = Self;

    fn rem_euclid(self, divisor: Component) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    RemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRemEuclid<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! signed_integer_rem_fraction_impl {
    ($($integer:ty)*) => ($(
        impl RemEuclid<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedRemEuclid<Fraction<Self>>>::checked_rem_euclid(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

signed_integer_rem_fraction_impl!(i8 i16 i32 i64 i128 isize);
