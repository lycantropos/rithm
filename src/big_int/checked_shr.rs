use std::mem::size_of;

use crate::traits::{CheckedShr, DivRem, Signed};

use super::digits::{
    primitive_shift_digits_right, shift_digits_right, to_digits_sign, ShiftableRightDigit,
};
use super::types::{BigInt, ShrError};

impl<Digit: ShiftableRightDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, ShrError>;

    fn checked_shr(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShrError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            let (sign, digits) =
                shift_digits_right::<Digit, SHIFT>(self.sign, &self.digits, &shift.digits);
            Ok(Self { sign, digits })
        }
    }
}

impl<Digit: ShiftableRightDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, ShrError>;

    fn checked_shr(self, shift: &Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShrError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            let (sign, digits) =
                shift_digits_right::<Digit, SHIFT>(self.sign, &self.digits, &shift.digits);
            Ok(Self { sign, digits })
        }
    }
}

impl<Digit: ShiftableRightDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedShr<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>;

    fn checked_shr(self, shift: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        if shift.is_negative() {
            Err(ShrError::NegativeShift)
        } else if self.is_zero() {
            Ok(self.clone())
        } else {
            let (sign, digits) =
                shift_digits_right::<Digit, SHIFT>(self.sign, &self.digits, &shift.digits);
            Ok(BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
        }
    }
}

impl<Digit: ShiftableRightDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, ShrError>;

    fn checked_shr(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShrError::NegativeShift)
        } else if self.is_zero() {
            Ok(self.clone())
        } else {
            let (sign, digits) =
                shift_digits_right::<Digit, SHIFT>(self.sign, &self.digits, &shift.digits);
            Ok(BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
        }
    }
}

macro_rules! primitive_signed_checked_shr_impl {
    ($($t:ty)*) => ($(
        impl<Digit: ShiftableRightDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, ShrError>;

            fn checked_shr(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if shift.is_negative() {
                    Err(ShrError::NegativeShift)
                } else if self.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) = shift.div_rem(SHIFT as $t);
                    if (<$t>::BITS as usize) + 8 * size_of::<Digit>() >= (usize::BITS as usize)
                        && shift_quotient >= ((usize::MAX / size_of::<Digit>()) as $t)
                    {
                        Ok(Self::zero())
                    } else if self.is_negative() {
                        let inverted = !self;
                        let digits = primitive_shift_digits_right::<Digit, SHIFT>(
                            &inverted.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
                        );
                        Ok(!Self {
                            sign: inverted.sign * to_digits_sign(&digits),
                            digits,
                        })
                    } else {
                        let digits = primitive_shift_digits_right::<Digit, SHIFT>(
                            &self.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
                        );
                        Ok(Self {
                            sign: self.sign * to_digits_sign(&digits),
                            digits,
                        })
                    }
                }
            }
        }
    )*)
}

primitive_signed_checked_shr_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! primitive_unsigned_checked_shr_impl {
    ($($t:ty)*) => ($(
        impl<Digit: ShiftableRightDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, ShrError>;

            fn checked_shr(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if self.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) = shift.div_rem(SHIFT as $t);
                    if (<$t>::BITS as usize) + 8 * size_of::<Digit>() >= (usize::BITS as usize)
                        && shift_quotient >= ((usize::MAX / size_of::<Digit>()) as $t)
                    {
                        Ok(Self::zero())
                    } else {
                        let digits = primitive_shift_digits_right::<Digit, SHIFT>(
                            &self.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
                        );
                        Ok(Self {
                            sign: self.sign * to_digits_sign(&digits),
                            digits,
                        })
                    }
                }
            }
        }
    )*)
}

primitive_unsigned_checked_shr_impl!(u8 u16 u32 u64 u128 usize);
