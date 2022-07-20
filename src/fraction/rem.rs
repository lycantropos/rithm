use std::ops::Rem;

use traiter::numbers::CheckedRem;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Component> Rem for Fraction<Component>
where
    Self: CheckedRem<Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Component> Rem<Component> for Fraction<Component>
where
    Self: CheckedRem<Component, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: Component) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRem<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! signed_integer_rem_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Rem<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn rem(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedRem<Fraction<Self>>>::checked_rem(
                    self, divisor,
                )
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

signed_integer_rem_fraction_impl!(i8 i16 i32 i64 i128 isize);
