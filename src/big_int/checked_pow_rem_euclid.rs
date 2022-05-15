use crate::traits::{
    Abs, CheckedPowRemEuclid, CheckedRemEuclid, CheckedRemEuclidInv, Signed, Unitary, Zeroable,
};

use super::constants::{WINDOW_BASE, WINDOW_CUTOFF, WINDOW_SHIFT};
use super::digits::{
    binary_digits_to_lesser_binary_base, ExponentiativeDigit, ModularInvertibleDigit,
};
use super::types::{BigInt, CheckedPowRemEuclidError, WindowDigit};

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: Self) -> Self::Output {
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, &Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: &Self) -> Self::Output {
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<&Self, Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(self, exponent: &Self, divisor: Self) -> Self::Output {
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<&Self, &Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(self, exponent: &Self, divisor: &Self) -> Self::Output {
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>, BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>, Self>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(
        self,
        exponent: Self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, Self> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<BigInt<Digit, SEPARATOR, SHIFT>, CheckedPowRemEuclidError>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: Self) -> Self::Output {
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

impl<
        Digit: ExponentiativeDigit + ModularInvertibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BigInt<Digit, SEPARATOR, SHIFT>
{
    fn checked_pow_abs_rem_euclid(
        self,
        exponent: &Self,
        divisor: &Self,
    ) -> Result<Self, CheckedPowRemEuclidError> {
        debug_assert!(divisor.is_positive());
        if divisor.is_one() {
            return Ok(Self::zero());
        }
        let base = if exponent.is_negative() {
            self.checked_rem_euclid_inv(divisor)
                .ok_or(CheckedPowRemEuclidError::NonInvertibleBase)?
        } else {
            self
        };
        let mut exponent_digit = exponent.digits[exponent.digits.len() - 1];
        Ok(
            if exponent.digits.len() == 1 && exponent_digit <= Digit::from(3) {
                if exponent_digit >= Digit::from(2) {
                    let result = unsafe {
                        (&base * &base)
                            .checked_rem_euclid(divisor)
                            .unwrap_unchecked()
                    };
                    if exponent_digit == Digit::from(3) {
                        unsafe {
                            (result * base)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        }
                    } else {
                        result
                    }
                } else if exponent_digit.is_one() {
                    unsafe { base.checked_rem_euclid(divisor).unwrap_unchecked() }
                } else {
                    Self::one()
                }
            } else if exponent.digits.len() <= WINDOW_CUTOFF {
                let mut result = base.clone();
                let mut exponent_digit_mask = Digit::from(2);
                loop {
                    if exponent_digit_mask > exponent_digit {
                        exponent_digit_mask >>= 1;
                        break;
                    }
                    exponent_digit_mask <<= 1;
                }
                exponent_digit_mask >>= 1;
                let mut exponent_digits_iterator = exponent.digits.iter().rev().skip(1).peekable();
                loop {
                    while !exponent_digit_mask.is_zero() {
                        result = unsafe {
                            (&result * &result)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        };
                        if !(exponent_digit & exponent_digit_mask).is_zero() {
                            result = unsafe {
                                (result * &base)
                                    .checked_rem_euclid(divisor)
                                    .unwrap_unchecked()
                            };
                        }
                        exponent_digit_mask >>= 1;
                    }
                    if exponent_digits_iterator.peek().is_none() {
                        break;
                    }
                    exponent_digit = unsafe { *exponent_digits_iterator.next().unwrap_unchecked() };
                    exponent_digit_mask = Digit::one() << (SHIFT - 1);
                }
                result
            } else {
                let mut cache = vec![Self::zero(); WINDOW_BASE];
                cache[0] = Self::one();
                for index in 1..WINDOW_BASE {
                    cache[index] = unsafe {
                        (&cache[index - 1] * &base)
                            .checked_rem_euclid(divisor)
                            .unwrap_unchecked()
                    };
                }
                let exponent_window_digits = binary_digits_to_lesser_binary_base::<
                    Digit,
                    WindowDigit,
                >(
                    &exponent.digits, SHIFT, WINDOW_SHIFT
                );
                let mut result = Self::one();
                for &digit in exponent_window_digits.iter().rev() {
                    for _ in 0..WINDOW_SHIFT {
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
