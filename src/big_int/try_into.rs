use std::convert::TryFrom;

use traiter::numbers::{LoadExp, Signed, Zeroable};

use super::digits::{FractExpDigits, MaybeReduceDigits};
use super::types::{
    BigInt, TryIntoFloatError, TryIntoSignedIntegerError,
    TryIntoUnsignedIntegerError,
};

macro_rules! float_try_from_big_int_impl {
    ($($float:ty)*) => ($(
        impl<
                Digit: FractExpDigits<$float>,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for $float
        {
            type Error = TryIntoFloatError;

            fn try_from(
                value: BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<Self, Self::Error> {
                match Digit::fract_exp_digits::<SHIFT>(&value.digits) {
                    Some((fraction_modulus, exponent)) => {
                        Ok(((value.sign as $float) * fraction_modulus)
                            .load_exp(exponent))
                    }
                    None => Err(TryIntoFloatError::TooLarge),
                }
            }
        }

        impl<
                Digit: FractExpDigits<$float>,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<&BigInt<Digit, SEPARATOR, SHIFT>> for $float
        {
            type Error = TryIntoFloatError;

            fn try_from(
                value: &BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<Self, Self::Error> {
                match Digit::fract_exp_digits::<SHIFT>(&value.digits) {
                    Some((fraction_modulus, exponent)) => {
                        Ok(((value.sign as $float) * fraction_modulus)
                            .load_exp(exponent))
                    }
                    None => Err(TryIntoFloatError::TooLarge),
                }
            }
        }
    )*)
}

float_try_from_big_int_impl!(f32 f64);

macro_rules! signed_integer_try_from_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: MaybeReduceDigits<$integer> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        {
            type Error = TryIntoSignedIntegerError;

            fn try_from(
                value: BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<Self, Self::Error> {
                let result =
                    Digit::maybe_reduce_digits::<SHIFT>(&value.digits)
                        .ok_or(TryIntoSignedIntegerError::TooLarge);
                if value.is_negative() {
                    result.map(|value| -value)
                } else {
                    result
                }
            }
        }

        impl<
                Digit: MaybeReduceDigits<$integer> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<&BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        {
            type Error = TryIntoSignedIntegerError;

            fn try_from(
                value: &BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<Self, Self::Error> {
                let result =
                    Digit::maybe_reduce_digits::<SHIFT>(&value.digits)
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

signed_integer_try_from_big_int_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_try_from_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: MaybeReduceDigits<$integer> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        {
            type Error = TryIntoUnsignedIntegerError;

            fn try_from(
                value: BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<Self, Self::Error> {
                if value.is_negative() {
                    Err(TryIntoUnsignedIntegerError::Negative)
                } else {
                    Digit::maybe_reduce_digits::<SHIFT>(&value.digits)
                        .ok_or(TryIntoUnsignedIntegerError::TooLarge)
                }
            }
        }

        impl<
                Digit: MaybeReduceDigits<$integer> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<&BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        {
            type Error = TryIntoUnsignedIntegerError;

            fn try_from(
                value: &BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Result<Self, Self::Error> {
                if value.is_negative() {
                    Err(TryIntoUnsignedIntegerError::Negative)
                } else {
                    Digit::maybe_reduce_digits::<SHIFT>(&value.digits)
                        .ok_or(TryIntoUnsignedIntegerError::TooLarge)
                }
            }
        }
    )*)
}

unsigned_integer_try_from_big_int_impl!(u8 u16 u32 u64 u128 usize);
