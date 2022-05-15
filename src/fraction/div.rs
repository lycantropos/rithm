use std::ops::Div;

use crate::big_int::{BigInt, GcdDigit, MultiplicativeDigit};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{CheckedDiv, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed};

use super::types::Fraction;

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid> Div
    for Fraction<Component>
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid>
    Div<Component> for Fraction<Component>
{
    type Output = Self;

    fn div(self, divisor: Component) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Div<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn div(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! primitive_div_fraction_impl {
    ($($t:ty)*) => ($(
        impl Div<Fraction<Self>> for $t {
            type Output = Fraction<Self>;

            fn div(self, divisor: Fraction<Self>) -> Self::Output {
                <$t as CheckedDiv<Fraction<Self>>>::checked_div(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

primitive_div_fraction_impl!(i8 i16 i32 i64 i128 isize);
