use core::fmt;
use std::cmp::Ordering;
use std::convert::TryFrom;

use num::{One, PrimInt, Unsigned, Zero};

use crate::utils;

pub(crate) trait DoublePrecisiony {
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

#[cfg(target_arch = "x86")]
pub(crate) type Digit = u16;
#[cfg(not(target_arch = "x86"))]
pub(crate) type Digit = u32;
pub(crate) type DoubleDigit = DoublePrecision<Digit>;

pub(crate) fn binary_digits_to_binary_base<SourceDigit, TargetDigit>(
    source_digits: &Vec<SourceDigit>,
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    usize: From<SourceDigit>,
    SourceDigit: DoublePrecisiony + PrimInt,
    TargetDigit: DoublePrecisiony
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecision<SourceDigit>>
        + TryFrom<DoublePrecision<TargetDigit>>,
    DoublePrecision<SourceDigit>: From<SourceDigit> + PrimInt,
    DoublePrecision<TargetDigit>: From<SourceDigit> + PrimInt,
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

type DoublePrecision<T> = <T as DoublePrecisiony>::Type;

pub(crate) fn binary_digits_to_non_binary_base<SourceDigit, TargetDigit>(
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
