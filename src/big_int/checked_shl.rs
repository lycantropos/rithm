use std::convert::TryFrom;
use std::mem::size_of;

use traiter::numbers::{CheckedShl, DivRem, Sign, Signed, Zeroable};

use super::digits::{PrimitiveShiftDigitsLeft, ShiftDigitsLeft};
use super::types::{BigInt, ShlError};

impl<Digit: ShiftDigitsLeft, const DIGIT_BITNESS: usize> CheckedShl
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: Signed,
{
    type Output = Result<Self, ShlError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, DIGIT_BITNESS> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<DIGIT_BITNESS>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self),
        }
    }
}

impl<Digit: ShiftDigitsLeft, const DIGIT_BITNESS: usize> CheckedShl<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: Signed,
{
    type Output = Result<Self, ShlError>;

    fn checked_shl(self, shift: &Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, DIGIT_BITNESS> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<DIGIT_BITNESS>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self),
        }
    }
}

impl<Digit: ShiftDigitsLeft, const DIGIT_BITNESS: usize>
    CheckedShl<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: Clone + Signed,
{
    type Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>;

    fn checked_shl(self, shift: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, DIGIT_BITNESS> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<DIGIT_BITNESS>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self.clone()),
        }
    }
}

impl<Digit: ShiftDigitsLeft, const DIGIT_BITNESS: usize> CheckedShl
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: Clone + Signed,
{
    type Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, DIGIT_BITNESS> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<DIGIT_BITNESS>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self.clone()),
        }
    }
}

macro_rules! checked_shl_signed_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: PrimitiveShiftDigitsLeft + TryFrom<usize>,
                const DIGIT_BITNESS: usize,
            > CheckedShl<$integer> for BigInt<Digit, DIGIT_BITNESS>
        {
            type Output = Result<Self, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || DIGIT_BITNESS < <$integer>::MAX as usize
                );
                match shift.sign() {
                    Sign::Negative => Err(ShlError::NegativeShift),
                    Sign::Positive => {
                        let (shift_quotient, shift_remainder) =
                            shift.div_rem(DIGIT_BITNESS as $integer);
                        if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                            >= (usize::BITS as usize)
                            && unsafe {
                                usize::try_from(shift_quotient)
                                    .unwrap_unchecked()
                            } >= (usize::MAX / size_of::<Digit>())
                        {
                            Err(ShlError::TooLarge)
                        } else {
                            let digits = Digit::primitive_shift_digits_left::<
                                DIGIT_BITNESS,
                            >(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            )
                            .ok_or(ShlError::OutOfMemory)?;
                            Ok(Self {
                                sign: self.sign,
                                digits,
                            })
                        }
                    }
                    Sign::Zero => Ok(self),
                }
            }
        }

        impl<
                Digit: PrimitiveShiftDigitsLeft + TryFrom<usize>,
                const DIGIT_BITNESS: usize,
            > CheckedShl<$integer> for &BigInt<Digit, DIGIT_BITNESS>
        where
            BigInt<Digit, DIGIT_BITNESS>: Clone,
        {
            type Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || DIGIT_BITNESS < <$integer>::MAX as usize
                );
                match shift.sign() {
                    Sign::Negative => Err(ShlError::NegativeShift),
                    Sign::Positive => {
                        let (shift_quotient, shift_remainder) =
                            shift.div_rem(DIGIT_BITNESS as $integer);
                        if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                            >= (usize::BITS as usize)
                            && unsafe {
                                usize::try_from(shift_quotient)
                                    .unwrap_unchecked()
                            } >= (usize::MAX / size_of::<Digit>())
                        {
                            Err(ShlError::TooLarge)
                        } else {
                            let digits = Digit::primitive_shift_digits_left::<
                                DIGIT_BITNESS,
                            >(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            )
                            .ok_or(ShlError::OutOfMemory)?;
                            Ok(BigInt::<Digit, DIGIT_BITNESS> {
                                sign: self.sign,
                                digits,
                            })
                        }
                    }
                    Sign::Zero => Ok(self.clone()),
                }
            }
        }
    )*)
}

checked_shl_signed_integer_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! checked_shl_unsigned_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: PrimitiveShiftDigitsLeft + TryFrom<usize>,
                const DIGIT_BITNESS: usize,
            > CheckedShl<$integer> for BigInt<Digit, DIGIT_BITNESS>
        where
            BigInt<Digit, DIGIT_BITNESS>: Zeroable,
        {
            type Output = Result<Self, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || DIGIT_BITNESS < <$integer>::MAX as usize
                );
                if shift.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) =
                        shift.div_rem(DIGIT_BITNESS as $integer);
                    if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                        >= (usize::BITS as usize)
                        && unsafe {
                            usize::try_from(shift_quotient).unwrap_unchecked()
                        } >= (usize::MAX / size_of::<Digit>())
                    {
                        Err(ShlError::TooLarge)
                    } else {
                        let digits =
                            Digit::primitive_shift_digits_left::<DIGIT_BITNESS>(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            )
                            .ok_or(ShlError::OutOfMemory)?;
                        Ok(Self {
                            sign: self.sign,
                            digits,
                        })
                    }
                }
            }
        }

        impl<
                Digit: PrimitiveShiftDigitsLeft + TryFrom<usize>,
                const DIGIT_BITNESS: usize,
            > CheckedShl<$integer> for &BigInt<Digit, DIGIT_BITNESS>
        where
            BigInt<Digit, DIGIT_BITNESS>: Clone + Zeroable,
        {
            type Output = Result<BigInt<Digit, DIGIT_BITNESS>, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || DIGIT_BITNESS < <$integer>::MAX as usize
                );
                if shift.is_zero() {
                    Ok(self.clone())
                } else {
                    let (shift_quotient, shift_remainder) =
                        shift.div_rem(DIGIT_BITNESS as $integer);
                    if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                        >= (usize::BITS as usize)
                        && unsafe {
                            usize::try_from(shift_quotient).unwrap_unchecked()
                        } >= (usize::MAX / size_of::<Digit>())
                    {
                        Err(ShlError::TooLarge)
                    } else {
                        let digits =
                            Digit::primitive_shift_digits_left::<DIGIT_BITNESS>(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            )
                            .ok_or(ShlError::OutOfMemory)?;
                        Ok(BigInt::<Digit, DIGIT_BITNESS> {
                            sign: self.sign,
                            digits,
                        })
                    }
                }
            }
        }
    )*)
}

checked_shl_unsigned_integer_impl!(u8 u16 u32 u64 u128 usize);
