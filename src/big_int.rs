use core::fmt;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::f64;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::ops::{Add, Mul, Neg, Sub};
use std::str::Chars;

use num::traits::WrappingSub;
use num::{One, PrimInt, Zero};

use crate::utils;

#[derive(Clone, PartialEq, Eq, Ord)]
pub struct BigInt<Digit, const SHIFT: usize> {
    sign: Sign,
    digits: Vec<Digit>,
}

#[inline]
fn digits_lesser_than<Digit: PartialOrd>(left: &Vec<Digit>, right: &Vec<Digit>) -> bool {
    left.len() < right.len()
        || left.len() == right.len() && left.iter().rev().lt(right.iter().rev())
}

impl<Digit: PartialOrd, const SHIFT: usize> PartialOrd for BigInt<Digit, SHIFT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if other.lt(self) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }

    fn lt(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && if self.sign > 0 {
                    digits_lesser_than(&self.digits, &other.digits)
                } else {
                    digits_lesser_than(&other.digits, &self.digits)
                }
    }

    fn le(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && !{
                    if self.sign > 0 {
                        digits_lesser_than(&other.digits, &self.digits)
                    } else {
                        digits_lesser_than(&self.digits, &other.digits)
                    }
                }
    }

    fn gt(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && if self.sign > 0 {
                    digits_lesser_than(&other.digits, &self.digits)
                } else {
                    digits_lesser_than(&self.digits, &other.digits)
                }
    }

    fn ge(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && !{
                    if self.sign > 0 {
                        digits_lesser_than(&self.digits, &other.digits)
                    } else {
                        digits_lesser_than(&other.digits, &self.digits)
                    }
                }
    }
}

const MAX_REPRESENTABLE_BASE: u8 = 36;

impl<Digit, const SHIFT: usize> BigInt<Digit, SHIFT>
where
    u8: DoublePrecision + PrimInt,
    Digit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<u8>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<DoublePrecisionOf<Digit>>
        + Zero,
    DoublePrecisionOf<u8>: From<u8> + PrimInt,
    DoublePrecisionOf<Digit>: From<u8> + From<Digit> + PrimInt + TryFrom<usize>,
{
    const SEPARATOR: char = '_';
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

    pub(crate) fn new(_string: &str, mut base: u8) -> Result<Self, String> {
        if (base != 0 && base < 2) || base > MAX_REPRESENTABLE_BASE {
            return Err(format!(
                "Base should be zero or in range from 2 to {}.",
                MAX_REPRESENTABLE_BASE
            ));
        }
        let mut characters = _string.trim().chars().peekable();
        let sign: Sign = if characters.peek() == Some(&'-') {
            characters.next();
            -1
        } else if characters.peek() == Some(&'+') {
            characters.next();
            1
        } else {
            1
        };
        if base == 0 {
            base = if characters.peek() != Some(&'0') {
                10
            } else {
                match characters.clone().nth(1) {
                    Some('b') | Some('B') => 2,
                    Some('o') | Some('O') => 8,
                    Some('x') | Some('X') => 16,
                    _ => 10,
                }
            };
        };
        if characters.peek() == Some(&'0') {
            match characters.clone().nth(1) {
                Some('b') | Some('B') => {
                    if base == 2 {
                        characters.nth(1);
                        characters.next_if_eq(&Self::SEPARATOR);
                    }
                }
                Some('o') | Some('O') => {
                    if base == 8 {
                        characters.nth(1);
                        characters.next_if_eq(&Self::SEPARATOR);
                    }
                }
                Some('x') | Some('X') => {
                    if base == 16 {
                        characters.nth(1);
                        characters.next_if_eq(&Self::SEPARATOR);
                    }
                }
                _ => {}
            };
        };
        let digits = digits_to_binary_base::<u8, Digit>(
            &Self::parse_digits(characters, base)?,
            base as usize,
            SHIFT,
        );
        Ok(Self {
            sign: sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
            digits,
        })
    }

    fn parse_digits(mut characters: Peekable<Chars>, base: u8) -> Result<Vec<u8>, String> {
        if characters.peek() == Some(&Self::SEPARATOR) {
            return Err(String::from("Should not start with separator."));
        }
        let mut result: Vec<u8> = Vec::new();
        let mut prev: char = Self::SEPARATOR;
        while let Some(character) = characters.next() {
            if character != Self::SEPARATOR {
                let digit = Self::ASCII_CODES_DIGIT_VALUES[character as usize];
                if digit >= base {
                    return Err(format!("Invalid digit in base {}: {}.", base, character));
                }
                result.push(digit);
            } else if prev == Self::SEPARATOR {
                return Err(String::from("Consecutive separators found."));
            }
            prev = character;
        }
        if prev == Self::SEPARATOR {
            return Err(String::from("Should not end with separator."));
        }
        result.reverse();
        Ok(result)
    }
}

impl<Digit, const SHIFT: usize> Display for BigInt<Digit, SHIFT>
where
    Digit:
        DoublePrecision + From<u8> + PrimInt + TryFrom<DoublePrecisionOf<Digit>> + TryFrom<usize>,
    DoublePrecisionOf<Digit>: From<Digit> + PrimInt + TryFrom<usize>,
    <DoublePrecisionOf<Digit> as TryFrom<usize>>::Error: fmt::Debug,
    <Digit as TryFrom<DoublePrecisionOf<Digit>>>::Error: fmt::Debug,
    usize: TryFrom<Digit>,
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(self.to_base_string(10).as_str())
    }
}

impl<Digit, const SHIFT: usize> BigInt<Digit, SHIFT>
where
    Digit:
        DoublePrecision + From<u8> + PrimInt + TryFrom<DoublePrecisionOf<Digit>> + TryFrom<usize>,
    DoublePrecisionOf<Digit>: From<Digit> + PrimInt + TryFrom<usize>,
    <DoublePrecisionOf<Digit> as TryFrom<usize>>::Error: fmt::Debug,
    <Digit as TryFrom<DoublePrecisionOf<Digit>>>::Error: fmt::Debug,
    usize: TryFrom<Digit>,
{
    const DIGIT_VALUES_ASCII_CODES: [char; MAX_REPRESENTABLE_BASE as usize] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    pub(crate) fn to_base_string(&self, base: usize) -> String {
        let shift = utils::floor_log(1 << SHIFT, base).unwrap();
        let digits: Vec<Digit> =
            binary_digits_to_base::<Digit, Digit>(&self.digits, SHIFT, utils::power(base, shift));
        let characters_count = ((self.sign < 0) as usize)
            + (digits.len() - 1) * shift
            + utils::floor_log(
                unsafe { usize::try_from(*digits.last().unwrap()).unwrap_unchecked() },
                base,
            )
            .unwrap_or(0usize)
            + 1;
        let mut characters: String = String::with_capacity(characters_count);
        let target_base = unsafe { Digit::try_from(base).unwrap_unchecked() };
        for index in 0..digits.len() - 1 {
            let mut remainder = digits[index];
            for _ in 0..shift {
                characters.push(
                    Self::DIGIT_VALUES_ASCII_CODES
                        [unsafe { usize::try_from(remainder % target_base).unwrap_unchecked() }],
                );
                remainder = remainder / target_base;
            }
        }
        let mut remainder = *digits.last().unwrap();
        while !remainder.is_zero() {
            characters.push(
                Self::DIGIT_VALUES_ASCII_CODES
                    [unsafe { usize::try_from(remainder % target_base).unwrap_unchecked() }],
            );
            remainder = remainder / target_base;
        }
        if self.sign == 0 {
            characters.push('0');
        } else if self.sign < 0 {
            characters.push('-');
        }
        characters.chars().rev().collect()
    }
}

#[cfg(target_arch = "x86")]
const HASH_BITS: usize = 31;
#[cfg(not(target_arch = "x86"))]
const HASH_BITS: usize = 61;
const HASH_MODULUS: usize = (1 << HASH_BITS) - 1;

impl<Digit, const SHIFT: usize> BigInt<Digit, SHIFT> {
    pub fn abs(self) -> Self {
        Self {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit, const SHIFT: usize> BigInt<Digit, SHIFT>
where
    Digit: From<bool> + PrimInt,
    usize: TryFrom<Digit>,
{
    pub(crate) fn hash(&self) -> usize {
        if self.digits.len() == 1 {
            return if self.sign < 0 {
                usize::MAX
                    - unsafe {
                        usize::try_from(
                            self.digits[0] + <Digit as From<bool>>::from(self.digits[0].is_one()),
                        )
                        .unwrap_unchecked()
                    }
                    + 1
            } else {
                unsafe { usize::try_from(self.digits[0]).unwrap_unchecked() }
            };
        };
        let mut result = 0;
        for &position in self.digits.iter().rev() {
            result = ((result << SHIFT) & HASH_MODULUS) | (result >> (HASH_BITS - SHIFT));
            result += unsafe { usize::try_from(position).unwrap_unchecked() };
            if result >= HASH_MODULUS {
                result -= HASH_MODULUS;
            }
        }
        if self.sign < 0 {
            result = usize::MAX - result + 1
        };
        result - ((result == usize::MAX) as usize)
    }
}

impl<Digit, const SHIFT: usize> Add for BigInt<Digit, SHIFT>
where
    Digit: PrimInt + TryFrom<usize> + WrappingSub,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.sign < 0 {
            if other.sign < 0 {
                Self {
                    sign: -1,
                    digits: sum_digits::<Digit, SHIFT>(&self.digits, &other.digits),
                }
            } else {
                let mut sign: Sign = 1;
                let digits =
                    subtract_digits::<Digit, SHIFT>(&other.digits, &self.digits, &mut sign);
                Self { sign, digits }
            }
        } else if other.sign < 0 {
            let mut sign: Sign = 1;
            let digits = subtract_digits::<Digit, SHIFT>(&self.digits, &other.digits, &mut sign);
            Self { sign, digits }
        } else {
            Self {
                sign: self.sign | other.sign,
                digits: sum_digits::<Digit, SHIFT>(&self.digits, &other.digits),
            }
        }
    }
}

impl<Digit, const SHIFT: usize> TryFrom<&str> for BigInt<Digit, SHIFT>
where
    u8: DoublePrecision + PrimInt,
    Digit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<u8>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<DoublePrecisionOf<Digit>>
        + Zero,
    DoublePrecisionOf<u8>: From<u8> + PrimInt,
    DoublePrecisionOf<Digit>: From<u8> + From<Digit> + PrimInt + TryFrom<usize>,
{
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::new(string, 0)
    }
}

impl<Digit, const SHIFT: usize> Mul for BigInt<Digit, SHIFT>
where
    Digit: DoublePrecision
        + PrimInt
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + WrappingSub,
    DoublePrecisionOf<Digit>: From<Digit> + PrimInt,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit, const SHIFT: usize> Neg for BigInt<Digit, SHIFT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            sign: -self.sign,
            digits: self.digits,
        }
    }
}

impl<Digit, const SHIFT: usize> Sub for BigInt<Digit, SHIFT>
where
    Digit: PrimInt + TryFrom<usize> + WrappingSub,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        return if self.sign < 0 {
            if other.sign < 0 {
                let mut sign: Sign = 1;
                let digits =
                    subtract_digits::<Digit, SHIFT>(&other.digits, &self.digits, &mut sign);
                Self { sign, digits }
            } else {
                Self {
                    sign: -1,
                    digits: sum_digits::<Digit, SHIFT>(&self.digits, &other.digits),
                }
            }
        } else if other.sign < 0 {
            Self {
                sign: 1,
                digits: sum_digits::<Digit, SHIFT>(&self.digits, &other.digits),
            }
        } else {
            let mut sign: Sign = 1;
            let digits = subtract_digits::<Digit, SHIFT>(&self.digits, &other.digits, &mut sign);
            Self { sign, digits }
        };
    }
}

impl<Digit, const SHIFT: usize> Zero for BigInt<Digit, SHIFT>
where
    Digit: PrimInt + TryFrom<usize> + WrappingSub,
{
    fn zero() -> Self {
        Self {
            sign: 0,
            digits: vec![Digit::zero()],
        }
    }

    fn is_zero(&self) -> bool {
        self.sign.is_zero()
    }
}

pub trait DoublePrecision {
    type Type;
}

impl DoublePrecision for i8 {
    type Type = i16;
}

impl DoublePrecision for i16 {
    type Type = i32;
}

impl DoublePrecision for i32 {
    type Type = i64;
}

impl DoublePrecision for i64 {
    type Type = i128;
}

impl DoublePrecision for u8 {
    type Type = u16;
}

impl DoublePrecision for u16 {
    type Type = u32;
}

impl DoublePrecision for u32 {
    type Type = u64;
}

impl DoublePrecision for u64 {
    type Type = u128;
}

pub trait Signed {
    type Type;
}

impl Signed for u8 {
    type Type = i8;
}

impl Signed for u16 {
    type Type = i16;
}

impl Signed for u32 {
    type Type = i32;
}

impl Signed for u64 {
    type Type = i64;
}

impl Signed for u128 {
    type Type = i128;
}

impl Signed for i8 {
    type Type = i8;
}

impl Signed for i16 {
    type Type = i16;
}

impl Signed for i32 {
    type Type = i32;
}

impl Signed for i64 {
    type Type = i64;
}

impl Signed for i128 {
    type Type = i128;
}

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Type;
pub type SignedOf<T> = <T as Signed>::Type;
pub(crate) type Sign = i8;

pub(crate) fn binary_digits_to_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy + DoublePrecision + From<u8> + PrimInt,
    TargetDigit: Copy
        + DoublePrecision
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + Zero,
    DoublePrecisionOf<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecisionOf<TargetDigit>:
        From<SourceDigit> + From<TargetDigit> + PrimInt + TryFrom<usize>,
    <DoublePrecisionOf<TargetDigit> as TryFrom<usize>>::Error: fmt::Debug,
    <TargetDigit as TryFrom<DoublePrecisionOf<TargetDigit>>>::Error: fmt::Debug,
    usize: TryFrom<SourceDigit>,
{
    if target_base & (target_base - 1) == 0 {
        binary_digits_to_binary_base(
            source_digits,
            source_shift,
            utils::floor_log2::<usize>(target_base).unwrap(),
        )
    } else {
        binary_digits_to_non_binary_base(source_digits, source_shift, target_base)
    }
}

pub(crate) fn digits_to_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_base: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: DoublePrecision + From<u8> + PrimInt,
    TargetDigit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + Zero,
    DoublePrecisionOf<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecisionOf<TargetDigit>:
        From<SourceDigit> + From<TargetDigit> + PrimInt + TryFrom<usize>,
    usize: TryFrom<SourceDigit>,
{
    if source_base & (source_base - 1) == 0 {
        binary_digits_to_binary_base(
            source_digits,
            utils::floor_log2::<usize>(source_base).unwrap(),
            target_shift,
        )
    } else {
        non_binary_digits_to_binary_base(source_digits, source_base, target_shift)
    }
}

fn binary_digits_to_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: DoublePrecision + From<u8> + PrimInt,
    TargetDigit: DoublePrecision
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>,
    DoublePrecisionOf<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecisionOf<TargetDigit>: From<SourceDigit> + PrimInt,
    usize: TryFrom<SourceDigit>,
{
    match target_shift.cmp(&source_shift) {
        Ordering::Equal => source_digits
            .iter()
            .map(|&digit| unsafe { TargetDigit::try_from(digit).unwrap_unchecked() })
            .collect(),
        Ordering::Greater => binary_digits_to_greater_binary_base::<SourceDigit, TargetDigit>(
            source_digits,
            source_shift,
            target_shift,
        ),
        Ordering::Less => binary_digits_to_lesser_binary_base::<SourceDigit, TargetDigit>(
            source_digits,
            source_shift,
            target_shift,
        ),
    }
}

fn binary_digits_to_non_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy + DoublePrecision,
    TargetDigit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>> + Zero,
    DoublePrecisionOf<TargetDigit>:
        From<SourceDigit> + From<TargetDigit> + PrimInt + TryFrom<usize>,
    <DoublePrecisionOf<TargetDigit> as TryFrom<usize>>::Error: fmt::Debug,
    <TargetDigit as TryFrom<DoublePrecisionOf<TargetDigit>>>::Error: fmt::Debug,
{
    let result_max_digits_count: usize = 1
        + ((((source_digits.len() * source_shift) as f64) / (target_base as f64).log2()) as usize);
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_max_digits_count);
    let target_base = DoublePrecisionOf::<TargetDigit>::try_from(target_base).unwrap();
    for source_digit in source_digits.iter().rev() {
        let mut digit: DoublePrecisionOf<TargetDigit> =
            DoublePrecisionOf::<TargetDigit>::from(*source_digit);
        for index in 0..result.len() {
            let step: DoublePrecisionOf<TargetDigit> =
                (DoublePrecisionOf::<TargetDigit>::from(result[index]) << source_shift) | digit;
            digit = step / target_base;
            result[index] = TargetDigit::try_from(step - digit * target_base).unwrap();
        }
        while !digit.is_zero() {
            result.push(TargetDigit::try_from(digit % target_base).unwrap());
            digit = digit / target_base;
        }
    }
    if result.is_empty() {
        result.push(TargetDigit::zero());
    }
    result
}

fn binary_digits_to_greater_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: PrimInt,
    TargetDigit: DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>>,
    DoublePrecisionOf<TargetDigit>: From<SourceDigit> + PrimInt,
{
    let target_digit_mask = (DoublePrecisionOf::<TargetDigit>::one() << target_shift)
        - DoublePrecisionOf::<TargetDigit>::one();
    let result_capacity: usize =
        (source_digits.len() * target_shift + (target_shift - 1)) / target_shift;
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_capacity);
    let mut accumulator = DoublePrecisionOf::<TargetDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for digit in source_digits {
        accumulator = accumulator
            | (DoublePrecisionOf::<TargetDigit>::from(*digit) << accumulator_bits_count);
        accumulator_bits_count += source_shift;
        if accumulator_bits_count >= target_shift {
            unsafe {
                result.push(
                    TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked(),
                );
            }
            accumulator = accumulator >> target_shift;
            accumulator_bits_count -= target_shift;
        }
    }
    if !accumulator.is_zero() || result.is_empty() {
        unsafe {
            result.push(TargetDigit::try_from(accumulator).unwrap_unchecked());
        }
    }
    result
}

fn binary_digits_to_lesser_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: DoublePrecision + From<u8> + PrimInt,
    TargetDigit: TryFrom<DoublePrecisionOf<SourceDigit>>,
    DoublePrecisionOf<SourceDigit>: PrimInt + From<SourceDigit>,
    usize: TryFrom<SourceDigit>,
{
    let target_digit_mask = (DoublePrecisionOf::<SourceDigit>::one() << target_shift)
        - DoublePrecisionOf::<SourceDigit>::one();
    let result_digits_bits_count: usize = (source_digits.len() - 1) * source_shift
        + utils::to_bit_length(*source_digits.last().unwrap());
    let result_digits_count: usize = (result_digits_bits_count + (target_shift - 1)) / target_shift;
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_digits_count);
    let mut accumulator = DoublePrecisionOf::<SourceDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for index in 0..source_digits.len() {
        accumulator = accumulator
            | DoublePrecisionOf::<SourceDigit>::from(
                source_digits[index] << accumulator_bits_count,
            );
        accumulator_bits_count += source_shift;
        loop {
            unsafe {
                result.push(
                    TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked(),
                );
            }
            accumulator_bits_count -= target_shift;
            accumulator = accumulator >> target_shift;
            if if index == source_digits.len() - 1 {
                accumulator.is_zero()
            } else {
                accumulator_bits_count < target_shift
            } {
                break;
            }
        }
    }
    result
}

pub(crate) fn non_binary_digits_to_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_base: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy,
    TargetDigit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>> + Zero,
    DoublePrecisionOf<TargetDigit>:
        PrimInt + From<SourceDigit> + From<TargetDigit> + TryFrom<usize>,
{
    let target_base = 1usize << target_shift;
    let target_digit_mask: DoublePrecisionOf<TargetDigit>;
    unsafe {
        target_digit_mask =
            DoublePrecisionOf::<TargetDigit>::try_from(target_base - 1).unwrap_unchecked();
    }
    static mut bases_logs: [f64; 37] = [0.0; 37];
    static mut infimum_bases_exponents: [usize; 37] = [0; 37];
    static mut infimum_bases_powers: [usize; 37] = [0; 37];
    if unsafe { bases_logs[source_base] } == 0.0 {
        let bases_log = (source_base as f64).ln() / (target_base as f64).ln();
        unsafe { bases_logs[source_base] = bases_log };
        let mut infimum_base_power = source_base;
        let mut infimum_base_exponent: usize = 1;
        loop {
            let candidate: usize = infimum_base_power * source_base;
            if candidate > target_base {
                break;
            }
            infimum_base_power = candidate;
            infimum_base_exponent += 1;
        }
        unsafe { infimum_bases_powers[source_base] = infimum_base_power };
        unsafe { infimum_bases_exponents[source_base] = infimum_base_exponent };
    }
    let digits_count_upper_bound =
        (source_digits.len() as f64) * unsafe { bases_logs[source_base] } + 1.0;
    let mut digits: Vec<TargetDigit> = Vec::with_capacity(digits_count_upper_bound as usize);
    let infimum_base_exponent = unsafe { infimum_bases_exponents[source_base] };
    let infimum_base_power = unsafe { infimum_bases_powers[source_base] };
    let mut reversed_source_digits = source_digits.iter().rev();
    while let Some(&source_digit) = reversed_source_digits.next() {
        let mut digit = DoublePrecisionOf::<TargetDigit>::from(source_digit);
        let mut base_exponent: usize = 1;
        while base_exponent < infimum_base_exponent {
            if let Some(&source_digit) = reversed_source_digits.next() {
                base_exponent += 1;
                unsafe {
                    digit = digit
                        * DoublePrecisionOf::<TargetDigit>::try_from(source_base)
                            .unwrap_unchecked()
                        + DoublePrecisionOf::<TargetDigit>::from(source_digit);
                }
            } else {
                break;
            }
        }
        let base_power = if base_exponent == infimum_base_exponent {
            infimum_base_power
        } else {
            source_base.pow(base_exponent as u32)
        };
        for index in 0..digits.len() {
            digit = digit
                + DoublePrecisionOf::<TargetDigit>::from(digits[index])
                    * unsafe {
                        DoublePrecisionOf::<TargetDigit>::try_from(base_power).unwrap_unchecked()
                    };
            digits[index] =
                unsafe { TargetDigit::try_from(digit & target_digit_mask).unwrap_unchecked() };
            digit = digit >> target_shift;
        }
        if !digit.is_zero() {
            digits.push(unsafe { TargetDigit::try_from(digit).unwrap_unchecked() });
        }
    }
    if digits.is_empty() {
        digits.push(TargetDigit::zero());
    }
    digits
}

fn multiply_digits<Digit, const SHIFT: usize>(first: &Vec<Digit>, second: &Vec<Digit>) -> Vec<Digit>
where
    Digit: DoublePrecision
        + PrimInt
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + WrappingSub,
    DoublePrecisionOf<Digit>: From<Digit> + PrimInt,
{
    let mut shortest = &first;
    let mut longest = &second;
    let mut size_shortest = shortest.len();
    let mut size_longest = longest.len();
    if size_longest < size_shortest {
        (shortest, longest) = (longest, shortest);
        (size_shortest, size_longest) = (size_longest, size_shortest);
    }
    const KARATSUBA_CUTOFF: usize = 70;
    const KARATSUBA_SQUARE_CUTOFF: usize = KARATSUBA_CUTOFF * 2;
    if size_shortest
        <= if shortest.as_ptr() == longest.as_ptr() {
            KARATSUBA_SQUARE_CUTOFF
        } else {
            KARATSUBA_CUTOFF
        }
    {
        return if size_shortest == 0 {
            vec![Digit::zero()]
        } else {
            multiply_digits_plain::<Digit, SHIFT>(*shortest, *longest)
        };
    };
    if 2 * size_shortest <= size_longest {
        return multiply_digits_lopsided::<Digit, SHIFT>(*shortest, *longest);
    }
    let shift = size_longest >> 1;
    let (shortest_high, shortest_low) = split_digits(*shortest, shift);
    let (longest_high, longest_low) = if shortest.as_ptr() == longest.as_ptr() {
        (shortest_high.clone(), shortest_low.clone())
    } else {
        split_digits(*longest, shift)
    };
    let mut result = vec![Digit::zero(); size_shortest + size_longest];
    let highs_product = multiply_digits::<Digit, SHIFT>(&shortest_high, &longest_high);
    for (index, &digit) in highs_product.iter().enumerate() {
        result[index + 2 * shift] = digit;
    }
    let lows_product = multiply_digits::<Digit, SHIFT>(&shortest_low, &longest_low);
    for (index, &digit) in lows_product.iter().enumerate() {
        result[index] = digit;
    }
    subtract_digits_in_place::<Digit, SHIFT>(&mut result[shift..], &lows_product);
    subtract_digits_in_place::<Digit, SHIFT>(
        &mut result[shift..],
        &highs_product,
    );
    let shortest_components_sum = sum_digits::<Digit, SHIFT>(&shortest_high, &shortest_low);
    let longest_components_sum = if shortest.as_ptr() == longest.as_ptr() {
        shortest_components_sum.clone()
    } else {
        sum_digits::<Digit, SHIFT>(&longest_high, &longest_low)
    };
    let components_sums_product =
        multiply_digits::<Digit, SHIFT>(&shortest_components_sum, &longest_components_sum);
    sum_digits_in_place::<Digit, SHIFT>(
        &mut result[shift..],
        &components_sums_product,
    );
    normalize_digits(&mut result);
    result
}

fn multiply_digits_lopsided<Digit, const SHIFT: usize>(
    shortest: &Vec<Digit>,
    longest: &Vec<Digit>,
) -> Vec<Digit>
where
    Digit: DoublePrecision
        + PrimInt
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + WrappingSub,
    DoublePrecisionOf<Digit>: From<Digit> + PrimInt,
{
    let size_shortest = shortest.len();
    let mut size_longest = longest.len();
    let mut result = vec![Digit::zero(); size_shortest + size_longest];
    let mut processed_digits_count = 0;
    while size_longest > 0 {
        let step_digits_count = size_longest.min(size_shortest);
        let product = multiply_digits::<Digit, SHIFT>(
            shortest,
            &longest[processed_digits_count..processed_digits_count + step_digits_count].to_vec(),
        );
        sum_digits_in_place::<Digit, SHIFT>(
            &mut result[processed_digits_count..],
            &product,
        );
        size_longest -= step_digits_count;
        processed_digits_count += step_digits_count;
    }
    normalize_digits(&mut result);
    result
}

fn multiply_digits_plain<Digit, const SHIFT: usize>(
    shortest: &Vec<Digit>,
    longest: &Vec<Digit>,
) -> Vec<Digit>
where
    Digit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>> + Zero,
    DoublePrecisionOf<Digit>: From<Digit> + PrimInt,
{
    let size_shortest = shortest.len();
    let size_longest = longest.len();
    let mut result: Vec<Digit> = vec![Digit::zero(); size_shortest + size_longest];
    let digit_mask =
        (DoublePrecisionOf::<Digit>::one() << SHIFT) - DoublePrecisionOf::<Digit>::one();
    if shortest.as_ptr() == longest.as_ptr() {
        for index in 0..size_shortest {
            let mut digit = DoublePrecisionOf::<Digit>::from(shortest[index]);
            let mut result_position = index << 1;
            let mut accumulator =
                DoublePrecisionOf::<Digit>::from(result[result_position]) + digit * digit;
            result[result_position] =
                unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
            result_position += 1;
            accumulator = accumulator >> SHIFT;
            digit = digit << 1;
            for first_position in index + 1..shortest.len() {
                accumulator = accumulator
                    + DoublePrecisionOf::<Digit>::from(result[result_position])
                    + DoublePrecisionOf::<Digit>::from(shortest[first_position]) * digit;
                result[result_position] =
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
                result_position += 1;
                accumulator = accumulator >> SHIFT;
            }
            if !accumulator.is_zero() {
                accumulator =
                    accumulator + DoublePrecisionOf::<Digit>::from(result[result_position]);
                result[result_position] =
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
                result_position += 1;
                accumulator = accumulator >> SHIFT;
            }
            if !accumulator.is_zero() {
                result[result_position] = result[result_position]
                    + unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
            }
        }
    } else {
        for index in 0..size_shortest {
            let mut accumulator = DoublePrecisionOf::<Digit>::zero();
            let digit = DoublePrecisionOf::<Digit>::from(shortest[index]);
            let mut result_position = index;
            for &second_digit in longest {
                accumulator = accumulator
                    + DoublePrecisionOf::<Digit>::from(result[result_position])
                    + DoublePrecisionOf::<Digit>::from(second_digit) * digit;
                result[result_position] =
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
                result_position += 1;
                accumulator = accumulator >> SHIFT;
            }
            if !accumulator.is_zero() {
                result[result_position] = result[result_position]
                    + unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
            }
        }
    }
    normalize_digits(&mut result);
    result
}

fn split_digits<Digit>(digits: &Vec<Digit>, size: usize) -> (Vec<Digit>, Vec<Digit>)
where
    Digit: Clone + Zero,
{
    let (low, high) = digits.split_at(digits.len().min(size));
    let (mut low, mut high) = (low.to_vec(), high.to_vec());
    normalize_digits(&mut high);
    normalize_digits(&mut low);
    (high, low)
}

fn subtract_digits<Digit, const SHIFT: usize>(
    first: &Vec<Digit>,
    second: &Vec<Digit>,
    sign: &mut Sign,
) -> Vec<Digit>
where
    Digit: PrimInt + TryFrom<usize> + WrappingSub,
{
    let mut longest = &first;
    let mut shortest = &second;
    let mut size_longest = longest.len();
    let mut size_shortest = shortest.len();
    let mut accumulator = Digit::zero();
    if size_longest < size_shortest {
        (longest, shortest) = (shortest, longest);
        (size_longest, size_shortest) = (size_shortest, size_longest);
        *sign = -*sign;
    } else if size_longest == size_shortest {
        let mut index = size_shortest;
        loop {
            index -= 1;
            if index == 0 || longest[index] != shortest[index] {
                break;
            }
        }
        if index == 0 && longest[0] == shortest[0] {
            *sign = 0 as Sign;
            return vec![Digit::zero()];
        }
        if longest[index] < shortest[index] {
            (longest, shortest) = (shortest, longest);
            *sign = -*sign;
        }
        size_longest = index + 1;
        size_shortest = index + 1;
    }
    let mut result: Vec<Digit> = Vec::with_capacity(size_longest);
    let digit_mask = (Digit::one() << SHIFT) - Digit::one();
    for index in 0..size_shortest {
        accumulator = longest[index].wrapping_sub(&shortest[index]) - accumulator;
        result.push(accumulator & digit_mask);
        accumulator = accumulator >> SHIFT;
        accumulator = accumulator & Digit::one();
    }
    for index in size_shortest..size_longest {
        accumulator = longest[index] - accumulator;
        result.push(accumulator & digit_mask);
        accumulator = accumulator >> SHIFT;
        accumulator = accumulator & Digit::one();
    }
    normalize_digits(&mut result);
    result
}

fn subtract_digits_in_place<Digit, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &Vec<Digit>,
) -> Digit
where
    Digit: PrimInt + WrappingSub,
{
    let mut accumulator = Digit::zero();
    let digit_mask = (Digit::one() << SHIFT) - Digit::one();
    for index in 0..shortest.len() {
        accumulator = longest[index].wrapping_sub(&shortest[index]) - accumulator;
        longest[index] = accumulator & digit_mask;
        accumulator = (accumulator >> SHIFT) & Digit::one();
    }
    for index in shortest.len()..longest.len() {
        if accumulator.is_zero() {
            break;
        }
        accumulator = longest[index].wrapping_sub(&accumulator);
        longest[index] = accumulator & digit_mask;
        accumulator = (accumulator >> SHIFT) & Digit::one();
    }
    accumulator
}

fn sum_digits<Digit, const SHIFT: usize>(first: &Vec<Digit>, second: &Vec<Digit>) -> Vec<Digit>
where
    Digit: PrimInt + TryFrom<usize>,
{
    let mut longest = &first;
    let mut shortest = &second;
    let mut size_longest = longest.len();
    let mut size_shortest = shortest.len();
    if size_longest < size_shortest {
        (size_longest, size_shortest) = (size_shortest, size_longest);
        (longest, shortest) = (shortest, longest);
    }
    let mut result: Vec<Digit> = Vec::with_capacity(size_longest + 1);
    let mut accumulator: Digit = Digit::zero();
    let digit_mask = (Digit::one() << SHIFT) - Digit::one();
    for index in 0..size_shortest {
        accumulator = accumulator + longest[index] + shortest[index];
        result.push(accumulator & digit_mask);
        accumulator = accumulator >> SHIFT;
    }
    for index in size_shortest..size_longest {
        accumulator = accumulator + longest[index];
        result.push(accumulator & digit_mask);
        accumulator = accumulator >> SHIFT;
    }
    result.push(accumulator);
    normalize_digits(&mut result);
    result
}

fn sum_digits_in_place<Digit, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &Vec<Digit>,
) -> Digit
where
    Digit: PrimInt,
{
    let mut accumulator = Digit::zero();
    let digit_mask = (Digit::one() << SHIFT) - Digit::one();
    for index in 0..shortest.len() {
        accumulator = longest[index] + shortest[index] + accumulator;
        longest[index] = accumulator & digit_mask;
        accumulator = accumulator >> SHIFT;
    }
    for index in shortest.len()..longest.len() {
        if accumulator.is_zero() {
            break;
        }
        accumulator = accumulator + longest[index];
        longest[index] = accumulator & digit_mask;
        accumulator = accumulator >> SHIFT;
    }
    accumulator
}

fn normalize_digits<Digit>(digits: &mut Vec<Digit>) -> ()
where
    Digit: Clone + Zero,
{
    let mut digits_count = digits.len();
    while digits_count > 1 && digits[digits_count - 1].is_zero() {
        digits_count -= 1;
    }
    if digits_count != digits.len() {
        digits.resize(digits_count, Digit::zero());
    }
}
