use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::mem::size_of;
use std::ops::Rem;

use crate::traits::{
    AssigningShiftingLeftMonoid, CheckedDivRemEuclid, CheckedShl, CheckedShr, DivRem, DivRemEuclid,
    Oppositive, RemEuclid,
};

use super::digits::*;
use super::types::BigInt;

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor).unwrap()
    }
}

impl<Digit: Clone + Eq + PartialOrd + ZeroableDigit, const SEPARATOR: char, const SHIFT: usize> Ord
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Rem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
                .unwrap();
        Self::Output { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> RemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .unwrap();
        Self::Output { sign, digits }
    }
}

impl<Digit: LeftShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, LeftShiftError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(LeftShiftError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            let (shift_quotient_digits, shift_remainder) =
                div_rem_digits_by_digit::<Digit, SHIFT>(&shift.digits, unsafe {
                    Digit::try_from(SHIFT).unwrap_unchecked()
                });
            let shift_quotient =
                checked_reduce_digits::<Digit, usize, SHIFT>(&shift_quotient_digits)
                    .ok_or(LeftShiftError::TooLarge)?;
            if shift_quotient >= usize::MAX / size_of::<Digit>() {
                Err(LeftShiftError::TooLarge)
            } else {
                let digits = shift_digits_left::<Digit, SHIFT>(
                    &self.digits,
                    shift_quotient,
                    shift_remainder,
                )
                .ok_or(LeftShiftError::OutOfMemory)?;
                Ok(Self {
                    sign: self.sign,
                    digits,
                })
            }
        }
    }
}

impl<Digit: RightShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, RightShiftError>;

    fn checked_shr(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(RightShiftError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            let (shift_quotient_digits, shift_remainder) =
                div_rem_digits_by_digit::<Digit, SHIFT>(&shift.digits, unsafe {
                    Digit::try_from(SHIFT).unwrap_unchecked()
                });
            let shift_quotient =
                checked_reduce_digits::<Digit, usize, SHIFT>(&shift_quotient_digits)
                    .unwrap_or(usize::MAX / size_of::<Digit>());
            if shift_quotient >= usize::MAX / size_of::<Digit>() {
                Ok(if self.is_negative() {
                    !Self::zero()
                } else {
                    Self::zero()
                })
            } else if self.is_negative() {
                let inverted = !self;
                let digits = shift_digits_right::<Digit, SHIFT>(
                    &inverted.digits,
                    shift_quotient,
                    shift_remainder,
                );
                Ok(!Self {
                    sign: inverted.sign * to_digits_sign(&digits),
                    digits,
                })
            } else {
                let digits = shift_digits_right::<Digit, SHIFT>(
                    &self.digits,
                    shift_quotient,
                    shift_remainder,
                );
                Ok(Self {
                    sign: self.sign * to_digits_sign(&digits),
                    digits,
                })
            }
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum LeftShiftError {
    NegativeShift,
    OutOfMemory,
    TooLarge,
}

impl LeftShiftError {
    fn description(&self) -> String {
        match self {
            LeftShiftError::NegativeShift => String::from("Shift by negative step is undefined."),
            LeftShiftError::OutOfMemory => String::from("Not enough memory for shift result."),
            LeftShiftError::TooLarge => String::from("Too large shift step."),
        }
    }
}

impl Debug for LeftShiftError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for LeftShiftError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Eq, PartialEq)]
pub enum RightShiftError {
    NegativeShift,
}

impl RightShiftError {
    fn description(&self) -> String {
        match self {
            RightShiftError::NegativeShift => String::from("Shift by negative step is undefined."),
        }
    }
}

impl Debug for RightShiftError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for RightShiftError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

macro_rules! plain_signed_checked_shl_impl {
    ($($t:ty)*) => ($(
        impl<Digit: LeftShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, LeftShiftError>;

            fn checked_shl(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if shift.is_negative() {
                    Err(LeftShiftError::NegativeShift)
                } else if self.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) = shift.div_rem(SHIFT as $t);
                    if (<$t>::BITS as usize) + 8 * size_of::<Digit>() >= (usize::BITS as usize)
                        && shift_quotient >= ((usize::MAX / size_of::<Digit>()) as $t) {
                        Err(LeftShiftError::TooLarge)
                    } else {
                        let digits = shift_digits_left::<Digit, SHIFT>(
                            &self.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
                        )
                        .ok_or(LeftShiftError::OutOfMemory)?;
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

plain_signed_checked_shl_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! plain_unsigned_checked_shl_impl {
    ($($t:ty)*) => ($(
        impl<Digit: LeftShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, LeftShiftError>;

            fn checked_shl(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if self.is_zero() {
                    Ok(self)
                } else {
                    let (shift_quotient, shift_remainder) = shift.div_rem(SHIFT as $t);
                    if (<$t>::BITS as usize) + 8 * size_of::<Digit>() >= (usize::BITS as usize)
                        && shift_quotient >= ((usize::MAX / size_of::<Digit>()) as $t) {
                        Err(LeftShiftError::TooLarge)
                    } else {
                        let digits = shift_digits_left::<Digit, SHIFT>(
                            &self.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
                        )
                        .ok_or(LeftShiftError::OutOfMemory)?;
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

plain_unsigned_checked_shl_impl!(u8 u16 u32 u64 u128 usize);

macro_rules! plain_signed_checked_shr_impl {
    ($($t:ty)*) => ($(
        impl<Digit: RightShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, RightShiftError>;

            fn checked_shr(self, shift: $t) -> Self::Output {
                debug_assert!(usize::BITS < <$t>::BITS || SHIFT < <$t>::MAX as usize);
                if shift.is_negative() {
                    Err(RightShiftError::NegativeShift)
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
                        let digits = shift_digits_right::<Digit, SHIFT>(
                            &inverted.digits,
                            shift_quotient as usize,
                            unsafe { Digit::try_from(shift_remainder as usize).unwrap_unchecked() },
                        );
                        Ok(!Self {
                            sign: inverted.sign * to_digits_sign(&digits),
                            digits,
                        })
                    } else {
                        let digits = shift_digits_right::<Digit, SHIFT>(
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

plain_signed_checked_shr_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! plain_unsigned_checked_shr_impl {
    ($($t:ty)*) => ($(
        impl<Digit: RightShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShr<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            type Output = Result<Self, RightShiftError>;

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
                        let digits = shift_digits_right::<Digit, SHIFT>(
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

plain_unsigned_checked_shr_impl!(u8 u16 u32 u64 u128 usize);

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT> {
    pub(crate) fn digits(&self) -> &[Digit] {
        &self.digits
    }
}
