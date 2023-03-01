use std::ops::Shr;

use traiter::numbers::CheckedShr;

use super::types::{BigInt, ShrError};

impl<Digit, const DIGIT_BITNESS: usize> Shr for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedShr<Output = Result<Self, ShrError>>,
{
    type Output = Self;

    fn shr(self, shift: Self) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Shr<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedShr<&'a Self, Output = Result<Self, ShrError>>,
{
    type Output = Self;

    fn shr(self, shift: &Self) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Shr<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedShr<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShrError>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn shr(self, shift: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Shr for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedShr<Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShrError>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn shr(self, shift: Self) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

macro_rules! checked_shr_integer_impl {
    ($($integer:ty)*) => ($(
        impl<Digit, const DIGIT_BITNESS: usize> Shr<$integer>
            for BigInt<Digit, DIGIT_BITNESS>
        where
            Self: CheckedShr<
                $integer,
                Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShrError>,
            >,
        {
            type Output = Self;

            fn shr(self, shift: $integer) -> Self::Output {
                self.checked_shr(shift).unwrap()
            }
        }
    )*)
}

checked_shr_integer_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
