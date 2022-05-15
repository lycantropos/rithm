use std::convert::TryFrom;

use crate::traits::{LdExp, Signed, Zeroable};

use super::digits::{
    fraction_exponent_digits, maybe_reduce_digits, BinaryDigitConvertibleToFloat, MaybeReducibleTo,
};
use super::types::{
    BigInt, TryIntoFloatError, TryIntoSignedIntegerError, TryIntoUnsignedIntegerError,
};

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

macro_rules! signed_primitive_try_from_big_int {
    ($($t:ty)*) => ($(
        impl<Digit: MaybeReducibleTo<$t> + Zeroable, const SEPARATOR: char, const SHIFT: usize>
            TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            type Error = TryIntoSignedIntegerError;

            fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
                let result = maybe_reduce_digits::<Digit, $t, SHIFT>(&value.digits)
                    .ok_or(TryIntoSignedIntegerError::TooLarge);
                if value.is_negative() {
                    result.map(|value| -value)
                } else {
                    result
                }
            }
        }

        impl<Digit: MaybeReducibleTo<$t> + Zeroable, const SEPARATOR: char, const SHIFT: usize>
            TryFrom<&BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            type Error = TryIntoSignedIntegerError;

            fn try_from(value: &BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
                let result = maybe_reduce_digits::<Digit, $t, SHIFT>(&value.digits)
                    .ok_or(TryIntoSignedIntegerError::TooLarge);
                if value.is_negative() {
                    result.map(|value| -value)
                } else {
                    result
                }
            }
        }
    )*)
}

signed_primitive_try_from_big_int!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_primitive_try_from_big_int {
    ($($t:ty)*) => ($(
        impl<Digit: MaybeReducibleTo<$t> + Zeroable, const SEPARATOR: char, const SHIFT: usize>
            TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            type Error = TryIntoUnsignedIntegerError;

            fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
                if value.is_negative() {
                    Err(TryIntoUnsignedIntegerError::Negative)
                } else {
                    maybe_reduce_digits::<Digit, $t, SHIFT>(&value.digits)
                        .ok_or(TryIntoUnsignedIntegerError::TooLarge)
                }
            }
        }

        impl<Digit: MaybeReducibleTo<$t> + Zeroable, const SEPARATOR: char, const SHIFT: usize>
            TryFrom<&BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            type Error = TryIntoUnsignedIntegerError;

            fn try_from(value: &BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
                if value.is_negative() {
                    Err(TryIntoUnsignedIntegerError::Negative)
                } else {
                    maybe_reduce_digits::<Digit, $t, SHIFT>(&value.digits)
                        .ok_or(TryIntoUnsignedIntegerError::TooLarge)
                }
            }
        }
    )*)
}

unsigned_primitive_try_from_big_int!(u8 u16 u32 u64 u128 usize);
