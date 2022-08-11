use std::convert::TryFrom;

use crate::big_int::BigInt;
use crate::traits::TryDivAsFloat;

use super::types::Fraction;

macro_rules! try_float_from_big_int_fraction_impl {
    ($($float:ty)*) => ($(
        impl<Digit, const SEPARATOR: char, const SHIFT: usize>
            TryFrom<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>> for $float
        where
            BigInt<Digit, SEPARATOR, SHIFT>:
                TryDivAsFloat<BigInt<Digit, SEPARATOR, SHIFT>, $float>,
        {
            type Error = <BigInt<Digit, SEPARATOR, SHIFT> as TryDivAsFloat<
                BigInt<Digit, SEPARATOR, SHIFT>,
                $float,
            >>::Error;

            fn try_from(
                value: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
            ) -> Result<$float, Self::Error> {
                value.numerator.try_div_as_float(value.denominator)
            }
        }

        impl<'component, Digit, const SEPARATOR: char, const SHIFT: usize>
            TryFrom<&'component Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
            for $float
        where
            for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
                TryDivAsFloat<&'a BigInt<Digit, SEPARATOR, SHIFT>, $float>,
        {
            type Error = <&'component BigInt<Digit, SEPARATOR, SHIFT> as TryDivAsFloat<&'component BigInt<Digit, SEPARATOR, SHIFT>, $float>>::Error;

            fn try_from(
                value: &'component Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
            ) -> Result<$float, Self::Error> {
                value.numerator.try_div_as_float(&value.denominator)
            }
        }
    )*)
}

try_float_from_big_int_fraction_impl!(f32 f64);

macro_rules! try_float_from_integer_fraction_impl {
    ($float:ty => $($integer:ty)*) => ($(
        impl TryFrom<Fraction<$integer>> for $float {
            type Error = <$integer as TryDivAsFloat<$integer, $float>>::Error;

            fn try_from(
                value: Fraction<$integer>,
            ) -> Result<$float, Self::Error> {
                value.numerator.try_div_as_float(value.denominator)
            }
        }
    )*)
}

try_float_from_integer_fraction_impl!(
    f32 => i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
try_float_from_integer_fraction_impl!(
    f64 => i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
