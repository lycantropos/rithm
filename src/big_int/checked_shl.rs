use std::convert::TryFrom;
use std::mem::size_of;

use traiter::numbers::{CheckedShl, DivRem, Sign, Signed, Zeroable};

use super::digits::{PrimitiveShiftDigitsLeft, ShiftDigitsLeft};
use super::types::{BigInt, ShlError};

impl<Digit: ShiftDigitsLeft, const SEPARATOR: char, const SHIFT: usize>
    CheckedShl for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Signed,
{
    type Output = Result<Self, ShlError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<SHIFT>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self),
        }
    }
}

impl<Digit: ShiftDigitsLeft, const SEPARATOR: char, const SHIFT: usize>
    CheckedShl<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Signed,
{
    type Output = Result<Self, ShlError>;

    fn checked_shl(self, shift: &Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<SHIFT>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self),
        }
    }
}

impl<Digit: ShiftDigitsLeft, const SEPARATOR: char, const SHIFT: usize>
    CheckedShl<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone + Signed,
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>;

    fn checked_shl(
        self,
        shift: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<SHIFT>(
                    &self.digits,
                    &shift.digits,
                )?,
            }),
            Sign::Zero => Ok(self.clone()),
        }
    }
}

impl<Digit: ShiftDigitsLeft, const SEPARATOR: char, const SHIFT: usize>
    CheckedShl for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone + Signed,
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShlError::NegativeShift),
            Sign::Positive => Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                sign: self.sign,
                digits: Digit::shift_digits_left::<SHIFT>(
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
                const SEPARATOR: char,
                const SHIFT: usize,
            > CheckedShl<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                match shift.sign() {
                    Sign::Negative => Err(ShlError::NegativeShift),
                    Sign::Positive => {
                        let (shift_quotient, shift_remainder) =
                            shift.div_rem(SHIFT as $integer);
                        if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                            >= (usize::BITS as usize)
                            && shift_quotient
                                >= ((usize::MAX / size_of::<Digit>())
                                    as $integer)
                        {
                            Err(ShlError::TooLarge)
                        } else {
                            let digits = Digit::primitive_shift_digits_left::<
                                SHIFT,
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
                const SEPARATOR: char,
                const SHIFT: usize,
            > CheckedShl<$integer> for &BigInt<Digit, SEPARATOR, SHIFT>
        where
            BigInt<Digit, SEPARATOR, SHIFT>: Clone,
        {
            type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                match shift.sign() {
                    Sign::Negative => Err(ShlError::NegativeShift),
                    Sign::Positive => {
                        let (shift_quotient, shift_remainder) =
                            shift.div_rem(SHIFT as $integer);
                        if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                            >= (usize::BITS as usize)
                            && shift_quotient
                                >= ((usize::MAX / size_of::<Digit>())
                                    as $integer)
                        {
                            Err(ShlError::TooLarge)
                        } else {
                            let digits = Digit::primitive_shift_digits_left::<
                                SHIFT,
                            >(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            )
                            .ok_or(ShlError::OutOfMemory)?;
                            Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
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
            const SEPARATOR: char,
            const SHIFT: usize,
        > CheckedShl<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
            where BigInt<Digit, SEPARATOR, SHIFT>: Zeroable
        {
            type Output = Result<Self, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                if shift.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) =
                        shift.div_rem(SHIFT as $integer);
                    if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                        >= (usize::BITS as usize)
                        && shift_quotient
                        >= ((usize::MAX / size_of::<Digit>()) as $integer)
                    {
                        Err(ShlError::TooLarge)
                    } else {
                        let digits =
                            Digit::primitive_shift_digits_left::<SHIFT>(
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
            const SEPARATOR: char,
            const SHIFT: usize,
        > CheckedShl<$integer> for &BigInt<Digit, SEPARATOR, SHIFT>
            where BigInt<Digit, SEPARATOR, SHIFT>: Clone + Zeroable
        {
            type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>;

            fn checked_shl(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                if shift.is_zero() {
                    Ok(self.clone())
                } else {
                    let (shift_quotient, shift_remainder) =
                        shift.div_rem(SHIFT as $integer);
                    if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                        >= (usize::BITS as usize)
                        && shift_quotient
                        >= ((usize::MAX / size_of::<Digit>()) as $integer)
                    {
                        Err(ShlError::TooLarge)
                    } else {
                        let digits =
                            Digit::primitive_shift_digits_left::<SHIFT>(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            )
                                .ok_or(ShlError::OutOfMemory)?;
                        Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
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
