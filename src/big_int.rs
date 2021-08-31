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

pub(crate) type DoublePrecision<T> = <T as DoublePrecisiony>::Type;

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
    SourceDigit: Copy + DoublePrecisiony + PrimInt,
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
            bases_logs[source_base as usize] =
                (source_base as f64).ln() / (target_base as f64).ln();
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
    let digits_count_upper_bound: f64;
    unsafe {
        digits_count_upper_bound = (source_digits.len() as f64) * bases_logs[source_base] + 1.0;
    }
    let mut digits: Vec<TargetDigit> = Vec::with_capacity(digits_count_upper_bound as usize);
    let infimum_base_exponent: usize;
    let infimum_base_power: usize;
    unsafe {
        infimum_base_exponent = infimum_bases_exponents[source_base];
        infimum_base_power = infimum_bases_powers[source_base];
    }
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
            unsafe {
                digit = digit
                    + DoublePrecision::<TargetDigit>::from(digits[index])
                        * DoublePrecision::<TargetDigit>::try_from(base_power).unwrap_unchecked();
            }
            unsafe {
                digits[index] = TargetDigit::try_from(digit & target_digit_mask).unwrap_unchecked();
            }
            digit = digit >> target_shift;
        }
        if !digit.is_zero() {
            unsafe {
                digits.push(TargetDigit::try_from(digit).unwrap_unchecked());
            }
        }
    }
    if digits.is_empty() {
        digits.push(TargetDigit::zero());
    }
    digits
}
