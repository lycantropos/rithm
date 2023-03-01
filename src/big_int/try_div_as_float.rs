use crate::traits::TryDivAsFloat;

use super::digits::TryDivDigitsAsFloat;
use super::types::BigInt;

macro_rules! try_div_big_int_as_float_impl {
    ($($float:ty)*) => ($(
        impl<
                Digit: TryDivDigitsAsFloat<$float>,
                const DIGIT_BITNESS: usize,
            > TryDivAsFloat<Self, $float> for BigInt<Digit, DIGIT_BITNESS>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: Self,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<DIGIT_BITNESS>(
                    &self.digits,
                    &divisor.digits,
                )
                .map(|modulus| {
                    ((self.sign * divisor.sign) as $float) * modulus
                })
            }
        }

        impl<
                Digit: TryDivDigitsAsFloat<$float>,
                const DIGIT_BITNESS: usize,
            > TryDivAsFloat<&Self, $float>
            for BigInt<Digit, DIGIT_BITNESS>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: &Self,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<DIGIT_BITNESS>(
                    &self.digits,
                    &divisor.digits,
                )
                .map(|modulus| {
                    ((self.sign * divisor.sign) as $float) * modulus
                })
            }
        }

        impl<
                Digit: TryDivDigitsAsFloat<$float>,
                const DIGIT_BITNESS: usize,
            > TryDivAsFloat<BigInt<Digit, DIGIT_BITNESS>, $float>
            for &BigInt<Digit, DIGIT_BITNESS>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: BigInt<Digit, DIGIT_BITNESS>,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<DIGIT_BITNESS>(
                    &self.digits,
                    &divisor.digits,
                )
                .map(|modulus| {
                    ((self.sign * divisor.sign) as $float) * modulus
                })
            }
        }

        impl<
                Digit: TryDivDigitsAsFloat<$float>,
                const DIGIT_BITNESS: usize,
            > TryDivAsFloat<Self, $float>
            for &BigInt<Digit, DIGIT_BITNESS>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: Self,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<DIGIT_BITNESS>(
                    &self.digits,
                    &divisor.digits,
                )
                .map(|modulus| {
                    ((self.sign * divisor.sign) as $float) * modulus
                })
            }
        }
    )*)
}

try_div_big_int_as_float_impl!(f32 f64);
