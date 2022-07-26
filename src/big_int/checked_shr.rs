use std::convert::TryFrom;
use std::mem::size_of;
use std::ops::Not;

use traiter::numbers::{CheckedShr, DivRem, Sign, Signed, Zeroable};

use super::digits::{
    to_digits_sign, PrimitiveShiftDigitsRight, ShiftDigitsRight,
};
use super::types::{BigInt, ShrError};

impl<Digit: ShiftDigitsRight, const SEPARATOR: char, const SHIFT: usize>
    CheckedShr for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Signed,
{
    type Output = Result<Self, ShrError>;

    fn checked_shr(self, shift: Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShrError::NegativeShift),
            Sign::Positive => {
                let (sign, digits) = Digit::shift_digits_right::<SHIFT>(
                    self.sign,
                    &self.digits,
                    &shift.digits,
                );
                Ok(BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
            }
            Sign::Zero => Ok(self),
        }
    }
}

impl<Digit: ShiftDigitsRight, const SEPARATOR: char, const SHIFT: usize>
    CheckedShr<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Signed,
{
    type Output = Result<Self, ShrError>;

    fn checked_shr(self, shift: &Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShrError::NegativeShift),
            Sign::Positive => {
                let (sign, digits) = Digit::shift_digits_right::<SHIFT>(
                    self.sign,
                    &self.digits,
                    &shift.digits,
                );
                Ok(BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
            }
            Sign::Zero => Ok(self),
        }
    }
}

impl<Digit: ShiftDigitsRight, const SEPARATOR: char, const SHIFT: usize>
    CheckedShr<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone + Signed,
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>;

    fn checked_shr(
        self,
        shift: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShrError::NegativeShift),
            Sign::Positive => {
                let (sign, digits) = Digit::shift_digits_right::<SHIFT>(
                    self.sign,
                    &self.digits,
                    &shift.digits,
                );
                Ok(BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
            }
            Sign::Zero => Ok(self.clone()),
        }
    }
}

impl<Digit: ShiftDigitsRight, const SEPARATOR: char, const SHIFT: usize>
    CheckedShr for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone + Signed,
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>;

    fn checked_shr(self, shift: Self) -> Self::Output {
        match shift.sign() {
            Sign::Negative => Err(ShrError::NegativeShift),
            Sign::Positive => {
                let (sign, digits) = Digit::shift_digits_right::<SHIFT>(
                    self.sign,
                    &self.digits,
                    &shift.digits,
                );
                Ok(BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
            }
            Sign::Zero => Ok(self.clone()),
        }
    }
}

macro_rules! checked_shr_signed_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: PrimitiveShiftDigitsRight + TryFrom<usize> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > CheckedShr<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Self: Clone + Not<Output = Self>,
        {
            type Output = Result<Self, ShrError>;

            fn checked_shr(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                match shift.sign() {
                    Sign::Negative => Err(ShrError::NegativeShift),
                    Sign::Positive => {
                        let (shift_quotient, shift_remainder) =
                            shift.div_rem(SHIFT as $integer);
                        if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                            >= (usize::BITS as usize)
                            && shift_quotient
                                >= ((usize::MAX / size_of::<Digit>())
                                    as $integer)
                        {
                            Ok(Self::zero())
                        } else if self.is_negative() {
                            let inverted = !self;
                            let digits = Digit::primitive_shift_digits_right::<
                                SHIFT,
                            >(
                                &inverted.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            );
                            Ok(!Self {
                                sign: inverted.sign * to_digits_sign(&digits),
                                digits,
                            })
                        } else {
                            let digits = Digit::primitive_shift_digits_right::<
                                SHIFT,
                            >(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            );
                            Ok(Self {
                                sign: self.sign * to_digits_sign(&digits),
                                digits,
                            })
                        }
                    }
                    Sign::Zero => Ok(self),
                }
            }
        }

        impl<
                Digit: PrimitiveShiftDigitsRight + TryFrom<usize> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > CheckedShr<$integer> for &BigInt<Digit, SEPARATOR, SHIFT>
        where
            BigInt<Digit, SEPARATOR, SHIFT>:
                Clone + Not<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
            Self: Not<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
        {
            type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>;

            fn checked_shr(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                match shift.sign() {
                    Sign::Negative => Err(ShrError::NegativeShift),
                    Sign::Positive => {
                        let (shift_quotient, shift_remainder) =
                            shift.div_rem(SHIFT as $integer);
                        if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                            >= (usize::BITS as usize)
                            && shift_quotient
                                >= ((usize::MAX / size_of::<Digit>())
                                    as $integer)
                        {
                            Ok(BigInt::<Digit, SEPARATOR, SHIFT>::zero())
                        } else if self.is_negative() {
                            let inverted = !self;
                            let digits = Digit::primitive_shift_digits_right::<
                                SHIFT,
                            >(
                                &inverted.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            );
                            Ok(!BigInt::<Digit, SEPARATOR, SHIFT> {
                                sign: inverted.sign * to_digits_sign(&digits),
                                digits,
                            })
                        } else {
                            let digits = Digit::primitive_shift_digits_right::<
                                SHIFT,
                            >(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            );
                            Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                                sign: self.sign * to_digits_sign(&digits),
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

checked_shr_signed_integer_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! checked_shr_unsigned_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: PrimitiveShiftDigitsRight + TryFrom<usize> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > CheckedShr<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, ShrError>;

            fn checked_shr(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                if shift == 0 {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) =
                        shift.div_rem(SHIFT as $integer);
                    if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                        >= (usize::BITS as usize)
                        && shift_quotient
                            >= ((usize::MAX / size_of::<Digit>()) as $integer)
                    {
                        Ok(Self::zero())
                    } else {
                        let digits =
                            Digit::primitive_shift_digits_right::<SHIFT>(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            );
                        Ok(Self {
                            sign: self.sign * to_digits_sign(&digits),
                            digits,
                        })
                    }
                }
            }
        }

        impl<
                Digit: PrimitiveShiftDigitsRight + TryFrom<usize> + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > CheckedShr<$integer> for &BigInt<Digit, SEPARATOR, SHIFT>
        where
            BigInt<Digit, SEPARATOR, SHIFT>: Clone,
        {
            type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>;

            fn checked_shr(self, shift: $integer) -> Self::Output {
                debug_assert!(
                    usize::BITS < <$integer>::BITS
                        || SHIFT < <$integer>::MAX as usize
                );
                if shift == 0 {
                    Ok(self.clone())
                } else {
                    let (shift_quotient, shift_remainder) =
                        shift.div_rem(SHIFT as $integer);
                    if (<$integer>::BITS as usize) + 8 * size_of::<Digit>()
                        >= (usize::BITS as usize)
                        && shift_quotient
                            >= ((usize::MAX / size_of::<Digit>()) as $integer)
                    {
                        Ok(BigInt::<Digit, SEPARATOR, SHIFT>::zero())
                    } else {
                        let digits =
                            Digit::primitive_shift_digits_right::<SHIFT>(
                                &self.digits,
                                shift_quotient as usize,
                                unsafe {
                                    Digit::try_from(shift_remainder as usize)
                                        .unwrap_unchecked()
                                },
                            );
                        Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                            sign: self.sign * to_digits_sign(&digits),
                            digits,
                        })
                    }
                }
            }
        }
    )*)
}

checked_shr_unsigned_integer_impl!(u8 u16 u32 u64 u128 usize);
