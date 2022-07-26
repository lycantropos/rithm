use std::ops::Shr;

use traiter::numbers::CheckedShr;

use super::types::{BigInt, ShrError};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shr
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedShr<Output = Result<Self, ShrError>>,
{
    type Output = Self;

    fn shr(self, shift: Self) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shr<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedShr<&'a Self, Output = Result<Self, ShrError>>,
{
    type Output = Self;

    fn shr(self, shift: &Self) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Shr<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedShr<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn shr(self, shift: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shr
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self:
        CheckedShr<Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn shr(self, shift: Self) -> Self::Output {
        self.checked_shr(shift).unwrap()
    }
}

macro_rules! checked_shr_integer_impl {
    ($($integer:ty)*) => ($(
        impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shr<$integer>
            for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Self: CheckedShr<
                $integer,
                Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>,
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
