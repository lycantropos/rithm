use crate::big_int::{BigInt, GcdDigit, MultiplicativeDigit};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{
    CheckedDivEuclid, DivEuclid, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed,
};

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + DivisivePartialMagma
            + GcdMagma
            + Signed
            + MultiplicativeMonoid,
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
            + DivisivePartialMagma
            + GcdMagma
            + Signed
            + MultiplicativeMonoid,
    > DivEuclid<Component> for Fraction<Component>
{
    type Output = Component;

    fn div_euclid(self, divisor: Component) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    DivEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! primitive_div_fraction_impl {
    ($($t:ty)*) => ($(
        impl DivEuclid<Fraction<Self>> for $t {
            type Output = Self;

            fn div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                <$t as CheckedDivEuclid<Fraction<Self>>>::checked_div_euclid(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

primitive_div_fraction_impl!(i8 i16 i32 i64 i128 isize);
