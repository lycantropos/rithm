use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::mem::size_of;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, Sub, SubAssign};
use std::str::Chars;

use crate::digits::*;
use crate::traits::{
    Abs, AssigningShiftingLeftMonoid, BitwiseNegatableUnaryAlgebra, CheckedDiv, CheckedDivAsF32,
    CheckedDivAsF64, CheckedDivEuclid, CheckedDivRem, CheckedDivRemEuclid, CheckedPow,
    CheckedPowRemEuclid, CheckedRem, CheckedRemEuclid, CheckedRemEuclidInv, CheckedShl, CheckedShr,
    DivEuclid, DivRem, DivRemEuclid, DoublePrecisionOf, FromStrRadix, Gcd, Oppose, OppositionOf,
    Oppositive, Pow, RemEuclid, Unitary, Zeroable,
};
use crate::utils;

#[derive(Clone, PartialEq, Eq)]
pub struct BigInt<Digit, const SEPARATOR: char, const SHIFT: usize> {
    sign: Sign,
    digits: Vec<Digit>,
}

const MAX_REPRESENTABLE_BASE: u8 = 36;

pub enum ConversionError {
    TooLarge,
}

impl ConversionError {
    fn description(&self) -> &str {
        match self {
            ConversionError::TooLarge => "Too large to convert to floating point.",
        }
    }
}

impl Debug for ConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for ConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

pub enum ParsingError {
    StartsWithSeparator,
    ConsecutiveSeparators,
    InvalidDigit(char, u8),
    EndsWithSeparator,
}

impl ParsingError {
    fn description(&self) -> String {
        match self {
            ParsingError::StartsWithSeparator => String::from("Should not start with separator."),
            ParsingError::InvalidDigit(character, base) => {
                format!("Invalid digit in base {}: {}.", base, character)
            }
            ParsingError::ConsecutiveSeparators => String::from("Consecutive separators found."),
            ParsingError::EndsWithSeparator => String::from("Should not end with separator."),
        }
    }
}

impl Debug for ParsingError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for ParsingError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

pub enum ShiftError {
    NegativeShift,
    OutOfMemory,
    TooLarge,
}

impl ShiftError {
    fn description(&self) -> String {
        match self {
            ShiftError::NegativeShift => String::from("Shift by negative step is undefined."),
            ShiftError::OutOfMemory => String::from("Not enough memory for shift result."),
            ShiftError::TooLarge => String::from("Too large shift step."),
        }
    }
}

impl Debug for ShiftError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for ShiftError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize>
    BigInt<Digit, SEPARATOR, SHIFT>
{
    const ASCII_CODES_DIGIT_VALUES: [u8; 256] = [
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 37, 37, 37, 37, 37, 37, 37, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 37, 37, 37,
        37, 37, 37, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
        37,
    ];

    fn new(string: &str, mut base: u8) -> Result<Self, ParsingError> {
        debug_assert!(Self::ASCII_CODES_DIGIT_VALUES[SEPARATOR as usize] >= MAX_REPRESENTABLE_BASE);
        let mut characters = string.trim().chars().peekable();
        let sign = Self::parse_sign(&mut characters);
        if base == 0 {
            base = Self::guess_base(&mut characters);
        };
        Self::skip_prefix(&mut characters, base);
        Self::parse_digits(characters, base).map(|digits| {
            let digits = digits_to_binary_base::<u8, Digit, SHIFT>(&digits, base as usize);
            Self {
                sign: sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
                digits,
            }
        })
    }

    #[inline]
    fn guess_base(characters: &mut Peekable<Chars>) -> u8 {
        if characters.peek() != Some(&'0') {
            10
        } else {
            match characters.clone().nth(1) {
                Some('b') | Some('B') => 2,
                Some('o') | Some('O') => 8,
                Some('x') | Some('X') => 16,
                _ => 10,
            }
        }
    }

    #[inline]
    fn parse_digits(mut characters: Peekable<Chars>, base: u8) -> Result<Vec<u8>, ParsingError> {
        if characters.peek() == Some(&SEPARATOR) {
            return Err(ParsingError::StartsWithSeparator);
        }
        let mut result: Vec<u8> = Vec::new();
        let mut prev: char = SEPARATOR;
        for character in characters {
            if character != SEPARATOR {
                let digit = Self::ASCII_CODES_DIGIT_VALUES[character as usize];
                if digit >= base {
                    return Err(ParsingError::InvalidDigit(character, base));
                }
                result.push(digit);
            } else if prev == SEPARATOR {
                return Err(ParsingError::ConsecutiveSeparators);
            }
            prev = character;
        }
        if prev == SEPARATOR {
            return Err(ParsingError::EndsWithSeparator);
        }
        result.reverse();
        Ok(result)
    }

    #[inline]
    fn parse_sign(characters: &mut Peekable<Chars>) -> i8 {
        if characters.peek() == Some(&'-') {
            characters.next();
            -Sign::one()
        } else if characters.peek() == Some(&'+') {
            characters.next();
            Sign::one()
        } else {
            Sign::one()
        }
    }

    fn skip_prefix(characters: &mut Peekable<Chars>, base: u8) {
        if characters.peek() == Some(&'0') {
            match characters.clone().nth(1) {
                Some('b') | Some('B') => {
                    if base == 2 {
                        characters.nth(1);
                        characters.next_if_eq(&SEPARATOR);
                    }
                }
                Some('o') | Some('O') => {
                    if base == 8 {
                        characters.nth(1);
                        characters.next_if_eq(&SEPARATOR);
                    }
                }
                Some('x') | Some('X') => {
                    if base == 16 {
                        characters.nth(1);
                        characters.next_if_eq(&SEPARATOR);
                    }
                }
                _ => {}
            };
        };
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT> {
    pub(crate) fn digits(&self) -> &[Digit] {
        &self.digits
    }
}

impl<Digit: DisplayDigit, const SEPARATOR: char, const SHIFT: usize>
    BigInt<Digit, SEPARATOR, SHIFT>
{
    const DIGIT_VALUES_ASCII_CODES: [char; MAX_REPRESENTABLE_BASE as usize] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    fn to_base_string(&self, base: usize) -> String {
        let shift =
            if (1usize << SHIFT) >= (MAX_REPRESENTABLE_BASE as usize) || base < (1usize << SHIFT) {
                unsafe { utils::floor_log(1 << SHIFT, base).unwrap_unchecked() }
            } else {
                1usize
            };
        let digits =
            binary_digits_to_base::<Digit, Digit>(&self.digits, SHIFT, utils::power(base, shift));
        let characters_count = (self.is_negative() as usize)
            + (digits.len() - 1) * shift
            + utils::floor_log(
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

const MIDDLE_BYTE: u8 = 1u8 << (u8::BITS - 1);

impl<
        Digit: BinaryDigitConvertibleToBinary<Digit> + From<u8>,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: TryFrom<Digit>,
{
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut result =
            binary_digits_to_binary_base::<Digit, Digit>(&self.digits, SHIFT, u8::BITS as usize)
                .iter()
                .map(|&byte| unsafe { u8::try_from(byte).unwrap_unchecked() })
                .collect::<Vec<u8>>();
        let most_significant_byte = result[result.len() - 1];
        if most_significant_byte >= MIDDLE_BYTE
            && !(most_significant_byte == MIDDLE_BYTE
                && result.iter().rev().skip(1).all(Zeroable::is_zero)
                && self.is_negative())
        {
            result.push(0u8);
        }
        if self.is_negative() {
            negate_digits(&mut result);
        }
        result
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: BinaryDigitConvertibleToBinary<Digit>,
{
    pub(crate) fn from_bytes(mut bytes: Vec<u8>) -> Self {
        let most_significant_byte = bytes[bytes.len() - 1];
        let sign = if most_significant_byte >= MIDDLE_BYTE {
            negate_digits(&mut bytes);
            -Sign::one()
        } else {
            (bytes.len() > 1 || !bytes[0].is_zero()) as Sign
        };
        Self {
            sign,
            digits: binary_digits_to_binary_base::<u8, Digit>(
                &bytes[..bytes.len()
                    - ((bytes.len() > 1 && bytes[bytes.len() - 1].is_zero()) as usize)],
                u8::BITS as usize,
                SHIFT,
            ),
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Abs for BigInt<Digit, SEPARATOR, SHIFT> {
    type Output = Self;

    fn abs(self) -> Self {
        Self {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(&self.digits, self.sign, &other.digits, other.sign);
        Self { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            sum_signed_digits::<Digit, SHIFT>(&self.digits, self.sign, &other.digits, other.sign);
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        let digits_count = self.digits.len();
        let divisor_digits_count = divisor.digits.len();
        if divisor.is_zero() {
            None
        } else if self.is_zero()
            || digits_count < divisor_digits_count
            || (digits_count == divisor_digits_count
                && self.digits[self.digits.len() - 1] < divisor.digits[divisor.digits.len() - 1])
        {
            Some((Self::zero(), self))
        } else if divisor_digits_count == 1 {
            let (quotient_digits, remainder_digit) =
                div_rem_digits_by_digit::<Digit, SHIFT>(&self.digits, divisor.digits[0]);
            Some((
                Self {
                    sign: self.sign * divisor.sign,
                    digits: quotient_digits,
                },
                Self {
                    sign: self.sign * ((!remainder_digit.is_zero()) as Sign),
                    digits: vec![remainder_digit],
                },
            ))
        } else {
            let (quotient_digits, remainder_digits) =
                div_rem_two_or_more_digits::<Digit, SHIFT>(&self.digits, &divisor.digits);
            Some((
                Self {
                    sign: self.sign
                        * divisor.sign
                        * ((quotient_digits.len() > 1 || !quotient_digits[0].is_zero()) as Sign),
                    digits: quotient_digits,
                },
                Self {
                    sign: self.sign
                        * ((remainder_digits.len() > 1 || !remainder_digits[0].is_zero()) as Sign),
                    digits: remainder_digits,
                },
            ))
        }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem(divisor.clone())
            .map(|(mut quotient, mut modulo)| {
                if (divisor.is_negative() && modulo.is_positive())
                    || (divisor.is_positive() && modulo.is_negative())
                {
                    quotient -= Self::one();
                    modulo += divisor;
                }
                (quotient, modulo)
            })
    }
}

const WINDOW_CUTOFF: usize = 8;
const WINDOW_SHIFT: usize = 5;
const WINDOW_BASE: usize = 1 << WINDOW_SHIFT;

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> CheckedPow<Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: Self) -> Self::Output {
        if exponent.is_negative() {
            return None;
        }
        let mut result = Self::one();
        let mut exponent_digit = exponent.digits[exponent.digits.len() - 1];
        if exponent.digits.len() == 1 && exponent_digit <= Digit::from(3) {
            if exponent_digit >= Digit::from(2) {
                result = self.clone() * self.clone();
                if exponent_digit == Digit::from(3) {
                    result *= self;
                }
            } else if exponent_digit.is_one() {
                result *= self;
            }
        } else if exponent.digits.len() <= WINDOW_CUTOFF {
            result = self.clone();
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
                    result *= result.clone();
                    if !(exponent_digit & exponent_digit_mask).is_zero() {
                        result *= self.clone();
                    }
                    exponent_digit_mask >>= 1;
                }
                if exponent_digits_iterator.peek().is_none() {
                    break;
                }
                exponent_digit = unsafe { *exponent_digits_iterator.next().unwrap_unchecked() };
                exponent_digit_mask = Digit::one() << (SHIFT - 1);
            }
        } else {
            let mut cache = vec![Self::zero(); WINDOW_BASE];
            cache[0] = result.clone();
            for index in 1..WINDOW_BASE {
                cache[index] = cache[index - 1].clone() * self.clone();
            }
            let exponent_window_digits = binary_digits_to_lesser_binary_base::<Digit, WindowDigit>(
                &exponent.digits,
                SHIFT,
                WINDOW_SHIFT,
            );
            for &digit in exponent_window_digits.iter().rev() {
                for _ in 0..WINDOW_SHIFT {
                    result *= result.clone();
                }
                if !digit.is_zero() {
                    result *= cache[digit as usize].clone();
                }
            }
        }
        Some(result)
    }
}

impl<
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let is_negative = divisor.is_negative();
        let divisor = divisor.abs();
        if divisor.is_one() {
            return Some(Self::zero());
        }
        let (base, exponent) = if exponent.is_negative() {
            (self.checked_rem_euclid_inv(divisor.clone())?, -exponent)
        } else {
            (self, exponent)
        };
        let mut result = Self::one();
        let mut exponent_digit = exponent.digits[exponent.digits.len() - 1];
        if exponent.digits.len() == 1 && exponent_digit <= Digit::from(3) {
            if exponent_digit >= Digit::from(2) {
                result = (base.clone() * base.clone()).rem_euclid(divisor.clone());
                if exponent_digit == Digit::from(3) {
                    result = (result * base).rem_euclid(divisor.clone());
                }
            } else if exponent_digit.is_one() {
                result = (result * base).rem_euclid(divisor.clone());
            }
        } else if exponent.digits.len() <= WINDOW_CUTOFF {
            result = base.clone();
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
                    result = (result.clone() * result).rem_euclid(divisor.clone());
                    if !(exponent_digit & exponent_digit_mask).is_zero() {
                        result = (result * base.clone()).rem_euclid(divisor.clone());
                    }
                    exponent_digit_mask >>= 1;
                }
                if exponent_digits_iterator.peek().is_none() {
                    break;
                }
                exponent_digit = unsafe { *exponent_digits_iterator.next().unwrap_unchecked() };
                exponent_digit_mask = Digit::one() << (SHIFT - 1);
            }
        } else {
            let mut cache = vec![Self::zero(); WINDOW_BASE];
            cache[0] = result.clone();
            for index in 1..WINDOW_BASE {
                cache[index] =
                    (cache[index - 1].clone() * base.clone()).rem_euclid(divisor.clone());
            }
            let exponent_window_digits = binary_digits_to_lesser_binary_base::<Digit, WindowDigit>(
                &exponent.digits,
                SHIFT,
                WINDOW_SHIFT,
            );
            for &digit in exponent_window_digits.iter().rev() {
                for _ in 0..WINDOW_SHIFT {
                    result = (result.clone() * result).rem_euclid(divisor.clone());
                }
                if !digit.is_zero() {
                    result = (result * cache[digit as usize].clone()).rem_euclid(divisor.clone());
                }
            }
        }
        Some(if is_negative && !result.is_zero() {
            result - divisor
        } else {
            result
        })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_rem_euclid::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclidInv
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
        let mut candidate = Self::zero();
        let mut result = Self::one();
        let mut step_dividend = self;
        let mut step_divisor = divisor.clone();
        while !step_divisor.is_zero() {
            let (quotient, remainder) = step_dividend.div_rem_euclid(step_divisor.clone());
            step_dividend = step_divisor;
            step_divisor = remainder;
            (result, candidate) = (candidate.clone(), result - quotient * candidate);
        }
        if step_dividend.is_one() {
            Some(if result.is_negative() {
                divisor + result
            } else {
                result
            })
        } else {
            None
        }
    }
}

impl<Digit: DisplayDigit, const SEPARATOR: char, const SHIFT: usize> Display
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&self.to_base_string(10))
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Div
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div::<Digit, SHIFT>(
            self.digits.as_slice(),
            self.sign,
            divisor.digits.as_slice(),
            divisor.sign,
        )
        .unwrap();
        Self { sign, digits }
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f32> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF32 for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivApproximationError>;

    fn checked_div_as_f32(self, divisor: Self) -> Self::Output {
        checked_div_approximation::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64 for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivApproximationError>;

    fn checked_div_as_f64(self, divisor: Self) -> Self::Output {
        checked_div_approximation::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn div_assign(&mut self, divisor: Self) {
        (self.sign, self.digits) = checked_div::<Digit, SHIFT>(
            self.digits.as_slice(),
            self.sign,
            divisor.digits.as_slice(),
            divisor.sign,
        )
        .unwrap();
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div_euclid::<Digit, SHIFT>(
            &self.digits,
            self.sign,
            &divisor.digits,
            divisor.sign,
        )
        .unwrap();
        Self { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem(self, divisor: Self) -> Self::Output {
        self.checked_div_rem(divisor).unwrap()
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> DivRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = (Self, Self);

    fn div_rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_rem_euclid(divisor).unwrap()
    }
}

impl<SourceDigit, TargetDigit, const SEPARATOR: char, const SHIFT: usize> From<SourceDigit>
    for BigInt<TargetDigit, SEPARATOR, SHIFT>
where
    SourceDigit: BinaryDigit + Oppose,
    TargetDigit: BinaryDigit + Oppose + TryFrom<SourceDigit>,
{
    fn from(mut value: SourceDigit) -> Self {
        if value.is_zero() {
            Self::zero()
        } else if size_of::<SourceDigit>() < size_of::<TargetDigit>()
            || (size_of::<SourceDigit>() == size_of::<TargetDigit>()
                && utils::are_same::<SourceDigit, OppositionOf<SourceDigit>>()
                && !utils::are_same::<TargetDigit, OppositionOf<TargetDigit>>())
        {
            let mut value = unsafe { TargetDigit::try_from(value).unwrap_unchecked() };
            let sign = Sign::one();
            let mut digits = Vec::<TargetDigit>::new();
            let digit_mask = to_digit_mask::<TargetDigit>(SHIFT);
            while !value.is_zero() {
                digits.push(value & digit_mask);
                value >>= SHIFT;
            }
            Self { sign, digits }
        } else {
            let sign = Sign::one();
            let mut digits = Vec::<TargetDigit>::new();
            let digit_mask = to_digit_mask::<SourceDigit>(SHIFT);
            while !value.is_zero() {
                digits
                    .push(unsafe { TargetDigit::try_from(value & digit_mask).unwrap_unchecked() });
                value >>= SHIFT;
            }
            Self { sign, digits }
        }
    }
}

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize> FromStrRadix
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = ParsingError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error> {
        if (radix != 0 && radix < 2) || radix > (MAX_REPRESENTABLE_BASE as u32) {
            panic!(
                "Radix should be in range from 2 to {}.",
                MAX_REPRESENTABLE_BASE
            );
        }
        Self::new(string, radix as u8)
    }
}

impl<Digit: GcdDigit, const SEPARATOR: char, const SHIFT: usize> Gcd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn gcd(self, other: Self) -> Self {
        let mut largest_digits = self.digits;
        let mut smallest_digits = other.digits;
        if digits_lesser_than(&largest_digits, &smallest_digits) {
            (largest_digits, smallest_digits) = (smallest_digits, largest_digits);
        }
        loop {
            let largest_digits_count = largest_digits.len();
            if largest_digits_count <= 2 {
                break;
            }
            let smallest_digits_count = smallest_digits.len();
            if smallest_digits_count == 1 && smallest_digits[0].is_zero() {
                return Self {
                    sign: Sign::one(),
                    digits: largest_digits,
                };
            }
            let highest_digit_bit_length =
                utils::bit_length(largest_digits[largest_digits.len() - 1]);
            let mut largest_leading_bits = (OppositionOf::<DoublePrecisionOf<Digit>>::from(
                largest_digits[largest_digits_count - 1],
            ) << (2 * SHIFT - highest_digit_bit_length))
                | (OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    largest_digits[largest_digits_count - 2],
                ) << (SHIFT - highest_digit_bit_length))
                | OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    largest_digits[largest_digits_count - 3] >> highest_digit_bit_length,
                );
            let mut smallest_leading_bits = if smallest_digits_count >= largest_digits_count - 2 {
                OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    smallest_digits[largest_digits_count - 3] >> highest_digit_bit_length,
                )
            } else {
                OppositionOf::<DoublePrecisionOf<Digit>>::zero()
            } | if smallest_digits_count >= largest_digits_count - 1
            {
                OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    smallest_digits[largest_digits_count - 2],
                ) << (SHIFT - highest_digit_bit_length)
            } else {
                OppositionOf::<DoublePrecisionOf<Digit>>::zero()
            } | if smallest_digits_count >= largest_digits_count {
                OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    smallest_digits[largest_digits_count - 1],
                ) << (2 * SHIFT - highest_digit_bit_length)
            } else {
                OppositionOf::<DoublePrecisionOf<Digit>>::zero()
            };
            let mut first_coefficient = OppositionOf::<DoublePrecisionOf<Digit>>::one();
            let mut second_coefficient = OppositionOf::<DoublePrecisionOf<Digit>>::zero();
            let mut third_coefficient = OppositionOf::<DoublePrecisionOf<Digit>>::zero();
            let mut fourth_coefficient = OppositionOf::<DoublePrecisionOf<Digit>>::one();
            let mut iterations_count = 0usize;
            loop {
                if third_coefficient == smallest_leading_bits {
                    break;
                }
                let scale = (largest_leading_bits
                    + (first_coefficient - OppositionOf::<DoublePrecisionOf<Digit>>::one()))
                    / (smallest_leading_bits - third_coefficient);
                let next_third_coefficient = second_coefficient + scale * fourth_coefficient;
                let next_smallest_leading_bits =
                    largest_leading_bits - scale * smallest_leading_bits;
                if next_third_coefficient > next_smallest_leading_bits {
                    break;
                }
                largest_leading_bits = smallest_leading_bits;
                smallest_leading_bits = next_smallest_leading_bits;
                let next_fourth_coefficient = first_coefficient + scale * third_coefficient;
                first_coefficient = fourth_coefficient;
                second_coefficient = third_coefficient;
                third_coefficient = next_third_coefficient;
                fourth_coefficient = next_fourth_coefficient;
                iterations_count += 1;
            }
            if iterations_count == 0 {
                (largest_digits, smallest_digits) = if smallest_digits_count == 1 {
                    let (_, remainder) = div_rem_digits_by_digit::<Digit, SHIFT>(
                        &largest_digits,
                        smallest_digits[0],
                    );
                    (smallest_digits, vec![remainder])
                } else {
                    let (_, remainder) = div_rem_two_or_more_digits::<Digit, SHIFT>(
                        &largest_digits,
                        &smallest_digits,
                    );
                    (smallest_digits, remainder)
                };
                continue;
            }
            if iterations_count % 2 != 0 {
                (first_coefficient, second_coefficient) = (-second_coefficient, -first_coefficient);
                (third_coefficient, fourth_coefficient) = (-fourth_coefficient, -third_coefficient);
            }
            let digit_mask = to_digit_mask::<OppositionOf<DoublePrecisionOf<Digit>>>(SHIFT);
            let mut next_largest_accumulator = OppositionOf::<DoublePrecisionOf<Digit>>::zero();
            let mut next_smallest_accumulator = OppositionOf::<DoublePrecisionOf<Digit>>::zero();
            let mut next_largest_digits = Vec::<Digit>::with_capacity(largest_digits_count);
            let mut next_smallest_digits = Vec::<Digit>::with_capacity(largest_digits_count);
            for index in 0..smallest_digits_count {
                next_largest_accumulator = next_largest_accumulator
                    + (first_coefficient
                        * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]))
                    - (second_coefficient
                        * OppositionOf::<DoublePrecisionOf<Digit>>::from(smallest_digits[index]));
                next_smallest_accumulator = next_smallest_accumulator
                    + (fourth_coefficient
                        * OppositionOf::<DoublePrecisionOf<Digit>>::from(smallest_digits[index]))
                    - (third_coefficient
                        * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]));
                next_largest_digits.push(unsafe {
                    Digit::try_from(next_largest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_smallest_digits.push(unsafe {
                    Digit::try_from(next_smallest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_largest_accumulator >>= SHIFT;
                next_smallest_accumulator >>= SHIFT;
            }
            for index in smallest_digits_count..largest_digits_count {
                next_largest_accumulator += first_coefficient
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]);
                next_smallest_accumulator -= third_coefficient
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]);
                next_largest_digits.push(unsafe {
                    Digit::try_from(next_largest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_smallest_digits.push(unsafe {
                    Digit::try_from(next_smallest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_largest_accumulator >>= SHIFT;
                next_smallest_accumulator >>= SHIFT;
            }
            trim_leading_zeros(&mut next_largest_digits);
            trim_leading_zeros(&mut next_smallest_digits);
            largest_digits = next_largest_digits;
            smallest_digits = next_smallest_digits;
        }
        Self::from(utils::gcd::<DoublePrecisionOf<Digit>>(
            reduce_digits::<Digit, DoublePrecisionOf<Digit>, SHIFT>(&largest_digits),
            reduce_digits::<Digit, DoublePrecisionOf<Digit>, SHIFT>(&smallest_digits),
        ))
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> Mul
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> MulAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn mul_assign(&mut self, other: Self) {
        self.sign *= other.sign;
        self.digits = multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Neg for BigInt<Digit, SEPARATOR, SHIFT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            sign: -self.sign,
            digits: self.digits,
        }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Not
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn not(self) -> Self::Output {
        -(self + Self::one())
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Oppositive
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: Zeroable,
{
    fn is_negative(&self) -> bool {
        self.sign.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.sign.is_positive()
    }
}

impl<Digit: Clone + Eq + PartialOrd + Zeroable, const SEPARATOR: char, const SHIFT: usize> Ord
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

impl<Digit: Clone + PartialOrd + Zeroable, const SEPARATOR: char, const SHIFT: usize> PartialOrd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn ge(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && !{
                    if self.is_positive() {
                        digits_lesser_than(&self.digits, &other.digits)
                    } else {
                        digits_lesser_than(&other.digits, &self.digits)
                    }
                }
    }

    fn gt(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && if self.is_positive() {
                    digits_lesser_than(&other.digits, &self.digits)
                } else {
                    digits_lesser_than(&self.digits, &other.digits)
                }
    }

    fn le(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && !{
                    if self.is_positive() {
                        digits_lesser_than(&other.digits, &self.digits)
                    } else {
                        digits_lesser_than(&self.digits, &other.digits)
                    }
                }
    }

    fn lt(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && if self.is_positive() {
                    digits_lesser_than(&self.digits, &other.digits)
                } else {
                    digits_lesser_than(&other.digits, &self.digits)
                }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl<Digit: ExponentiativeDigit, const SEPARATOR: char, const SHIFT: usize> Pow<Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn pow(self, exponent: Self) -> Self::Output {
        self.checked_pow(exponent)
            .unwrap_or_else(|| panic!("Exponent should be non-negative."))
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Rem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)
                .unwrap();
        Self { sign, digits }
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> RemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
            &self.digits,
            self.sign,
            &divisor.digits,
            divisor.sign,
        )
        .unwrap();
        Self { sign, digits }
    }
}

impl<Digit: LeftShiftableDigit, const SEPARATOR: char, const SHIFT: usize> CheckedShl
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<Self, ShiftError>;

    fn checked_shl(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShiftError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            let (shift_quotient_digits, shift_remainder) =
                div_rem_digits_by_digit::<Digit, SHIFT>(&shift.digits, unsafe {
                    Digit::try_from(SHIFT).unwrap_unchecked()
                });
            let shift_quotient =
                checked_reduce_digits::<Digit, usize, SHIFT>(&shift_quotient_digits)
                    .ok_or(ShiftError::TooLarge)?;
            if shift_quotient >= usize::MAX / size_of::<Digit>() {
                Err(ShiftError::TooLarge)
            } else {
                let digits = left_shift_digits::<Digit, SHIFT>(
                    &self.digits,
                    shift_quotient,
                    shift_remainder,
                )
                .ok_or(ShiftError::OutOfMemory)?;
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
    type Output = Result<Self, ShiftError>;

    fn checked_shr(self, shift: Self) -> Self::Output {
        if shift.is_negative() {
            Err(ShiftError::NegativeShift)
        } else if self.is_zero() {
            Ok(self)
        } else {
            let (shift_quotient_digits, shift_remainder) =
                div_rem_digits_by_digit::<Digit, SHIFT>(&shift.digits, unsafe {
                    Digit::try_from(SHIFT).unwrap_unchecked()
                });
            let shift_quotient =
                checked_reduce_digits::<Digit, usize, SHIFT>(&shift_quotient_digits)
                    .ok_or(ShiftError::TooLarge)?;
            if shift_quotient >= usize::MAX / size_of::<Digit>() {
                Err(ShiftError::TooLarge)
            } else if self.is_negative() {
                let inverted = !self;
                let digits = right_shift_digits::<Digit, SHIFT>(
                    &inverted.digits,
                    shift_remainder,
                    shift_quotient,
                );
                Ok(!Self {
                    sign: inverted.sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
                    digits,
                })
            } else {
                let digits = right_shift_digits::<Digit, SHIFT>(
                    &self.digits,
                    shift_remainder,
                    shift_quotient,
                );
                Ok(Self {
                    sign: self.sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
                    digits,
                })
            }
        }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Sub
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = subtract_signed_digits::<Digit, SHIFT>(
            &self.digits,
            self.sign,
            &subtrahend.digits,
            subtrahend.sign,
        );
        Self { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> SubAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.sign, self.digits) = subtract_signed_digits::<Digit, SHIFT>(
            &self.digits,
            self.sign,
            &subtrahend.digits,
            subtrahend.sign,
        );
    }
}

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize> TryFrom<&str>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = ParsingError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::new(string, 0)
    }
}

impl<Digit: BinaryDigitConvertibleToFloat<f32>, const SEPARATOR: char, const SHIFT: usize>
    TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for f32
{
    type Error = ConversionError;

    fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
        match fraction_exponent_digits::<Digit, f32, SHIFT>(&value.digits) {
            Some((fraction_modulus, exponent)) => Ok(utils::load_exponent(
                (value.sign as f32) * fraction_modulus,
                exponent,
            )),
            None => Err(ConversionError::TooLarge),
        }
    }
}

impl<Digit: BinaryDigitConvertibleToFloat<f64>, const SEPARATOR: char, const SHIFT: usize>
    TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for f64
{
    type Error = ConversionError;

    fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
        match fraction_exponent_digits::<Digit, f64, SHIFT>(&value.digits) {
            Some((fraction_modulus, exponent)) => Ok(utils::load_exponent(
                (value.sign as f64) * fraction_modulus,
                exponent,
            )),
            None => Err(ConversionError::TooLarge),
        }
    }
}

impl<Digit: Unitary + Zeroable, const SEPARATOR: char, const SHIFT: usize> Unitary
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn one() -> Self {
        Self {
            sign: Sign::one(),
            digits: vec![Digit::one()],
        }
    }

    fn is_one(&self) -> bool {
        self.is_positive() && self.digits.len() == 1 && self.digits[0].is_one()
    }
}

impl<Digit: Zeroable, const SEPARATOR: char, const SHIFT: usize> Zeroable
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn zero() -> Self {
        Self {
            sign: Sign::zero(),
            digits: vec![Digit::zero()],
        }
    }

    fn is_zero(&self) -> bool {
        self.sign.is_zero()
    }
}
