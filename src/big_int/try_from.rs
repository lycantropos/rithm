use std::convert::TryFrom;

use crate::traits::Unitary;

use super::digits::{
    digits_from_finite_positive_improper_float, DigitConvertibleFromF64, FromStrDigit,
};
use super::types::{BigInt, Sign, TryFromFloatError, TryFromStringError};

impl<Digit: DigitConvertibleFromF64, const SEPARATOR: char, const SHIFT: usize> TryFrom<f64>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = TryFromFloatError;

    fn try_from(mut value: f64) -> Result<Self, Self::Error> {
        debug_assert!(usize::BITS < i32::BITS || SHIFT < (i32::MAX as usize));
        if value.is_infinite() {
            Err(TryFromFloatError::Infinity)
        } else if value.is_nan() {
            Err(TryFromFloatError::NaN)
        } else if value.abs() < f64::one() {
            Ok(Self::zero())
        } else {
            let mut sign = Sign::one();
            if value.is_sign_negative() {
                sign = -sign;
                value = -value;
            }
            Ok(Self {
                sign,
                digits: digits_from_finite_positive_improper_float::<Digit, f64, SHIFT>(value),
            })
        }
    }
}

impl<Digit: DigitConvertibleFromF64, const SEPARATOR: char, const SHIFT: usize> TryFrom<f32>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = TryFromFloatError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize> TryFrom<&str>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = TryFromStringError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::try_from_string(string, 0)
    }
}
