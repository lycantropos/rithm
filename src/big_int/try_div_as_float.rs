use crate::traits::TryDivAsFloat;

use super::digits::TryDivDigitsAsFloat;
use super::types::BigInt;

macro_rules! try_div_big_int_as_float {
    ($($float:ty)*) => ($(
        impl<
                Digit: TryDivDigitsAsFloat<$float>,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryDivAsFloat<Self, $float> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: Self,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<SHIFT>(
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
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryDivAsFloat<&Self, $float>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: &Self,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<SHIFT>(
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
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryDivAsFloat<BigInt<Digit, SEPARATOR, SHIFT>, $float>
            for &BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<SHIFT>(
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
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryDivAsFloat<Self, $float>
            for &BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Error = <Digit as TryDivDigitsAsFloat<$float>>::Error;

            fn try_div_as_float(
                self,
                divisor: Self,
            ) -> Result<$float, Self::Error> {
                Digit::checked_div_digits_as_float::<SHIFT>(
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

try_div_big_int_as_float!(f32 f64);
