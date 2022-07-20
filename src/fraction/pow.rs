use traiter::numbers::{CheckedPow, Pow};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Component> Pow<Component> for Fraction<Component>
where
    Self: CheckedPow<Component, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: Component) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! signed_integer_fraction_pow_impl {
    ($($integer:ty)*) => ($(
        impl Pow<u32> for Fraction<$integer> {
            type Output = Self;

            fn pow(self, exponent: u32) -> Self::Output {
                Self::Output {
                    numerator: self.numerator.pow(exponent),
                    denominator: self.denominator.pow(exponent),
                }
            }
        }
    )*)
}

signed_integer_fraction_pow_impl!(i8 i16 i32 i64 i128 isize);
