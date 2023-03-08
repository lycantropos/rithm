use std::ops::{BitAnd, Mul, Shl, ShlAssign, ShrAssign, Sub};

use traiter::numbers::{
    Abs, CheckedPowRemEuclid, CheckedRemEuclid, CheckedRemEuclidInv, Signed,
    Unitary, Zeroable,
};

use super::constants::{WINDOW_BASE, WINDOW_BITNESS, WINDOW_CUTOFF};
use super::digits::LesserBinaryBaseFromBinaryDigits;
use super::types::{BigInt, CheckedPowRemEuclidError, WindowDigit};

impl<Digit, const DIGIT_BITNESS: usize> CheckedPowRemEuclid<Self, Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPowAbsRemEuclid + Signed + Sub<Output = Self>,
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: Self,
        divisor: Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.checked_pow_abs_rem_euclid(&exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedPowRemEuclid<Self, &Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPowAbsRemEuclid + Signed + Sub<Output = Self>,
    for<'a> &'a Self: Abs<Output = Self>,
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: Self,
        divisor: &Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.checked_pow_abs_rem_euclid(&exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedPowRemEuclid<&Self, Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPowAbsRemEuclid + Signed + Sub<Output = Self>,
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: &Self,
        divisor: Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.checked_pow_abs_rem_euclid(exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedPowRemEuclid<&Self, &Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPowAbsRemEuclid + Signed + Sub<Output = Self>,
    for<'a> &'a Self: Abs<Output = Self>,
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: &Self,
        divisor: &Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.checked_pow_abs_rem_euclid(exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPowRemEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        BigInt<Digit, DIGIT_BITNESS>,
    > for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedPowAbsRemEuclid
        + Clone
        + Signed
        + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output =
        Result<BigInt<Digit, DIGIT_BITNESS>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: BigInt<Digit, DIGIT_BITNESS>,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.clone()
                .checked_pow_abs_rem_euclid(&exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPowRemEuclid<BigInt<Digit, DIGIT_BITNESS>, Self>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedPowAbsRemEuclid
        + Clone
        + Signed
        + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    Self: Abs<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output =
        Result<BigInt<Digit, DIGIT_BITNESS>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: BigInt<Digit, DIGIT_BITNESS>,
        divisor: Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.clone()
                .checked_pow_abs_rem_euclid(&exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedPowRemEuclid<Self, BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedPowAbsRemEuclid
        + Clone
        + Signed
        + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output =
        Result<BigInt<Digit, DIGIT_BITNESS>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: Self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.clone()
                .checked_pow_abs_rem_euclid(exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedPowRemEuclid<Self, Self>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedPowAbsRemEuclid
        + Clone
        + Signed
        + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    Self: Abs<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output =
        Result<BigInt<Digit, DIGIT_BITNESS>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: Self,
        divisor: Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            Err(CheckedPowRemEuclidError::ZeroDivisor)
        } else {
            let is_negative = divisor.is_negative();
            let divisor_modulus = divisor.abs();
            self.clone()
                .checked_pow_abs_rem_euclid(exponent, &divisor_modulus)
                .map(|result| {
                    if is_negative && !result.is_zero() {
                        result - divisor_modulus
                    } else {
                        result
                    }
                })
        }
    }
}

pub trait CheckedPowAbsRemEuclid: Sized {
    fn checked_pow_abs_rem_euclid(
        self,
        exponent: &Self,
        divisor: &Self,
    ) -> Result<Self, CheckedPowRemEuclidError>;
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedPowAbsRemEuclid
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPowAbsRemEuclidImpl + Signed + Unitary,
    for<'a> Self: CheckedRemEuclidInv<&'a Self, Output = Option<Self>>,
{
    fn checked_pow_abs_rem_euclid(
        self,
        exponent: &Self,
        divisor: &Self,
    ) -> Result<Self, CheckedPowRemEuclidError> {
        debug_assert!(divisor.is_positive());
        if divisor.is_one() {
            Ok(Self::zero())
        } else {
            if exponent.is_negative() {
                self.checked_rem_euclid_inv(divisor)
                    .ok_or(CheckedPowRemEuclidError::NonInvertibleBase)?
            } else {
                self
            }
            .checked_pow_abs_rem_euclid_impl(exponent, divisor)
        }
    }
}

trait CheckedPowAbsRemEuclidImpl: Sized {
    fn checked_pow_abs_rem_euclid_impl(
        self,
        exponent: &Self,
        divisor: &Self,
    ) -> Result<Self, CheckedPowRemEuclidError>;
}

impl<
        Digit: BitAnd<Output = Digit>
            + Copy
            + From<u8>
            + PartialOrd
            + Shl<usize, Output = Digit>
            + ShlAssign<usize>
            + ShrAssign<usize>
            + Unitary
            + Zeroable,
        const DIGIT_BITNESS: usize,
    > CheckedPowAbsRemEuclidImpl for BigInt<Digit, DIGIT_BITNESS>
where
    Self: Unitary + Zeroable,
    for<'a> Self: CheckedRemEuclidInv<&'a Self, Output = Option<Self>>
        + CheckedRemEuclid<&'a Self, Output = Option<Self>>
        + Clone
        + Mul<&'a Self, Output = Self>
        + Mul<Self, Output = Self>,
    for<'a> &'a Self: Mul<Output = Self>,
    WindowDigit: LesserBinaryBaseFromBinaryDigits<Digit>,
{
    fn checked_pow_abs_rem_euclid_impl(
        self,
        exponent: &Self,
        divisor: &Self,
    ) -> Result<Self, CheckedPowRemEuclidError> {
        let mut exponent_digit = exponent.digits[exponent.digits.len() - 1];
        Ok(
            if exponent.digits.len() == 1 && exponent_digit <= Digit::from(3) {
                if exponent_digit >= Digit::from(2) {
                    let result = unsafe {
                        (&self * &self)
                            .checked_rem_euclid(divisor)
                            .unwrap_unchecked()
                    };
                    if exponent_digit == Digit::from(3) {
                        unsafe {
                            (result * self)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        }
                    } else {
                        result
                    }
                } else if exponent_digit.is_one() {
                    unsafe {
                        self.checked_rem_euclid(divisor).unwrap_unchecked()
                    }
                } else {
                    Self::one()
                }
            } else if exponent.digits.len() <= WINDOW_CUTOFF {
                let mut result = self.clone();
                let mut exponent_digit_mask = Digit::from(2);
                loop {
                    if exponent_digit_mask > exponent_digit {
                        exponent_digit_mask >>= 1;
                        break;
                    }
                    exponent_digit_mask <<= 1;
                }
                exponent_digit_mask >>= 1;
                let mut exponent_digits_iterator =
                    exponent.digits.iter().rev().skip(1).peekable();
                loop {
                    while !exponent_digit_mask.is_zero() {
                        result = unsafe {
                            (&result * &result)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        };
                        if !(exponent_digit & exponent_digit_mask).is_zero() {
                            result = unsafe {
                                (result * &self)
                                    .checked_rem_euclid(divisor)
                                    .unwrap_unchecked()
                            };
                        }
                        exponent_digit_mask >>= 1;
                    }
                    if exponent_digits_iterator.peek().is_none() {
                        break;
                    }
                    exponent_digit = unsafe {
                        *exponent_digits_iterator.next().unwrap_unchecked()
                    };
                    exponent_digit_mask = Digit::one() << (DIGIT_BITNESS - 1);
                }
                result
            } else {
                let mut cache = vec![Self::zero(); WINDOW_BASE];
                cache[0] = Self::one();
                for index in 1..WINDOW_BASE {
                    cache[index] = unsafe {
                        (&cache[index - 1] * &self)
                            .checked_rem_euclid(divisor)
                            .unwrap_unchecked()
                    };
                }
                let exponent_window_digits: Vec<WindowDigit> =
                    WindowDigit::lesser_binary_base_from_binary_digits(
                        &exponent.digits,
                        DIGIT_BITNESS,
                        WINDOW_BITNESS,
                    );
                let mut result = Self::one();
                for &digit in exponent_window_digits.iter().rev() {
                    for _ in 0..WINDOW_BITNESS {
                        result = unsafe {
                            (&result * &result)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        };
                    }
                    if !digit.is_zero() {
                        result = unsafe {
                            (&result * &cache[digit as usize])
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        };
                    }
                }
                result
            },
        )
    }
}
