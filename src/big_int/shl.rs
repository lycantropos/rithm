use std::ops::Shl;

use traiter::numbers::CheckedShl;

use super::types::{BigInt, ShlError};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shl
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedShl<Output = Result<Self, ShlError>>,
{
    type Output = Self;

    fn shl(self, shift: Self) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shl<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedShl<&'a Self, Output = Result<Self, ShlError>>,
{
    type Output = Self;

    fn shl(self, shift: &Self) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Shl<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedShl<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>,
    >,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn shl(self, shift: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shl
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self:
        CheckedShl<Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>>,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn shl(self, shift: Self) -> Self::Output {
        self.checked_shl(shift).unwrap()
    }
}

macro_rules! checked_shl_integer_impl {
    ($($integer:ty)*) => ($(
        impl<Digit, const SEPARATOR: char, const SHIFT: usize> Shl<$integer>
            for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Self: CheckedShl<
                $integer,
                Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>,
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
