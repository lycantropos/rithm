use std::ops::Div;

use traiter::numbers::CheckedDiv;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Component> Div for Fraction<Component>
where
    Self: CheckedDiv<Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Component> Div<Component> for Fraction<Component>
where
    Self: CheckedDiv<Component, Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: Component) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Div<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDiv<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn div(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! integer_div_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Div<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn div(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedDiv<Fraction<Self>>>::checked_div(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

integer_div_fraction_impl!(i8 i16 i32 i64 i128 isize);
