use std::ops::Neg;

use traiter::numbers::{Ceil, CheckedDivEuclid, CheckedDivRemEuclid};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Ceil
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Neg<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn ceil(self) -> Self::Output {
        -unsafe {
            (-self.numerator)
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Ceil
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Neg<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn ceil(self) -> Self::Output {
        -unsafe {
            (-&self.numerator)
                .checked_div_euclid(&self.denominator)
                .unwrap_unchecked()
        }
    }
}

macro_rules! signed_integer_fraction_ceil_impl {
    ($($integer:ty)*) => ($(
        impl Ceil for Fraction<$integer> {
            type Output = $integer;

            fn ceil(self) -> Self::Output {
                -unsafe {
                    (-self.numerator)
                        .checked_div_euclid(self.denominator)
                        .unwrap_unchecked()
                }
            }
        }
    )*)
}

signed_integer_fraction_ceil_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_fraction_ceil_impl {
    ($($integer:ty)*) => ($(
        impl Ceil for Fraction<$integer> {
            type Output = $integer;

            fn ceil(self) -> Self::Output {
                let (quotient, remainder) = unsafe {
                    self.numerator
                        .checked_div_rem_euclid(self.denominator)
                        .unwrap_unchecked()
                };
                if remainder == 0 {
                    quotient
                } else {
                    quotient + 1
                }
            }
        }
    )*)
}

unsigned_integer_fraction_ceil_impl!(u8 u16 u32 u64 u128 usize);
