use std::convert::TryFrom;

use traiter::numbers::{CheckedShl, FractExp, Unitary, Zeroable};

use crate::big_int::{BigInt, ShlError};
use crate::contracts::is_signed;
use crate::traits::UncheckedToInt;

use super::types::{Fraction, FromFloatConstructionError, NormalizeModuli};

macro_rules! big_int_fraction_try_from_float_impl {
    ($($float:ty)*) => ($(
        impl<
                Digit: Copy + TryFrom<usize> + Unitary + Zeroable,
                const DIGIT_BITNESS: usize,
            > TryFrom<$float> for Fraction<BigInt<Digit, DIGIT_BITNESS>>
        where
            BigInt<Digit, DIGIT_BITNESS>: CheckedShl<
                u32,
                Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>,
            >,
            $float: From<Digit> + UncheckedToInt<Digit>,
        {
            type Error = FromFloatConstructionError;

            fn try_from(value: $float) -> Result<Self, Self::Error> {
                if value.is_infinite() {
                    Err(FromFloatConstructionError::Infinity)
                } else if value.is_nan() {
                    Err(FromFloatConstructionError::NaN)
                } else {
                    let (mut fraction, mut exponent) = value.fract_exp();
                    for _ in 0..300 {
                        if fraction == fraction.floor() {
                            break;
                        }
                        fraction *= 2.0 as $float;
                        exponent -= 1;
                    }
                    let mut numerator = unsafe {
                        <$float as UncheckedToInt<
                            BigInt<Digit, DIGIT_BITNESS>,
                        >>::unchecked_to_int(fraction)
                    };
                    let mut denominator =
                        BigInt::<Digit, DIGIT_BITNESS>::one();
                    if exponent.is_negative() {
                        denominator = denominator
                            .checked_shl((-exponent) as u32)
                            .or(Err(FromFloatConstructionError::OutOfBounds))?;
                    } else {
                        numerator = numerator
                            .checked_shl(exponent as u32)
                            .or(Err(FromFloatConstructionError::OutOfBounds))?;
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

macro_rules! try_integer_fraction_from_float_impl {
    ($float:ty => $($integer:ty)*) => ($(
        impl TryFrom<$float> for Fraction<$integer> {
            type Error = FromFloatConstructionError;

            fn try_from(value: $float) -> Result<Self, Self::Error> {
                if value.is_infinite() {
                    Err(FromFloatConstructionError::Infinity)
                } else if value.is_nan() {
                    Err(FromFloatConstructionError::NaN)
                } else if value.round() < (<$integer>::MIN as $float)
                    || value.round() > (<$integer>::MAX as $float)
                {
                    Err(FromFloatConstructionError::OutOfBounds)
                } else {
                    const MAX_EXPONENT_MODULUS: u32 = <$integer>::BITS
                        - 1
                        - (is_signed::<$integer>() as u32);
                    let (mut fraction, mut exponent) = value.fract_exp();
                    if (exponent.abs() as u32) > MAX_EXPONENT_MODULUS {
                        if exponent.is_negative() {
                            fraction *= ((exponent
                                + (MAX_EXPONENT_MODULUS as i32))
                                as $float)
                                .exp2();
                            exponent = -(MAX_EXPONENT_MODULUS as i32);
                        } else {
                            fraction *= ((exponent
                                - (MAX_EXPONENT_MODULUS as i32))
                                as $float)
                                .exp2();
                            exponent = MAX_EXPONENT_MODULUS as i32;
                        };
                    }
                    while fraction != fraction.floor()
                        && (fraction.round() as $integer)
                            >= <$integer>::MIN / 2
                        && (fraction.round() as $integer)
                            <= <$integer>::MAX / 2
                        && (!exponent.is_negative()
                            || ((-exponent) as u32) < MAX_EXPONENT_MODULUS)
                    {
                        fraction *= 2.0 as $float;
                        exponent -= 1;
                    }
                    if exponent.is_negative() {
                        let (numerator, denominator) =
                            <$integer>::normalize_moduli(
                                fraction.round() as $integer,
                                <$integer>::one() << ((-exponent) as u32),
                            );
                        Ok(Self {
                            numerator,
                            denominator,
                        })
                    } else {
                        Ok(Self {
                            numerator: value.round() as $integer,
                            denominator: <$integer>::one(),
                        })
                    }
                }
            }
        }
    )*)
}

try_integer_fraction_from_float_impl!(f32 => i8 i16 i32 i64 u8 u16 u32 u64);
try_integer_fraction_from_float_impl!(f64 => i8 i16 i32 i64 u8 u16 u32 u64);
