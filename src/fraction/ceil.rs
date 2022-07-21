use std::ops::Neg;

use traiter::numbers::{Ceil, CheckedDivEuclid, CheckedDivRemEuclid};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Ceil
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn ceil(self) -> Self::Output {
        -unsafe {
            (-self.numerator)
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Ceil
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

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
