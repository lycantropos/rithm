use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

use crate::traits::Unitary;

use super::digits::{
    digits_from_finite_positive_improper_float, DigitConvertibleFromFloat, FromStrDigit,
};
use super::try_from_string::TryFromStringError;
use super::types::{BigInt, Sign};

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize> TryFrom<f64>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = FromFloatConversionError;

    fn try_from(mut value: f64) -> Result<Self, Self::Error> {
        debug_assert!(usize::BITS < i32::BITS || SHIFT < (i32::MAX as usize));
        if value.is_infinite() {
            Err(FromFloatConversionError::Infinity)
        } else if value.is_nan() {
            Err(FromFloatConversionError::NaN)
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

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize> TryFrom<f32>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = FromFloatConversionError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

#[derive(Eq, PartialEq)]
pub enum FromFloatConversionError {
    Infinity,
    NaN,
}

impl FromFloatConversionError {
    fn description(&self) -> &str {
        match self {
            FromFloatConversionError::Infinity => "Conversion of infinity is undefined.",
            FromFloatConversionError::NaN => "Conversion of NaN is undefined.",
        }
    }
}

impl Debug for FromFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for FromFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
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
