use crate::big_int::{BigInt, EuclidDivisibleDigit, MultiplicativeDigit};
use crate::traits::{CheckedDivEuclid, MultiplicativeMonoid, Zeroable};

use super::types::Fraction;

impl<
        Component: Clone + CheckedDivEuclid<Output = Option<Component>> + MultiplicativeMonoid + Zeroable,
    > CheckedDivEuclid for Fraction<Component>
{
    type Output = Option<Component>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (self.numerator * divisor.denominator)
            .checked_div_euclid(divisor.numerator * self.denominator)
    }
}

impl<Component: Clone + CheckedDivEuclid<Output = Option<Component>> + MultiplicativeMonoid>
    CheckedDivEuclid<Component> for Fraction<Component>
{
    type Output = Option<Component>;

    fn checked_div_euclid(self, divisor: Component) -> Self::Output {
        self.numerator
            .checked_div_euclid(divisor * self.denominator)
    }
}

impl<
        Digit: EuclidDivisibleDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

macro_rules! primitive_checked_div_euclid_fraction_impl {
    ($($t:ty)*) => ($(
    impl CheckedDivEuclid<Fraction<Self>> for $t
    {
        type Output = Option<Self>;

        fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
            (self * divisor.denominator).checked_div_euclid(divisor.numerator)
        }
    }
    )*)
}

primitive_checked_div_euclid_fraction_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
