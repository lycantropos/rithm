use std::cmp::Ordering;
use std::convert::{FloatToInt, TryFrom};
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::mem::size_of;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, Sub, SubAssign,
};
use std::str::Chars;

use crate::digits::*;
use crate::traits::{
    Abs, AssigningShiftingLeftMonoid, BitLength, BitwiseNegatableUnaryAlgebra, CheckedDiv,
    CheckedDivAsF32, CheckedDivAsF64, CheckedDivEuclid, CheckedDivRem, CheckedDivRemEuclid,
    CheckedPow, CheckedPowRemEuclid, CheckedRem, CheckedRemEuclid, CheckedRemEuclidInv, CheckedShl,
    CheckedShr, DivEuclid, DivRem, DivRemEuclid, Endianness, Float, FromBytes, FromStrRadix, Gcd,
    Oppose, OppositionOf, Oppositive, Parity, Pow, RemEuclid, ToBytes, Unitary, Zeroable,
};
use crate::utils;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigInt<Digit, const SEPARATOR: char, const SHIFT: usize> {
    sign: Sign,
    digits: Vec<Digit>,
}

pub(crate) const MIN_REPRESENTABLE_BASE: u8 = 2;
pub(crate) const MAX_REPRESENTABLE_BASE: u8 = 36;

#[derive(Eq, PartialEq)]
pub enum FromFloatConversionError {
    Infinity,
    NaN,
}

impl FromFloatConversionError {
    fn description(&self) -> &str {
        match self {
            FromFloatConversionError::Infinity => "Conversion of infinity is undefined.",
            FromFloatConversionError::NaN => "Conversion of NaN is undefined.",
        }
    }
}

impl Debug for FromFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for FromFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Eq, PartialEq)]
pub enum FromStringConversionError {
    BaseOutOfBounds(u32),
    ConsecutiveSeparators,
    EndsWithSeparator,
    InvalidDigit(char, u8),
    StartsWithSeparator,
}

impl FromStringConversionError {
    fn description(&self) -> String {
        match self {
            FromStringConversionError::BaseOutOfBounds(base) => {
                format!(
                    "Base should be zero or in range from {MIN_REPRESENTABLE_BASE} \
                     to {MAX_REPRESENTABLE_BASE}, but found: {}.",
                    base
                )
            }
            FromStringConversionError::ConsecutiveSeparators => {
                String::from("Consecutive separators found.")
            }
            FromStringConversionError::EndsWithSeparator => {
                String::from("Should not end with separator.")
            }
            FromStringConversionError::InvalidDigit(character, base) => {
                format!("Invalid digit in base {}: {:?}.", base, character)
            }
            FromStringConversionError::StartsWithSeparator => {
                String::from("Should not start with separator.")
            }
        }
    }
}

impl Debug for FromStringConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for FromStringConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
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

#[derive(Eq, PartialEq)]
pub enum ToFloatConversionError {
    TooLarge,
}

impl ToFloatConversionError {
    fn description(&self) -> &str {
        match self {
            ToFloatConversionError::TooLarge => "Too large to convert to floating point.",
        }
    }
}

impl Debug for ToFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for ToFloatConversionError {
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

    fn new(string: &str, mut base: u8) -> Result<Self, FromStringConversionError> {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        debug_assert!(Self::ASCII_CODES_DIGIT_VALUES[SEPARATOR as usize] >= MAX_REPRESENTABLE_BASE);
        debug_assert!(
            base == 0 || (base >= MIN_REPRESENTABLE_BASE && base <= MAX_REPRESENTABLE_BASE)
        );
        let mut characters = string.trim().chars().peekable();
        let sign = Self::parse_sign(&mut characters);
        if base == 0 {
            base = Self::guess_base(&mut characters);
        };
        Self::skip_prefix(&mut characters, base);
        Self::parse_digits(characters, base).map(|digits| {
            let digits = digits_to_binary_base::<u8, Digit, SHIFT>(&digits, base as usize);
            Self {
                sign: sign * to_digits_sign(&digits),
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
    fn parse_digits(
        mut characters: Peekable<Chars>,
        base: u8,
    ) -> Result<Vec<u8>, FromStringConversionError> {
        if characters.peek() == Some(&SEPARATOR) {
            return Err(FromStringConversionError::StartsWithSeparator);
        }
        let mut result: Vec<u8> = Vec::new();
        let mut prev: char = SEPARATOR;
        for character in characters {
            if character != SEPARATOR {
                let digit = Self::ASCII_CODES_DIGIT_VALUES[character as usize];
                if digit >= base {
                    return Err(FromStringConversionError::InvalidDigit(character, base));
                }
                result.push(digit);
            } else if prev == SEPARATOR {
                return Err(FromStringConversionError::ConsecutiveSeparators);
            }
            prev = character;
        }
        if prev == SEPARATOR {
            return Err(FromStringConversionError::EndsWithSeparator);
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
        Digit: BinaryDigitConvertibleToBinary<Digit> + From<u8> + Oppose,
        const SEPARATOR: char,
        const SHIFT: usize,
    > ToBytes for &BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: TryFrom<Digit>,
{
    type Output = Vec<u8>;

    fn to_bytes(self, endianness: Endianness) -> Self::Output {
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
        match endianness {
            Endianness::Big => result.reverse(),
            Endianness::Little => {}
        }
        result
    }
}

impl<Digit: Oppose, const SEPARATOR: char, const SHIFT: usize> FromBytes
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    u8: BinaryDigitConvertibleToBinary<Digit>,
{
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        let mut bytes = bytes.to_vec();
        match endianness {
            Endianness::Big => bytes.reverse(),
            Endianness::Little => {}
        }
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        let most_significant_byte = bytes[bytes.len() - 1];
        let sign = if most_significant_byte >= MIDDLE_BYTE {
            negate_digits(&mut bytes);
            -Sign::one()
        } else {
            to_digits_sign(&bytes)
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

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const SEPARATOR: char, const SHIFT: usize> Abs
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits.clone(),
        }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize>
    Add<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn add(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Add
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: &Self) {
        (self.sign, self.digits) =
            sum_signed_digits::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAnd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits)
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAnd<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitand(self, other: &Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits.clone())
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits.clone(), self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize>
    BitAnd<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitand(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAnd
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitand(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_and::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAndAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_and::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitAndAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_and::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_and::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
    }
}

impl<
        Digit: BitLength<Output = usize> + BinaryDigit + MultiplicativeDigit + Oppose + TryFrom<usize>,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitLength for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bit_length(self) -> Self::Output {
        if self.digits.len() <= usize::MAX / SHIFT {
            Self::from(
                (self.digits.len() - 1) * SHIFT + self.digits[self.digits.len() - 1].bit_length(),
            )
        } else {
            Self::from(self.digits.len() - 1) * Self::from(SHIFT)
                + Self::from(self.digits[self.digits.len() - 1].bit_length())
        }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOr
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits)
        } else {
            bitwise_or::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOr<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitor(self, other: &Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits.clone())
        } else {
            bitwise_or::<Digit, SHIFT>(other.sign, other.digits.clone(), self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize>
    BitOr<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitor(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_or::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOr
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_or::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOrAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_or::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitOrAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_or::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_or::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXor
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits)
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXor<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitxor(self, other: &Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits.clone())
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits.clone(), self.sign, self.digits)
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize>
    BitXor<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitxor(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXor
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_xor::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
        Self::Output { sign, digits }
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXorAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(self.sign, self.digits.clone(), other.sign, other.digits)
        } else {
            bitwise_xor::<Digit, SHIFT>(other.sign, other.digits, self.sign, self.digits.clone())
        };
    }
}

impl<Digit: BinaryDigit, const SEPARATOR: char, const SHIFT: usize> BitXorAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = if self.digits.len() > other.digits.len() {
            bitwise_xor::<Digit, SHIFT>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            )
        } else {
            bitwise_xor::<Digit, SHIFT>(
                other.sign,
                other.digits.clone(),
                self.sign,
                self.digits.clone(),
            )
        };
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: &Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        checked_div::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: &Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        checked_div_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        checked_div_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits).map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRem<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: &Self) -> Self::Output {
        checked_div_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits).map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRem<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits).map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRem
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        checked_div_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits).map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: &Self) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
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

impl<
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, &Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: &Self) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<&Self, Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow_rem_euclid(self, exponent: &Self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<&Self, &Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_pow_rem_euclid(self, exponent: &Self, divisor: &Self) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>, BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow_rem_euclid(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>, Self>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow_rem_euclid(
        self,
        exponent: BigInt<Digit, SEPARATOR, SHIFT>,
        divisor: Self,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow_rem_euclid(
        self,
        exponent: Self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
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
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedPowRemEuclid<Self, Self> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_pow_rem_euclid(self, exponent: Self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
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

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: &Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_rem_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclid<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: &Self) -> Self::Output {
        checked_rem_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_rem_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_rem_euclid::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclidInv
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid_inv(&divisor)
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRemEuclidInv<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid_inv(self, divisor: &Self) -> Self::Output {
        let mut candidate = Self::zero();
        let mut result = Self::one();
        let mut step_dividend = self;
        let mut step_divisor = divisor.clone();
        while !step_divisor.is_zero() {
            let (quotient, remainder) = unsafe {
                step_dividend
                    .checked_div_rem_euclid(&step_divisor)
                    .unwrap_unchecked()
            };
            step_dividend = step_divisor;
            step_divisor = remainder;
            (candidate, result) = (result - quotient * &candidate, candidate);
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

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRemEuclidInv<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid_inv(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.clone().checked_rem_euclid_inv(&divisor)
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclidInv
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
        self.clone().checked_rem_euclid_inv(divisor)
    }
}

impl<Digit: DisplayableDigit, const SEPARATOR: char, const SHIFT: usize> Display
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
            self.sign,
            self.digits.as_slice(),
            divisor.sign,
            divisor.digits.as_slice(),
        )
        .unwrap();
        Self::Output { sign, digits }
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
            self.sign,
            self.digits.as_slice(),
            divisor.sign,
            divisor.digits.as_slice(),
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
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .unwrap();
        Self::Output { sign, digits }
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

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize>
    FloatToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f32
{
    unsafe fn to_int_unchecked(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize>
    FloatToInt<BigInt<Digit, SEPARATOR, SHIFT>> for f64
{
    unsafe fn to_int_unchecked(self) -> BigInt<Digit, SEPARATOR, SHIFT> {
        BigInt::<Digit, SEPARATOR, SHIFT>::try_from(self).unwrap_unchecked()
    }
}

impl<Source, Digit, const SEPARATOR: char, const SHIFT: usize> From<Source>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Source: BinaryDigit + Oppose + TryFrom<OppositionOf<Source>>,
    Digit: BinaryDigit + Oppose + TryFrom<Source>,
    OppositionOf<Source>: TryFrom<Source>,
{
    fn from(value: Source) -> Self {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        if value.is_zero() {
            Self::zero()
        } else {
            Self {
                sign: non_zero_value_to_sign(value),
                digits: non_zero_value_to_digits::<Source, Digit, SHIFT>(value),
            }
        }
    }
}

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize> FromStrRadix
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = FromStringConversionError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error> {
        if radix != 0
            && (radix < (MIN_REPRESENTABLE_BASE as u32) || radix > (MAX_REPRESENTABLE_BASE as u32))
        {
            Err(FromStringConversionError::BaseOutOfBounds(radix))
        } else {
            Self::new(string, radix as u8)
        }
    }
}

impl<Digit: GcdDigit, const SEPARATOR: char, const SHIFT: usize> Gcd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) = to_gcd::<Digit, SHIFT>(self.digits, other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: GcdDigit, const SEPARATOR: char, const SHIFT: usize> Gcd<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn gcd(self, other: &Self) -> Self::Output {
        let (sign, digits) = to_gcd::<Digit, SHIFT>(self.digits, other.digits.clone());
        Self::Output { sign, digits }
    }
}

impl<Digit: GcdDigit, const SEPARATOR: char, const SHIFT: usize>
    Gcd<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn gcd(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = to_gcd::<Digit, SHIFT>(self.digits.clone(), other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: GcdDigit, const SEPARATOR: char, const SHIFT: usize> Gcd
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) = to_gcd::<Digit, SHIFT>(self.digits.clone(), other.digits.clone());
        Self::Output { sign, digits }
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

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> Mul<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Mul<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn mul(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> Mul
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

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

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> MulAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn mul_assign(&mut self, other: &Self) {
        self.sign *= other.sign;
        self.digits = multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Neg for BigInt<Digit, SEPARATOR, SHIFT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            sign: -self.sign,
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const SEPARATOR: char, const SHIFT: usize> Neg
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn neg(self) -> Self::Output {
        Self::Output {
            sign: -self.sign,
            digits: self.digits.clone(),
        }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Not
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let (sign, digits) = invert_digits::<Digit, SHIFT>(self.sign, &self.digits);
        Self { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Not
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn not(self) -> Self::Output {
        let (sign, digits) = invert_digits::<Digit, SHIFT>(self.sign, &self.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: OppositiveDigit, const SEPARATOR: char, const SHIFT: usize> Oppositive
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_negative(&self) -> bool {
        self.sign.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.sign.is_positive()
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

macro_rules! plain_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<Digit: PartialEq, const SEPARATOR: char, const SHIFT: usize>
            PartialEq<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        where
            Digit: BinaryDigit + Oppose + TryFrom<$t>,
        {
            fn eq(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) == other.sign
                    && (self.is_zero()
                        || non_zero_value_to_digits::<$t, Digit, SHIFT>(*self) == other.digits)
            }
        }

        impl<Digit: PartialEq, const SEPARATOR: char, const SHIFT: usize> PartialEq<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Digit: BinaryDigit + Oppose + TryFrom<$t>,
        {
            fn eq(&self, other: &$t) -> bool {
                self.sign == value_to_sign(*other)
                    && (self.is_zero()
                        || self.digits == non_zero_value_to_digits::<$t, Digit, SHIFT>(*other))
            }
        }
    )*)
}

plain_partial_eq_to_big_int_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl<Digit: Clone + PartialOrd + ZeroableDigit, const SEPARATOR: char, const SHIFT: usize>
    PartialOrd for BigInt<Digit, SEPARATOR, SHIFT>
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

macro_rules! big_int_partial_eq_to_signed_primitive_impl {
    ($($t:ty)*) => ($(
        impl<Digit: PartialEq + OppositiveDigit, const SEPARATOR: char, const SHIFT: usize>
            PartialOrd<$t> for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Digit: BinaryDigit + Oppose + TryFrom<$t>,
        {
            fn ge(&self, other: &$t) -> bool {
                self.sign > ((*other).signum() as Sign)
                    || self.sign == ((*other).signum() as Sign)
                        && (other.is_zero()
                            || !{
                                if self.is_positive() {
                                    digits_lesser_than(
                                        &self.digits,
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                    )
                                } else {
                                    digits_lesser_than(
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                        &self.digits,
                                    )
                                }
                            })
            }

            fn gt(&self, other: &$t) -> bool {
                self.sign > ((*other).signum() as Sign)
                    || !other.is_zero()
                        && self.sign == ((*other).signum() as Sign)
                        && if self.is_positive() {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                &self.digits,
                            )
                        } else {
                            digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            )
                        }
            }

            fn le(&self, other: &$t) -> bool {
                self.sign < ((*other).signum() as Sign)
                    || self.sign == ((*other).signum() as Sign)
                        && (other.is_zero()
                            || !{
                                if self.is_positive() {
                                    digits_lesser_than(
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                        &self.digits,
                                    )
                                } else {
                                    digits_lesser_than(
                                        &self.digits,
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                    )
                                }
                            })
            }

            fn lt(&self, other: &$t) -> bool {
                self.sign < ((*other).signum() as Sign)
                    || !other.is_zero()
                        && self.sign == ((*other).signum() as Sign)
                        && if self.is_positive() {
                            digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            )
                        } else {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                &self.digits,
                            )
                        }
            }

            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                Some(if self.lt(other) {
                    Ordering::Less
                } else if self.gt(other) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

big_int_partial_eq_to_signed_primitive_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! big_int_partial_eq_to_unsigned_primitive_impl {
    ($($t:ty)*) => ($(
        impl<Digit: PartialEq + OppositiveDigit, const SEPARATOR: char, const SHIFT: usize>
            PartialOrd<$t> for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Digit: BinaryDigit + Oppose + TryFrom<$t>,
        {
            fn ge(&self, other: &$t) -> bool {
                self.is_zero() && other.is_zero()
                    || self.is_positive()
                        && (other.is_zero()
                            || !digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            ))
            }

            fn gt(&self, other: &$t) -> bool {
                self.is_positive()
                    && (other.is_zero()
                        || digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            &self.digits,
                        ))
            }

            fn le(&self, other: &$t) -> bool {
                !self.is_positive()
                    || !other.is_zero()
                        && digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            &self.digits,
                        )
            }

            fn lt(&self, other: &$t) -> bool {
                self.is_negative()
                    || !other.is_zero()
                        && (self.is_zero()
                            || digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            ))
            }

            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                Some(if self.lt(other) {
                    Ordering::Less
                } else if self.gt(other) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

big_int_partial_eq_to_unsigned_primitive_impl!(u8 u16 u32 u64 u128 usize);

macro_rules! signed_primitive_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<Digit: PartialEq + OppositiveDigit, const SEPARATOR: char, const SHIFT: usize>
            PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        where
            Digit: BinaryDigit + Oppose + TryFrom<$t>,
        {
            fn le(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) < other.sign
                    || value_to_sign(*self) == other.sign
                        && !{
                            if self.is_positive() {
                                digits_lesser_than(
                                    &other.digits,
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                )
                            } else {
                                digits_lesser_than(
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                    &other.digits,
                                )
                            }
                        }
            }

            fn lt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) < other.sign
                    || value_to_sign(*self) == other.sign
                        && if self.is_positive() {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                &other.digits,
                            )
                        } else {
                            digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            )
                        }
            }

            fn ge(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) > other.sign
                    || value_to_sign(*self) == other.sign
                        && !{
                            if self.is_positive() {
                                digits_lesser_than(
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                    &other.digits,
                                )
                            } else {
                                digits_lesser_than(
                                    &other.digits,
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                )
                            }
                        }
            }

            fn gt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) > other.sign
                    || value_to_sign(*self) == other.sign
                        && if self.is_positive() {
                            digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            )
                        } else {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                &other.digits,
                            )
                        }
            }

            fn partial_cmp(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Option<Ordering> {
                Some(if self.lt(other) {
                    Ordering::Less
                } else if self.gt(other) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

signed_primitive_partial_eq_to_big_int_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_primitive_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<Digit: PartialEq + OppositiveDigit, const SEPARATOR: char, const SHIFT: usize>
            PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        where
            Digit: BinaryDigit + Oppose + TryFrom<$t>,
        {
            fn ge(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                !other.is_positive()
                    || !self.is_zero()
                        && digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            &other.digits,
                        )
            }

            fn gt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                other.is_negative()
                    || !self.is_zero()
                        && (other.is_zero()
                            || digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            ))
            }

            fn le(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                self.is_zero() && other.is_zero()
                    || other.is_positive()
                        && (self.is_zero()
                            || !digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            ))
            }

            fn lt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                other.is_positive()
                    && (self.is_zero()
                        || digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            &other.digits,
                        ))
            }

            fn partial_cmp(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Option<Ordering> {
                Some(if other.lt(self) {
                    Ordering::Less
                } else if other.gt(self) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

unsigned_primitive_partial_eq_to_big_int_impl!(u8 u16 u32 u64 u128 usize);

impl<Digit: ParitiableDigit, const SEPARATOR: char, const SHIFT: usize> Parity
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_even(&self) -> bool {
        (self.digits[0] & Digit::one()).is_zero()
    }

    fn is_odd(&self) -> bool {
        !(self.digits[0] & Digit::one()).is_zero()
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

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Sub
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = subtract_signed_digits::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Sub<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn sub(self, subtrahend: &Self) -> Self::Output {
        let (sign, digits) = subtract_signed_digits::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize>
    Sub<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn sub(self, subtrahend: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = subtract_signed_digits::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> Sub
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = subtract_signed_digits::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> SubAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.sign, self.digits) = subtract_signed_digits::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> SubAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: &Self) {
        (self.sign, self.digits) = subtract_signed_digits::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize> TryFrom<f64>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = FromFloatConversionError;

    fn try_from(mut value: f64) -> Result<Self, Self::Error> {
        debug_assert!(usize::BITS < i32::BITS || SHIFT < (i32::MAX as usize));
        if value.is_infinite() {
            Err(FromFloatConversionError::Infinity)
        } else if value.is_nan() {
            Err(FromFloatConversionError::NaN)
        } else if value.abs() < f64::one() {
            Ok(Self::zero())
        } else {
            let mut sign = Sign::one();
            if value.is_sign_negative() {
                sign = -sign;
                value = -value;
            }
            Ok(Self {
                sign,
                digits: digits_from_finite_positive_improper_float::<Digit, f64, SHIFT>(value),
            })
        }
    }
}

impl<Digit: DigitConvertibleFromFloat, const SEPARATOR: char, const SHIFT: usize> TryFrom<f32>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = FromFloatConversionError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

impl<Digit: FromStrDigit, const SEPARATOR: char, const SHIFT: usize> TryFrom<&str>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Error = FromStringConversionError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::new(string, 0)
    }
}

impl<Digit: BinaryDigitConvertibleToFloat<f32>, const SEPARATOR: char, const SHIFT: usize>
    TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for f32
{
    type Error = ToFloatConversionError;

    fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
        match fraction_exponent_digits::<Digit, f32, SHIFT>(&value.digits) {
            Some((fraction_modulus, exponent)) => {
                Ok(((value.sign as f32) * fraction_modulus).ldexp(exponent))
            }
            None => Err(ToFloatConversionError::TooLarge),
        }
    }
}

impl<Digit: BinaryDigitConvertibleToFloat<f64>, const SEPARATOR: char, const SHIFT: usize>
    TryFrom<BigInt<Digit, SEPARATOR, SHIFT>> for f64
{
    type Error = ToFloatConversionError;

    fn try_from(value: BigInt<Digit, SEPARATOR, SHIFT>) -> Result<Self, Self::Error> {
        match fraction_exponent_digits::<Digit, f64, SHIFT>(&value.digits) {
            Some((fraction_modulus, exponent)) => {
                Ok(((value.sign as f64) * fraction_modulus).ldexp(exponent))
            }
            None => Err(ToFloatConversionError::TooLarge),
        }
    }
}

impl<Digit: UnitaryDigit, const SEPARATOR: char, const SHIFT: usize> Unitary
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn one() -> Self {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        Self {
            sign: Sign::one(),
            digits: vec![Digit::one()],
        }
    }

    fn is_one(&self) -> bool {
        self.is_positive() && self.digits.len() == 1 && self.digits[0].is_one()
    }
}

impl<Digit: ZeroableDigit, const SEPARATOR: char, const SHIFT: usize> Zeroable
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn zero() -> Self {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        Self {
            sign: Sign::zero(),
            digits: vec![Digit::zero()],
        }
    }

    fn is_zero(&self) -> bool {
        self.sign.is_zero()
    }
}

const fn is_valid_shift<Digit: Oppose, const SHIFT: usize>() -> bool {
    const BITS_IN_BYTE: usize = 8;
    SHIFT < BITS_IN_BYTE * size_of::<Digit>() - (utils::is_signed::<Digit>() as usize)
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

impl<
        Digit: ExponentiativeDigit + EuclidDivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BigInt<Digit, SEPARATOR, SHIFT>
{
    fn checked_pow_abs_rem_euclid(self, exponent: &Self, divisor: &Self) -> Option<Self> {
        debug_assert!(divisor.is_positive());
        if divisor.is_one() {
            return Some(Self::zero());
        }
        let base = if exponent.is_negative() {
            self.checked_rem_euclid_inv(divisor)?
        } else {
            self
        };
        let mut exponent_digit = exponent.digits[exponent.digits.len() - 1];
        Some(
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
