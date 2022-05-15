use std::mem::size_of;

use crate::traits::{CheckedShl, DivRem, Signed, Zeroable};

use super::digits::{primitive_shift_digits_left, shift_digits_left, ShiftableLeftDigit};
use super::types::{BigInt, ShlError};

impl<Digit: ShiftableLeftDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, ShlError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShlError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            Ok(Self {
                sign: self.sign,
                digits: shift_digits_left::<Digit, SHIFT>(&self.digits, &shift.digits)?,
            })
        }
    }
}

impl<Digit: ShiftableLeftDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, ShlError>;

    fn checked_shl(self, shift: &Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShlError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            Ok(Self {
                sign: self.sign,
                digits: shift_digits_left::<Digit, SHIFT>(&self.digits, &shift.digits)?,
            })
        }
    }
}

impl<Digit: ShiftableLeftDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedShl<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>;

    fn checked_shl(self, shift: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        if shift.is_negative() {
            Err(ShlError::NegativeShift)
        } else if self.is_zero() {
            Ok(self.clone())
        } else {
            Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                sign: self.sign,
                digits: shift_digits_left::<Digit, SHIFT>(&self.digits, &shift.digits)?,
            })
        }
    }
}

impl<Digit: ShiftableLeftDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShlError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShlError::NegativeShift)
        } else if self.is_zero() {
            Ok(self.clone())
        } else {
            Ok(BigInt::<Digit, SEPARATOR, SHIFT> {
                sign: self.sign,
                digits: shift_digits_left::<Digit, SHIFT>(&self.digits, &shift.digits)?,
            })
        }
    }
}

macro_rules! primitive_signed_checked_shl_impl {
    ($($t:ty)*) => ($(
        impl<Digit: ShiftableLeftDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, ShlError>;

            fn checked_shl(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if shift.is_negative() {
                    Err(ShlError::NegativeShift)
                } else if self.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) = shift.div_rem(SHIFT as $t);
                    if (<$t>::BITS as usize) + 8 * size_of::<Digit>() >= (usize::BITS as usize)
                        && shift_quotient >= ((usize::MAX / size_of::<Digit>()) as $t) {
                        Err(ShlError::TooLarge)
                    } else {
                        let digits = primitive_shift_digits_left::<Digit, SHIFT>(
                            &self.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
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
    )*)
}

primitive_signed_checked_shl_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! primitive_unsigned_checked_shl_impl {
    ($($t:ty)*) => ($(
        impl<Digit: ShiftableLeftDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, ShlError>;

            fn checked_shl(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if self.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) = shift.div_rem(SHIFT as $t);
                    if (<$t>::BITS as usize) + 8 * size_of::<Digit>() >= (usize::BITS as usize)
                        && shift_quotient >= ((usize::MAX / size_of::<Digit>()) as $t) {
                        Err(ShlError::TooLarge)
                    } else {
                        let digits = primitive_shift_digits_left::<Digit, SHIFT>(
                            &self.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
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
    )*)
}

primitive_unsigned_checked_shl_impl!(u8 u16 u32 u64 u128 usize);
