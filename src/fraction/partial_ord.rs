use std::cmp::Ordering;
use std::ops::Mul;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> PartialOrd
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: PartialOrd,
    Self: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (&self.numerator * &other.denominator)
            .partial_cmp(&(&self.denominator * &other.numerator))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: PartialOrd,
    Self: PartialEq<BigInt<Digit, SEPARATOR, SHIFT>>,
{
    fn partial_cmp(
        &self,
        other: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Option<Ordering> {
        self.numerator.partial_cmp(&(&self.denominator * other))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    PartialOrd<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a Self: Mul<Output = Self>,
    Self: PartialEq<Fraction<Self>> + PartialOrd,
{
    fn partial_cmp(&self, other: &Fraction<Self>) -> Option<Ordering> {
        (self * &other.denominator).partial_cmp(&other.numerator)
    }
}

macro_rules! integer_partial_ord_fraction_impl {
    ($($integer:ty)*) => ($(
        impl PartialOrd for Fraction<$integer> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                (self.numerator * other.denominator)
                    .partial_cmp(
                        &(self.denominator * other.numerator),
                    )
            }
        }

        impl PartialOrd<$integer> for Fraction<$integer> {
            fn partial_cmp(&self, other: &$integer) -> Option<Ordering> {
                self.numerator
                    .partial_cmp(&(self.denominator * other))
            }
        }

        impl PartialOrd<Fraction<Self>> for $integer
        {
            fn partial_cmp(&self, other: &Fraction<Self>) -> Option<Ordering> {
                (self * other.denominator).partial_cmp(&other.numerator)
            }
        }
    )*)
}

integer_partial_ord_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
