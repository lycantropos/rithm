use std::ops::{BitAnd, Mul, MulAssign, Shl, ShlAssign, ShrAssign};

use traiter::numbers::{CheckedPow, Signed, Unitary, Zeroable};

use super::constants::{WINDOW_BASE, WINDOW_CUTOFF, WINDOW_SHIFT};
use super::digits::LesserBinaryBaseFromBinaryDigits;
use super::types::{BigInt, WindowDigit};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: UncheckedPow + Signed,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: UncheckedPow + Signed,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedPow<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: UncheckedPow + Signed,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if exponent.is_negative() {
            None
        } else {
            Some(self.unchecked_pow(&exponent))
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<Self>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: UncheckedPow + Signed,
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

trait UncheckedPow: Sized {
    fn unchecked_pow(&self, exponent: &Self) -> Self;
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
        const SEPARATOR: char,
        const SHIFT: usize,
    > UncheckedPow for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: Mul<Output = Self> + MulAssign<&'a Self>,
    for<'a> &'a Self: Mul<Output = Self>,
    WindowDigit: LesserBinaryBaseFromBinaryDigits<Digit>,
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
            let mut exponent_digits_iterator =
                exponent.digits.iter().rev().skip(1);
            loop {
                while !exponent_digit_mask.is_zero() {
                    result = &result * &result;
                    if !(exponent_digit & exponent_digit_mask).is_zero() {
                        result *= self;
                    }
                    exponent_digit_mask >>= 1;
                }
                match exponent_digits_iterator.next() {
                    Some(&next_exponent_digit) => {
                        exponent_digit = next_exponent_digit;
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
            let exponent_window_digits: Vec<WindowDigit> =
                WindowDigit::lesser_binary_base_from_binary_digits(
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
