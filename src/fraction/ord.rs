use std::cmp::Ordering;
use std::ops::Mul;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Ord
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    BigInt<Digit, DIGIT_BITNESS>: Ord,
    Self: Eq + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.numerator * &other.denominator)
            .cmp(&(&self.denominator * &other.numerator))
    }
}

macro_rules! integer_fraction_ord_impl {
    ($($integer:ty)*) => ($(
        impl Ord for Fraction<$integer> {
            fn cmp(&self, other: &Self) -> Ordering {
                (self.numerator * other.denominator)
                    .cmp(&(self.denominator * other.numerator))
            }
        }
    )*)
}

integer_fraction_ord_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
