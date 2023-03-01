use std::hash::{Hash, Hasher};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Hash
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator.hash(state);
        self.denominator.hash(state);
    }
}

macro_rules! integer_fraction_hash_impl {
    ($($integer:ty)*) => ($(
        impl Hash for Fraction<$integer> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.numerator.hash(state);
                self.denominator.hash(state);
            }
        }
    )*)
}

integer_fraction_hash_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
