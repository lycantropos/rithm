use traiter::numbers::{CheckedDivEuclid, Floor};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Floor
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn floor(self) -> Self::Output {
        unsafe {
            self.numerator
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Floor
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn floor(self) -> Self::Output {
        unsafe {
            (&self.numerator)
                .checked_div_euclid(&self.denominator)
                .unwrap_unchecked()
        }
    }
}

macro_rules! integer_fraction_floor_impl {
    ($($integer:ty)*) => ($(
        impl Floor for Fraction<$integer> {
            type Output = $integer;

            fn floor(self) -> Self::Output {
                unsafe {
                    self.numerator
                        .checked_div_euclid(self.denominator)
                        .unwrap_unchecked()
                }
            }
        }
    )*)
}

integer_fraction_floor_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
