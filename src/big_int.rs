use core::fmt;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::f64;
use std::iter::Peekable;
use std::ops::{Add, Neg};
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
    u8: DoublePrecisiony + PrimInt,
    Digit: Copy
        + DoublePrecisiony
        + TryFrom<DoublePrecision<Digit>>
        + TryFrom<u8>
        + TryFrom<DoublePrecision<u8>>
        + TryFrom<DoublePrecision<Digit>>
        + Zero,
    DoublePrecision<u8>: From<u8> + PrimInt,
    DoublePrecision<Digit>: From<u8> + From<Digit> + PrimInt + TryFrom<usize>,
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

impl<Digit, const SHIFT: usize> BigInt<Digit, SHIFT>
where
    Digit: DoublePrecisiony + PrimInt + TryFrom<DoublePrecision<Digit>> + TryFrom<usize>,
    DoublePrecision<Digit>: From<Digit> + PrimInt + TryFrom<usize>,
    <DoublePrecision<Digit> as TryFrom<usize>>::Error: fmt::Debug,
    <Digit as TryFrom<DoublePrecision<Digit>>>::Error: fmt::Debug,
    usize: TryFrom<Digit>,
{
    const DIGIT_VALUES_ASCII_CODES: [char; MAX_REPRESENTABLE_BASE as usize] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    pub(crate) fn to_string(&self, target_base: usize) -> String {
        let target_shift = utils::floor_log(1 << SHIFT, target_base);
        let target_digits: Vec<Digit> = binary_digits_to_base::<Digit, Digit>(
            &self.digits,
            SHIFT,
            utils::power(target_base, target_shift),
        );
        let characters_count = ((self.sign < 0) as usize)
            + (target_digits.len() - 1) * target_shift
            + utils::floor_log(
                unsafe { usize::try_from(*target_digits.last().unwrap()).unwrap_unchecked() },
                target_base,
            )
            + 1;
        let mut characters: String = String::with_capacity(characters_count);
        let target_base = unsafe { Digit::try_from(target_base).unwrap_unchecked() };
        for index in 0..target_digits.len() - 1 {
            let mut remainder = target_digits[index];
            for _ in 0..target_shift {
                characters.push(
                    Self::DIGIT_VALUES_ASCII_CODES
                        [unsafe { usize::try_from(remainder % target_base).unwrap_unchecked() }],
                );
                remainder = remainder / target_base;
            }
        }
        let mut remainder = *target_digits.last().unwrap();
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

impl<Digit, const SHIFT: usize> Neg for BigInt<Digit, SHIFT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            sign: -self.sign,
            digits: self.digits,
        }
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

pub trait DoublePrecisiony {
    type Type;
}

impl DoublePrecisiony for u8 {
    type Type = u16;
}

impl DoublePrecisiony for u16 {
    type Type = u32;
}

impl DoublePrecisiony for u32 {
    type Type = u64;
}

pub type DoublePrecision<T> = <T as DoublePrecisiony>::Type;
pub(crate) type Sign = i8;

pub(crate) fn binary_digits_to_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy + DoublePrecisiony + PrimInt,
    TargetDigit: Copy
        + DoublePrecisiony
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecision<SourceDigit>>
        + TryFrom<DoublePrecision<TargetDigit>>
        + Zero,
    DoublePrecision<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecision<TargetDigit>: From<SourceDigit> + From<TargetDigit> + PrimInt + TryFrom<usize>,
    <DoublePrecision<TargetDigit> as TryFrom<usize>>::Error: fmt::Debug,
    <TargetDigit as TryFrom<DoublePrecision<TargetDigit>>>::Error: fmt::Debug,
    usize: TryFrom<SourceDigit>,
{
    if target_base & (target_base - 1) == 0 {
        binary_digits_to_binary_base(
            source_digits,
            source_shift,
            utils::floor_log2::<usize>(target_base),
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
    SourceDigit: DoublePrecisiony + PrimInt,
    TargetDigit: Copy
        + DoublePrecisiony
        + TryFrom<DoublePrecision<TargetDigit>>
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecision<SourceDigit>>
        + TryFrom<DoublePrecision<TargetDigit>>
        + Zero,
    DoublePrecision<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecision<TargetDigit>: From<SourceDigit> + From<TargetDigit> + PrimInt + TryFrom<usize>,
    usize: TryFrom<SourceDigit>,
{
    if source_base & (source_base - 1) == 0 {
        binary_digits_to_binary_base(
            source_digits,
            utils::floor_log2::<usize>(source_base),
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
    SourceDigit: DoublePrecisiony + PrimInt,
    TargetDigit: DoublePrecisiony
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecision<SourceDigit>>
        + TryFrom<DoublePrecision<TargetDigit>>,
    DoublePrecision<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecision<TargetDigit>: From<SourceDigit> + PrimInt,
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
    SourceDigit: Copy + DoublePrecisiony,
    TargetDigit: Copy + DoublePrecisiony + TryFrom<DoublePrecision<TargetDigit>> + Zero,
    DoublePrecision<TargetDigit>: From<SourceDigit> + From<TargetDigit> + PrimInt + TryFrom<usize>,
    <DoublePrecision<TargetDigit> as TryFrom<usize>>::Error: fmt::Debug,
    <TargetDigit as TryFrom<DoublePrecision<TargetDigit>>>::Error: fmt::Debug,
{
    let result_max_digits_count: usize = 1
        + ((((source_digits.len() * source_shift) as f64) / (target_base as f64).log2()) as usize);
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_max_digits_count);
    let target_base = DoublePrecision::<TargetDigit>::try_from(target_base).unwrap();
    for source_digit in source_digits.iter().rev() {
        let mut digit: DoublePrecision<TargetDigit> =
            DoublePrecision::<TargetDigit>::from(*source_digit);
        for index in 0..result.len() {
            let step: DoublePrecision<TargetDigit> =
                (DoublePrecision::<TargetDigit>::from(result[index]) << source_shift) | digit;
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
    TargetDigit: DoublePrecisiony + TryFrom<DoublePrecision<TargetDigit>>,
    DoublePrecision<TargetDigit>: From<SourceDigit> + PrimInt,
{
    let target_digit_mask = (DoublePrecision::<TargetDigit>::one() << target_shift)
        - DoublePrecision::<TargetDigit>::one();
    let result_capacity: usize =
        (source_digits.len() * target_shift + (target_shift - 1)) / target_shift;
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_capacity);
    let mut accumulator = DoublePrecision::<TargetDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for digit in source_digits {
        accumulator =
            accumulator | (DoublePrecision::<TargetDigit>::from(*digit) << accumulator_bits_count);
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
    SourceDigit: DoublePrecisiony + PrimInt,
    TargetDigit: TryFrom<DoublePrecision<SourceDigit>>,
    DoublePrecision<SourceDigit>: PrimInt + From<SourceDigit>,
    usize: TryFrom<SourceDigit>,
{
    let target_digit_mask = (DoublePrecision::<SourceDigit>::one() << target_shift)
        - DoublePrecision::<SourceDigit>::one();
    let result_digits_bits_count: usize = (source_digits.len() - 1) * source_shift
        + utils::to_bit_length(*source_digits.last().unwrap());
    let result_digits_count: usize = (result_digits_bits_count + (target_shift - 1)) / target_shift;
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_digits_count);
    let mut accumulator = DoublePrecision::<SourceDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for index in 0..source_digits.len() {
        accumulator = accumulator
            | DoublePrecision::<SourceDigit>::from(source_digits[index] << accumulator_bits_count);
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
    TargetDigit: Copy + DoublePrecisiony + TryFrom<DoublePrecision<TargetDigit>> + Zero,
    DoublePrecision<TargetDigit>: PrimInt + From<SourceDigit> + From<TargetDigit> + TryFrom<usize>,
{
    let target_base = 1usize << target_shift;
    let target_digit_mask: DoublePrecision<TargetDigit>;
    unsafe {
        target_digit_mask =
            DoublePrecision::<TargetDigit>::try_from(target_base - 1).unwrap_unchecked();
    }
    static mut bases_logs: [f64; 37] = [0.0; 37];
    static mut infimum_bases_exponents: [usize; 37] = [0; 37];
    static mut infimum_bases_powers: [usize; 37] = [0; 37];
    unsafe {
        if bases_logs[source_base] == 0.0 {
            let mut infimum_base_power = source_base;
            let mut infimum_base_exponent: usize = 1;
            bases_logs[source_base] = (source_base as f64).ln() / (target_base as f64).ln();
            loop {
                let candidate: usize = infimum_base_power * source_base;
                if candidate > target_base {
                    break;
                }
                infimum_base_power = candidate;
                infimum_base_exponent += 1;
            }
            infimum_bases_powers[source_base] = infimum_base_power;
            infimum_bases_exponents[source_base] = infimum_base_exponent;
        }
    }
    let digits_count_upper_bound =
        (source_digits.len() as f64) * unsafe { bases_logs[source_base] } + 1.0;
    let mut digits: Vec<TargetDigit> = Vec::with_capacity(digits_count_upper_bound as usize);
    let infimum_base_exponent = unsafe { infimum_bases_exponents[source_base] };
    let infimum_base_power = unsafe { infimum_bases_powers[source_base] };
    let mut reversed_source_digits = source_digits.iter().rev();
    while let Some(&source_digit) = reversed_source_digits.next() {
        let mut digit = DoublePrecision::<TargetDigit>::from(source_digit);
        let mut base_exponent: usize = 1;
        while base_exponent < infimum_base_exponent {
            if let Some(&source_digit) = reversed_source_digits.next() {
                base_exponent += 1;
                unsafe {
                    digit = digit
                        * DoublePrecision::<TargetDigit>::try_from(source_base).unwrap_unchecked()
                        + DoublePrecision::<TargetDigit>::from(source_digit);
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
                + DoublePrecision::<TargetDigit>::from(digits[index])
                    * unsafe {
                        DoublePrecision::<TargetDigit>::try_from(base_power).unwrap_unchecked()
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
