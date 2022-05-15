use crate::traits::{CheckedPow, Signed};

use super::constants::{WINDOW_BASE, WINDOW_CUTOFF, WINDOW_SHIFT};
use super::digits::{binary_digits_to_lesser_binary_base, ExponentiativeDigit};
use super::types::{BigInt, WindowDigit};

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: Self) -> Self::Output {
        if exponent.is_negative() {
            None
        } else {
            Some(self.unchecked_pow(&exponent))
        }
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: &Self) -> Self::Output {
        if exponent.is_negative() {
            None
        } else {
            Some(self.unchecked_pow(exponent))
        }
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedPow<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow(self, exponent: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        if exponent.is_negative() {
            None
        } else {
            Some(self.unchecked_pow(&exponent))
        }
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<Self>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow(self, exponent: Self) -> Self::Output {
        if exponent.is_negative() {
            None
        } else {
            Some(self.unchecked_pow(exponent))
        }
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize>
    BigInt<Digit, SEPARATOR, SHIFT>
{
    fn unchecked_pow(&self, exponent: &Self) -> Self {
        debug_assert!(!exponent.is_negative());
        let mut exponent_digit = exponent.digits[exponent.digits.len() - 1];
        if exponent.digits.len() == 1 && exponent_digit <= Digit::from(3) {
            let mut result = Self::one();
            if exponent_digit >= Digit::from(2) {
                result = self * self;
                if exponent_digit == Digit::from(3) {
                    result *= self;
                }
            } else if exponent_digit.is_one() {
                result *= self;
            }
            result
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
            let mut exponent_digits_iterator = exponent.digits.iter().rev().skip(1);
            loop {
                while !exponent_digit_mask.is_zero() {
                    result = &result * &result;
                    if !(exponent_digit & exponent_digit_mask).is_zero() {
                        result *= self;
                    }
                    exponent_digit_mask >>= 1;
                }
                match exponent_digits_iterator.next() {
                    Some(next_exponent_digit) => {
                        exponent_digit = *next_exponent_digit;
                        exponent_digit_mask = Digit::one() << (SHIFT - 1);
                    }
                    None => {
                        break;
                    }
                }
            }
            result
        } else {
            let mut cache = vec![Self::zero(); WINDOW_BASE];
            cache[0] = Self::one();
            for index in 1..WINDOW_BASE {
                cache[index] = &cache[index - 1] * self;
            }
            let exponent_window_digits = binary_digits_to_lesser_binary_base::<Digit, WindowDigit>(
                &exponent.digits,
                SHIFT,
                WINDOW_SHIFT,
            );
            let mut result = Self::one();
            for &digit in exponent_window_digits.iter().rev() {
                for _ in 0..WINDOW_SHIFT {
                    result = &result * &result;
                }
                if !digit.is_zero() {
                    result *= &cache[digit as usize];
                }
            }
            result
        }
    }
}
