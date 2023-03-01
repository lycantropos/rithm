use std::convert::TryFrom;

use traiter::numbers::{FractExp, LoadExp, Unitary, Zeroable};

use crate::traits::UncheckedToInt;

use super::try_from_string::TryFromString;
use super::types::{BigInt, Sign, TryFromFloatError, TryFromStringError};

macro_rules! try_from_float_impl {
    ($($float:ty)*) => ($(
        impl<
                Digit: Copy + Zeroable,
                const DIGIT_BITNESS: usize,
            > TryFrom<$float> for BigInt<Digit, DIGIT_BITNESS>
        where
            $float: From<Digit> + UncheckedToInt<Digit>,
        {
            type Error = TryFromFloatError;

            fn try_from(mut value: $float) -> Result<Self, Self::Error> {
                debug_assert!(
                    usize::BITS < i32::BITS || DIGIT_BITNESS < (i32::MAX as usize)
                );
                if value.is_infinite() {
                    Err(TryFromFloatError::Infinity)
                } else if value.is_nan() {
                    Err(TryFromFloatError::NaN)
                } else if value.abs() < (1 as $float) {
                    Ok(Self::zero())
                } else {
                    let mut sign = Sign::one();
                    if value.is_sign_negative() {
                        sign = -sign;
                        value = -value;
                    }
                    let (fraction, exponent) = value.fract_exp();
                    let mut digits = vec![
                        Digit::zero();
                        ((exponent as usize) - 1) / DIGIT_BITNESS + 1
                    ];
                    let mut fraction =
                        fraction.load_exp((exponent - 1) % (DIGIT_BITNESS as i32) + 1);
                    for index in (0..digits.len()).rev() {
                        let digit =
                            unsafe { <$float>::unchecked_to_int(fraction) };
                        digits[index] = digit;
                        fraction -= <$float>::from(digit);
                        fraction = fraction.load_exp(DIGIT_BITNESS as i32);
                    }
                    Ok(Self { sign, digits })
                }
            }
        }
    )*)
}

try_from_float_impl!(f32 f64);

impl<Digit, const DIGIT_BITNESS: usize> TryFrom<&str>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: TryFromString,
{
    type Error = TryFromStringError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::try_from_string(string, 0)
    }
}
