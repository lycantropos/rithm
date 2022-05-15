use crate::big_int::{BigInt, GcdDigit, MultiplicativeDigit};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use crate::traits::{
    CheckedRemEuclid, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, RemEuclid, Signed,
};

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + DivisivePartialMagma
            + GcdMagma
            + Signed
            + MultiplicativeMonoid,
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
            + DivisivePartialMagma
            + GcdMagma
            + Signed
            + MultiplicativeMonoid,
    > RemEuclid<Component> for Fraction<Component>
{
    type Output = Self;

    fn rem_euclid(self, divisor: Component) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit: GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    RemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! primitive_rem_fraction_impl {
    ($($t:ty)*) => ($(
        impl RemEuclid<Fraction<Self>> for $t {
            type Output = Fraction<Self>;

            fn rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                <$t as CheckedRemEuclid<Fraction<Self>>>::checked_rem_euclid(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

primitive_rem_fraction_impl!(i8 i16 i32 i64 i128 isize);
