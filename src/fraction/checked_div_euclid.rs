use std::ops::Mul;

use traiter::numbers::{CheckedDivEuclid, Zeroable};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<
        Component: CheckedDivEuclid<Output = Option<Component>>
            + Mul<Output = Component>
            + Zeroable,
    > CheckedDivEuclid for Fraction<Component>
{
    type Output = Option<Component>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (self.numerator * divisor.denominator)
            .checked_div_euclid(divisor.numerator * self.denominator)
    }
}

impl<
        Component: CheckedDivEuclid<Output = Option<Component>> + Mul<Output = Component>,
    > CheckedDivEuclid<Component> for Fraction<Component>
{
    type Output = Option<Component>;

    fn checked_div_euclid(self, divisor: Component) -> Self::Output {
        self.numerator
            .checked_div_euclid(divisor * self.denominator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivEuclid<Output = Option<Self>> + Mul<Output = Self>,
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

macro_rules! integer_checked_div_euclid_fraction_impl {
    ($($integer:ty)*) => ($(
    impl CheckedDivEuclid<Fraction<Self>> for $integer
    {
        type Output = Option<Self>;

        fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
            (self * divisor.denominator).checked_div_euclid(divisor.numerator)
        }
    }
    )*)
}

integer_checked_div_euclid_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
