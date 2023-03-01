use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::{Div, DivAssign};

use traiter::numbers::{RemEuclid, Signed, Zeroable};

use super::constants::MAX_REPRESENTABLE_BASE;
use super::digits::BaseFromBinaryDigits;
use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> Display
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: ToBaseString,
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&self.to_base_string(10))
    }
}

trait ToBaseString {
    const DIGIT_VALUES_ASCII_CODES: [char; MAX_REPRESENTABLE_BASE as usize] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd',
        'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    fn to_base_string(&self, base: usize) -> String;
}

impl<
        Digit: Copy
            + Div<Output = Digit>
            + DivAssign
            + BaseFromBinaryDigits<Digit>
            + RemEuclid<Output = Digit>
            + TryFrom<usize>
            + Zeroable,
        const DIGIT_BITNESS: usize,
    > ToBaseString for BigInt<Digit, DIGIT_BITNESS>
where
    usize: TryFrom<Digit>,
{
    fn to_base_string(&self, base: usize) -> String {
        let shift = if (1usize << DIGIT_BITNESS)
            >= (MAX_REPRESENTABLE_BASE as usize)
            || base < (1usize << DIGIT_BITNESS)
        {
            unsafe { floor_log(1 << DIGIT_BITNESS, base).unwrap_unchecked() }
        } else {
            1usize
        };
        let digits = Digit::base_from_binary_digits(
            &self.digits,
            DIGIT_BITNESS,
            power(base, shift),
        );
        let characters_count = usize::from(self.is_negative())
            + (digits.len() - 1) * shift
            + floor_log(
                unsafe {
                    usize::try_from(digits[digits.len() - 1])
                        .unwrap_unchecked()
                },
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
                        usize::try_from(remainder.rem_euclid(target_base))
                            .unwrap_unchecked()
                    }],
                );
                remainder /= target_base;
            }
        }
        let mut remainder = digits[digits.len() - 1];
        while !remainder.is_zero() {
            characters.push(
                Self::DIGIT_VALUES_ASCII_CODES[unsafe {
                    usize::try_from(remainder.rem_euclid(target_base))
                        .unwrap_unchecked()
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
