use crate::big_int::BigInt;
use traiter::numbers::Zeroable;

use super::types::Fraction;

impl<'a, Digit, const DIGIT_BITNESS: usize> Zeroable
    for &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    &'a BigInt<Digit, DIGIT_BITNESS>: Zeroable,
{
    fn is_zero(self) -> bool {
        (&self.numerator).is_zero()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Zeroable
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Zeroable,
{
    fn is_zero(self) -> bool {
        self.numerator.is_zero()
    }
}

macro_rules! integer_fraction_zeroable_impl {
    ($($integer:ty)*) => ($(
        impl Zeroable for &Fraction<$integer> {
            fn is_zero(self) -> bool {
                self.numerator.is_zero()
            }
        }

        impl Zeroable for Fraction<$integer> {
            fn is_zero(self) -> bool {
                self.numerator.is_zero()
            }
        }
    )*)
}

integer_fraction_zeroable_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
