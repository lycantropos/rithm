use std::convert::{FloatToInt, TryFrom};

use crate::big_int::{BigInt, DigitConvertibleFromF64, ShiftableLeftDigit};
use crate::contracts::is_signed;
use crate::traits::{CheckedShl, FrExp, Maybe, Unitary};

use super::types::{normalize_components_moduli, Fraction, FromFloatConversionError};

macro_rules! big_int_fraction_try_from_float_impl {
    ($($f:ty)*) => ($(
        impl<
                Digit: DigitConvertibleFromF64 + ShiftableLeftDigit + Unitary,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<$f> for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
        where
            $f: FloatToInt<BigInt<Digit, SEPARATOR, SHIFT>>,
        {
            type Error = FromFloatConversionError;

            fn try_from(value: $f) -> Result<Self, Self::Error> {
                if value.is_infinite() {
                    Err(FromFloatConversionError::Infinity)
                } else if value.is_nan() {
                    Err(FromFloatConversionError::NaN)
                } else {
                    let (mut fraction, mut exponent) = value.frexp();
                    for _ in 0..300 {
                        if fraction == fraction.floor() {
                            break;
                        }
                        fraction *= 2.0 as $f;
                        exponent -= 1;
                    }
                    let mut numerator =
                        unsafe { fraction.to_int_unchecked::<BigInt<Digit, SEPARATOR, SHIFT>>() };
                    let mut denominator = BigInt::<Digit, SEPARATOR, SHIFT>::one();
                    if exponent.is_negative() {
                        denominator = denominator.checked_shl((-exponent) as u32).result();
                    } else {
                        numerator = numerator.checked_shl(exponent as u32).result();
                    }
                    Ok(Self {
                        numerator,
                        denominator,
                    })
                }
            }
        }
    )*)
}

big_int_fraction_try_from_float_impl!(f32 f64);

macro_rules! primitive_fraction_try_from_float_impl {
    ($f:ty => $($t:ty)*) => ($(
        impl TryFrom<$f> for Fraction<$t> {
            type Error = FromFloatConversionError;

            fn try_from(value: $f) -> Result<Self, Self::Error> {
                if value.is_infinite() {
                    Err(FromFloatConversionError::Infinity)
                } else if value.is_nan() {
                    Err(FromFloatConversionError::NaN)
                } else if value.round() < (<$t>::MIN as $f) || value.round() > (<$t>::MAX as $f) {
                    Err(FromFloatConversionError::OutOfBounds)
                } else {
                    let (mut fraction, mut exponent) = value.frexp();
                    const MAX_EXPONENT_MODULUS: u32 = <$t>::BITS - 1 - (is_signed::<$t>() as u32);
                    if (exponent.abs() as u32) > MAX_EXPONENT_MODULUS {
                        if exponent.is_negative() {
                            fraction *= ((exponent + (MAX_EXPONENT_MODULUS as i32)) as $f).exp2();
                            exponent = -(MAX_EXPONENT_MODULUS as i32);
                        } else {
                            fraction *= ((exponent - (MAX_EXPONENT_MODULUS as i32)) as $f).exp2();
                            exponent = MAX_EXPONENT_MODULUS as i32;
                        };
                    }
                    while fraction != fraction.floor()
                        && (fraction.round() as $t) >= <$t>::MIN / 2
                        && (fraction.round() as $t) <= <$t>::MAX / 2
                        && (!exponent.is_negative() || ((-exponent) as u32) < MAX_EXPONENT_MODULUS)
                    {
                        fraction *= 2.0 as $f;
                        exponent -= 1;
                    }
                    if exponent.is_negative() {
                        let (numerator, denominator) = normalize_components_moduli(
                            fraction.round() as $t,
                            <$t>::one() << ((-exponent) as u32),
                        );
                        Ok(Self {
                            numerator,
                            denominator,
                        })
                    } else {
                        Ok(Self {
                            numerator: value.round() as $t,
                            denominator: <$t>::one(),
                        })
                    }
                }
            }
        }
    )*)
}

primitive_fraction_try_from_float_impl!(f32 => i8 i16 i32 i64 u8 u16 u32 u64);
primitive_fraction_try_from_float_impl!(f64 => i8 i16 i32 i64 u8 u16 u32 u64);
