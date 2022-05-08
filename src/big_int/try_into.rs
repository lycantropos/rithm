use std::convert::TryFrom;

use crate::traits::LdExp;

use super::digits::{fraction_exponent_digits, BinaryDigitConvertibleToFloat};
use super::types::{BigInt, TryIntoFloatError};

macro_rules! float_try_from_big_int {
    ($($t:ty)*) => ($(
        impl<
                Digit: BinaryDigitConvertibleToFloat<$t>,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            type Error = TryIntoFloatError;

            fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
                match fraction_exponent_digits::<Digit, $t, SHIFT>(&value.digits) {
                    Some((fraction_modulus, exponent)) => {
                        Ok(((value.sign as $t) * fraction_modulus).ldexp(exponent))
                    }
                    None => Err(TryIntoFloatError::TooLarge),
                }
            }
        }

        impl<
                Digit: BinaryDigitConvertibleToFloat<$t>,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<&BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            type Error = TryIntoFloatError;

            fn try_from(value: &BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
                match fraction_exponent_digits::<Digit, $t, SHIFT>(&value.digits) {
                    Some((fraction_modulus, exponent)) => {
                        Ok(((value.sign as $t) * fraction_modulus).ldexp(exponent))
                    }
                    None => Err(TryIntoFloatError::TooLarge),
                }
            }
        }
    )*)
}

float_try_from_big_int!(f32 f64);
