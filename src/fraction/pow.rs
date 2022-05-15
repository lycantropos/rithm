use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{CheckedPow, Pow, Signed, Unitary, Zeroable};

use super::types::Fraction;

impl<
        Component: Clone + Signed + CheckedPow<Component, Output = Option<Component>> + Unitary + Zeroable,
    > Pow<Component> for Fraction<Component>
{
    type Output = Self;

    fn pow(self, exponent: Component) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! primitive_fraction_pow_impl {
    ($($t:ty)*) => ($(
        impl Pow<u32> for Fraction<$t> {
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

primitive_fraction_pow_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
