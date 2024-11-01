use crate::big_int::BigInt;
use traiter::numbers::Unitary;

use super::types::Fraction;

impl<'a, Digit, const DIGIT_BITNESS: usize> Unitary
    for &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    &'a BigInt<Digit, DIGIT_BITNESS>: Unitary,
{
    fn is_one(self) -> bool {
        (&self.numerator).is_one()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Unitary
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Unitary,
{
    fn is_one(self) -> bool {
        self.numerator.is_one()
    }
}

macro_rules! integer_fraction_unitary_impl {
    ($($integer:ty)*) => ($(
        impl Unitary for &Fraction<$integer> {
            fn is_one(self) -> bool {
                self.numerator.is_one()
            }
        }

        impl Unitary for Fraction<$integer> {
            fn is_one(self) -> bool {
                self.numerator.is_one()
            }
        }
    )*)
}

integer_fraction_unitary_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
