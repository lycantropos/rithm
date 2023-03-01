use traiter::numbers::{Ceil, Floor, Signed, Trunc};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Trunc
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: Ceil<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Floor<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Signed,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn trunc(self) -> Self::Output {
        if self.is_negative() {
            self.ceil()
        } else {
            self.floor()
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Trunc
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Signed,
    Self: Ceil<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Floor<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn trunc(self) -> Self::Output {
        if self.is_negative() {
            self.ceil()
        } else {
            self.floor()
        }
    }
}

macro_rules! signed_integer_fraction_trunc_impl {
    ($($integer:ty)*) => ($(
        impl Trunc for Fraction<$integer> {
            type Output = $integer;

            fn trunc(self) -> Self::Output {
                if self.is_negative() {
                    self.ceil()
                } else {
                    self.floor()
                }
            }
        }
    )*)
}

signed_integer_fraction_trunc_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_fraction_trunc_impl {
    ($($integer:ty)*) => ($(
        impl Trunc for Fraction<$integer> {
            type Output = $integer;

            #[inline]
            fn trunc(self) -> Self::Output {
                self.floor()
            }
        }
    )*)
}

unsigned_integer_fraction_trunc_impl!(u8 u16 u32 u64 u128 usize);
