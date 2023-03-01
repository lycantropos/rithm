use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Eq
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: PartialEq,
{
}

macro_rules! integer_fraction_eq_impl {
    ($($integer:ty)*) => ($(
        impl Eq for Fraction<$integer> where Self: PartialEq {}
    )*)
}

integer_fraction_eq_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
