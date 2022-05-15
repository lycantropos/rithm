use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::traits::Signed;

use super::constants::MAX_REPRESENTABLE_BASE;
use super::digits::{binary_digits_to_base, DisplayableDigit};
use super::types::BigInt;

impl<Digit: DisplayableDigit, const SEPARATOR: char, const SHIFT: usize> Display
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&self.to_base_string(10))
    }
}

impl<Digit: DisplayableDigit, const SEPARATOR: char, const SHIFT: usize>
    BigInt<Digit, SEPARATOR, SHIFT>
{
    const DIGIT_VALUES_ASCII_CODES: [char; MAX_REPRESENTABLE_BASE as usize] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    fn to_base_string(&self, base: usize) -> String {
        let shift =
            if (1usize << SHIFT) >= (MAX_REPRESENTABLE_BASE as usize) || base < (1usize << SHIFT) {
                unsafe { floor_log(1 << SHIFT, base).unwrap_unchecked() }
            } else {
                1usize
            };
        let digits = binary_digits_to_base::<Digit, Digit>(&self.digits, SHIFT, power(base, shift));
        let characters_count = (self.is_negative() as usize)
            + (digits.len() - 1) * shift
            + floor_log(
                unsafe { usize::try_from(digits[digits.len() - 1]).unwrap_unchecked() },
                base,
            )
            .unwrap_or(0usize)
            + 1;
        let mut characters: String = String::with_capacity(characters_count);
        let target_base = unsafe { Digit::try_from(base).unwrap_unchecked() };
        for &(mut remainder) in digits.iter().take(digits.len() - 1) {
            for _ in 0..shift {
                characters.push(
                    Self::DIGIT_VALUES_ASCII_CODES[unsafe {
                        usize::try_from(remainder.rem_euclid(target_base)).unwrap_unchecked()
                    }],
                );
                remainder /= target_base;
            }
        }
        let mut remainder = digits[digits.len() - 1];
        while !remainder.is_zero() {
            characters.push(
                Self::DIGIT_VALUES_ASCII_CODES[unsafe {
                    usize::try_from(remainder.rem_euclid(target_base)).unwrap_unchecked()
                }],
            );
            remainder /= target_base;
        }
        if self.is_zero() {
            characters.push('0');
        } else if self.is_negative() {
            characters.push('-');
        }
        characters.chars().rev().collect()
    }
}

const fn floor_log(value: usize, base: usize) -> Option<usize> {
    if value == 0usize {
        None
    } else if value < base {
        Some(0)
    } else {
        match floor_log(value / base, base) {
            Some(value) => Some(value + 1),
            None => None,
        }
    }
}

const fn power(base: usize, exponent: usize) -> usize {
    match exponent {
        0 => 1,
        _ => base * power(base, exponent - 1),
    }
}
