use std::ops::Rem;

use crate::big_int::{BigInt, GcdDigit, MultiplicativeDigit};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{CheckedRem, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed};

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + DivisivePartialMagma
            + GcdMagma
            + Signed
            + MultiplicativeMonoid,
    > Rem for Fraction<Component>
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + DivisivePartialMagma
            + GcdMagma
            + Signed
            + MultiplicativeMonoid,
    > Rem<Component> for Fraction<Component>
{
    type Output = Self;

    fn rem(self, divisor: Component) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Rem<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn rem(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! primitive_rem_fraction_impl {
    ($($t:ty)*) => ($(
        impl Rem<Fraction<Self>> for $t {
            type Output = Fraction<Self>;

            fn rem(self, divisor: Fraction<Self>) -> Self::Output {
                <$t as CheckedRem<Fraction<Self>>>::checked_rem(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

primitive_rem_fraction_impl!(i8 i16 i32 i64 i128 isize);
