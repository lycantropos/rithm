use std::convert::TryFrom;

use crate::big_int::BigInt;
use crate::traits::TryDivAsFloat;

use super::types::Fraction;

macro_rules! try_float_from_big_int_fraction_impl {
    ($($float:ty)*) => ($(
        impl<Digit, const DIGIT_BITNESS: usize>
            TryFrom<Fraction<BigInt<Digit, DIGIT_BITNESS>>> for $float
        where
            BigInt<Digit, DIGIT_BITNESS>:
                TryDivAsFloat<BigInt<Digit, DIGIT_BITNESS>, $float>,
        {
            type Error = <BigInt<Digit, DIGIT_BITNESS> as TryDivAsFloat<
                BigInt<Digit, DIGIT_BITNESS>,
                $float,
            >>::Error;

            fn try_from(
                value: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
            ) -> Result<$float, Self::Error> {
                value.numerator.try_div_as_float(value.denominator)
            }
        }

        impl<'component, Digit, const DIGIT_BITNESS: usize>
            TryFrom<&'component Fraction<BigInt<Digit, DIGIT_BITNESS>>>
            for $float
        where
            for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
                TryDivAsFloat<&'a BigInt<Digit, DIGIT_BITNESS>, $float>,
        {
            type Error = <&'component BigInt<Digit, DIGIT_BITNESS> as TryDivAsFloat<&'component BigInt<Digit, DIGIT_BITNESS>, $float>>::Error;

            fn try_from(
                value: &'component Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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
