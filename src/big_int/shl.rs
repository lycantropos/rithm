use std::ops::Shl;

use traiter::numbers::CheckedShl;

use super::types::{BigInt, ShlError};

impl<Digit, const DIGIT_BITNESS: usize> Shl for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedShl<Output = Result<Self, ShlError>>,
{
    type Output = Self;

    fn shl(self, shift: Self) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Shl<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedShl<&'a Self, Output = Result<Self, ShlError>>,
{
    type Output = Self;

    fn shl(self, shift: &Self) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Shl<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedShl<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn shl(self, shift: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Shl for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedShl<Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn shl(self, shift: Self) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

macro_rules! checked_shl_integer_impl {
    ($($integer:ty)*) => ($(
        impl<Digit, const DIGIT_BITNESS: usize> Shl<$integer>
            for BigInt<Digit, DIGIT_BITNESS>
        where
            Self: CheckedShl<
                $integer,
                Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>,
            >,
        {
            type Output = Self;

            fn shl(self, shift: $integer) -> Self::Output {
                self.checked_shl(shift).unwrap()
            }
        }
    )*)
}

checked_shl_integer_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
