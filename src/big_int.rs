use std::cmp::Ordering;
use std::f64;

use crate::utils;
use crate::utils::BitSized;

pub(crate) trait DoublePrecision {
    type Type;
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

#[cfg(target_arch = "x86")]
pub(crate) type Digit = u16;
#[cfg(not(target_arch = "x86"))]
pub(crate) type Digit = u32;
pub(crate) type DoubleDigit = <Digit as DoublePrecision>::Type;

pub(crate) fn binary_digits_to_binary_base(
    source_digits: &Vec<Digit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<Digit> {
    match target_shift.cmp(&source_shift) {
        Ordering::Equal => source_digits.clone(),
        Ordering::Greater => {
            binary_digits_to_greater_binary_base(source_digits, source_shift, target_shift)
        }
        Ordering::Less => {
            binary_digits_to_lesser_binary_base(source_digits, source_shift, target_shift)
        }
    }
}

pub(crate) fn binary_digits_to_non_binary_base(
    source_digits: &Vec<Digit>,
    source_shift: usize,
    target_base: usize,
) -> Vec<Digit> {
    type SourceDigit = Digit;
    type TargetDigit = SourceDigit;
    let result_max_digits_count: usize = 1
        + ((((source_digits.len() * source_shift) as f64) / (target_base as f64).log2()) as usize);
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_max_digits_count);
    type TargetDoubleDigit = <TargetDigit as DoublePrecision>::Type;
    for source_digit in source_digits.iter().rev() {
        let mut digit: TargetDoubleDigit = *source_digit as TargetDoubleDigit;
        for index in 0..result.len() {
            let step: TargetDoubleDigit =
                ((result[index] as TargetDoubleDigit) << source_shift) | digit;
            digit = step / (target_base as TargetDoubleDigit);
            result[index] = (step
                - (digit as TargetDoubleDigit) * (target_base as TargetDoubleDigit))
                as TargetDigit;
        }
        while digit != 0 {
            result.push((digit % (target_base as TargetDoubleDigit)) as TargetDigit);
            digit /= target_base as TargetDoubleDigit;
        }
    }
    if result.is_empty() {
        result.push(0);
    }
    result
}

fn binary_digits_to_greater_binary_base(
    source_digits: &Vec<Digit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<Digit> {
    type SourceDigit = Digit;
    type TargetDigit = SourceDigit;
    type TargetDoubleDigit = <TargetDigit as DoublePrecision>::Type;
    let target_digit_mask: TargetDoubleDigit = (1 << target_shift) - 1;
    let result_capacity: usize =
        (source_digits.len() * target_shift + (target_shift - 1)) / target_shift;
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_capacity);
    let mut accumulator: TargetDoubleDigit = 0;
    let mut accumulator_bits_count: usize = 0;
    for digit in source_digits {
        accumulator |= (digit << accumulator_bits_count) as TargetDoubleDigit;
        accumulator_bits_count += source_shift;
        if accumulator_bits_count >= target_shift {
            result.push((accumulator & target_digit_mask) as TargetDigit);
            accumulator >>= target_shift;
            accumulator_bits_count -= target_shift;
        }
    }
    if accumulator != 0 {
        result.push(accumulator as TargetDigit);
    }
    result
}

fn binary_digits_to_lesser_binary_base(
    source_digits: &Vec<Digit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<Digit> {
    type SourceDigit = Digit;
    type TargetDigit = SourceDigit;
    type SourceDoubleDigit = <TargetDigit as DoublePrecision>::Type;
    let target_digit_mask: SourceDoubleDigit = (1 << target_shift) - 1;
    let result_digits_bits_count: usize = (source_digits.len() - 1) * source_shift
        + source_digits.last().unwrap().bit_length();
    let result_digits_count: usize = (result_digits_bits_count + (target_shift - 1)) / target_shift;
    let mut result: Vec<TargetDigit> = Vec::with_capacity(result_digits_count);
    let mut accumulator: SourceDoubleDigit = 0;
    let mut accumulator_bits_count: usize = 0;
    for index in 0..source_digits.len() {
        accumulator |= (source_digits[index] << accumulator_bits_count) as SourceDoubleDigit;
        accumulator_bits_count += source_shift;
        loop {
            result.push((accumulator & target_digit_mask) as TargetDigit);
            accumulator_bits_count -= target_shift;
            accumulator >>= target_shift;
            if if index == source_digits.len() - 1 {
                accumulator == 0
            } else {
                accumulator_bits_count < target_shift
            } {
                break;
            }
        }
    }
    result
}
