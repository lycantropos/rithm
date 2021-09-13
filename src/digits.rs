use std::cmp::Ordering;
use std::convert::TryFrom;
use std::{f64, fmt};

use crate::traits::{
    AssigningAdditiveMonoid, AssigningBitwiseConjunctiveMagma, AssigningBitwiseDisjunctiveMonoid,
    AssigningDivisivePartialMagma, AssigningMultiplicativeMonoid, AssigningShiftingLeftMonoid,
    AssigningShiftingRightMonoid, AssigningSubtractiveMagma, DivisivePartialMagma, DoublePrecision,
    DoublePrecisionOf, ModularPartialMagma, ModularSubtractiveMagma, Oppose, OppositionOf, Unitary,
    Zeroable,
};
use crate::utils;

pub trait BinaryDigit = AssigningAdditiveMonoid
    + AssigningBitwiseConjunctiveMagma
    + AssigningBitwiseDisjunctiveMonoid
    + AssigningMultiplicativeMonoid
    + AssigningShiftingLeftMonoid<usize>
    + AssigningShiftingRightMonoid<usize>
    + AssigningSubtractiveMagma
    + Copy
    + PartialOrd
    + Unitary;

pub(crate) type Sign = i8;

pub(crate) fn binary_digits_to_base<SourceDigit, TargetDigit>(
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: BinaryDigit + DoublePrecision + From<u8>,
    TargetDigit: Copy
        + DoublePrecision
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + Zeroable,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    DoublePrecisionOf<TargetDigit>: AssigningDivisivePartialMagma
        + BinaryDigit
        + From<SourceDigit>
        + From<TargetDigit>
        + ModularPartialMagma
        + TryFrom<usize>,
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
    source_digits: &[SourceDigit],
    source_base: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: BinaryDigit + DoublePrecision + From<u8>,
    TargetDigit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + Zeroable,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    DoublePrecisionOf<TargetDigit>:
        BinaryDigit + From<SourceDigit> + From<TargetDigit> + TryFrom<usize>,
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
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: BinaryDigit + DoublePrecision + From<u8>,
    TargetDigit: DoublePrecision
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    DoublePrecisionOf<TargetDigit>: BinaryDigit + From<SourceDigit>,
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
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy + DoublePrecision,
    TargetDigit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>> + Zeroable,
    DoublePrecisionOf<TargetDigit>: AssigningDivisivePartialMagma
        + BinaryDigit
        + From<SourceDigit>
        + From<TargetDigit>
        + ModularPartialMagma
        + TryFrom<usize>,
    <DoublePrecisionOf<TargetDigit> as TryFrom<usize>>::Error: fmt::Debug,
    <TargetDigit as TryFrom<DoublePrecisionOf<TargetDigit>>>::Error: fmt::Debug,
{
    let result_max_digits_count: usize = 1
        + ((((source_digits.len() * source_shift) as f64) / (target_base as f64).log2()) as usize);
    let mut result = Vec::<TargetDigit>::with_capacity(result_max_digits_count);
    let target_base = DoublePrecisionOf::<TargetDigit>::try_from(target_base).unwrap();
    for source_digit in source_digits.iter().rev() {
        let mut digit: DoublePrecisionOf<TargetDigit> =
            DoublePrecisionOf::<TargetDigit>::from(*source_digit);
        for result_position in result.iter_mut() {
            let step: DoublePrecisionOf<TargetDigit> =
                (DoublePrecisionOf::<TargetDigit>::from(*result_position) << source_shift) | digit;
            digit = step / target_base;
            *result_position = TargetDigit::try_from(step - digit * target_base).unwrap();
        }
        while !digit.is_zero() {
            result.push(TargetDigit::try_from(digit.rem_euclid(target_base)).unwrap());
            digit /= target_base;
        }
    }
    if result.is_empty() {
        result.push(TargetDigit::zero());
    }
    result
}

fn binary_digits_to_greater_binary_base<SourceDigit, TargetDigit>(
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: BinaryDigit,
    TargetDigit: DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>>,
    DoublePrecisionOf<TargetDigit>: BinaryDigit + From<SourceDigit>,
{
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<TargetDigit>>(target_shift);
    let result_capacity: usize =
        (source_digits.len() * target_shift + (target_shift - 1)) / target_shift;
    let mut result = Vec::<TargetDigit>::with_capacity(result_capacity);
    let mut accumulator = DoublePrecisionOf::<TargetDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for digit in source_digits {
        accumulator |= DoublePrecisionOf::<TargetDigit>::from(*digit) << accumulator_bits_count;
        accumulator_bits_count += source_shift;
        if accumulator_bits_count >= target_shift {
            unsafe {
                result.push(
                    TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked(),
                );
            }
            accumulator >>= target_shift;
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

pub(crate) fn binary_digits_to_lesser_binary_base<SourceDigit, TargetDigit>(
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: BinaryDigit + DoublePrecision + From<u8>,
    TargetDigit: TryFrom<DoublePrecisionOf<SourceDigit>>,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    usize: TryFrom<SourceDigit>,
{
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<SourceDigit>>(target_shift);
    let result_digits_bits_count: usize = (source_digits.len() - 1) * source_shift
        + utils::bit_length(*source_digits.last().unwrap());
    let result_digits_count: usize = (result_digits_bits_count + (target_shift - 1)) / target_shift;
    let mut result = Vec::<TargetDigit>::with_capacity(result_digits_count);
    let mut accumulator = DoublePrecisionOf::<SourceDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for index in 0..source_digits.len() {
        accumulator |=
            DoublePrecisionOf::<SourceDigit>::from(source_digits[index] << accumulator_bits_count);
        accumulator_bits_count += source_shift;
        loop {
            unsafe {
                result.push(
                    TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked(),
                );
            }
            accumulator_bits_count -= target_shift;
            accumulator >>= target_shift;
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

pub(crate) fn checked_div<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    dividend_sign: Sign,
    divisor: &[Digit],
    divisor_sign: Sign,
) -> Option<(Sign, Vec<Digit>)>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularSubtractiveMagma
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma + Oppose,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<OppositionOf<Digit>>,
    usize: TryFrom<Digit>,
{
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero() || digits_lesser_than(dividend, divisor) {
        Some((Sign::zero(), vec![Digit::zero()]))
    } else if divisor.len() == 1 {
        let (digits, _) = div_rem_digits_by_digit::<Digit, SHIFT>(dividend, divisor[0]);
        Some((dividend_sign * divisor_sign, digits))
    } else {
        let (digits, _) = div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
        Some((
            dividend_sign * divisor_sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
            digits,
        ))
    }
}

pub(crate) fn checked_div_euclid<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    dividend_sign: Sign,
    divisor: &[Digit],
    divisor_sign: Sign,
) -> Option<(Sign, Vec<Digit>)>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularSubtractiveMagma
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma + Oppose,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<OppositionOf<Digit>>,
    usize: TryFrom<Digit>,
{
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero() {
        Some((Sign::zero(), vec![Digit::zero()]))
    } else if digits_lesser_than(dividend, divisor) {
        Some(
            if (dividend_sign.is_negative() && divisor_sign.is_positive())
                || (dividend_sign.is_positive() && divisor_sign.is_negative())
            {
                (-Sign::one(), vec![Digit::one()])
            } else {
                (Sign::zero(), vec![Digit::zero()])
            },
        )
    } else {
        let (sign, mut digits, remainder_is_non_zero) = if divisor.len() == 1 {
            let (digits, remainder_digit) =
                div_rem_digits_by_digit::<Digit, SHIFT>(dividend, divisor[0]);
            (
                dividend_sign * divisor_sign,
                digits,
                !remainder_digit.is_zero(),
            )
        } else {
            let (digits, remainder_digits) =
                div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
            (
                dividend_sign * divisor_sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
                digits,
                remainder_digits.len() > 1 || !remainder_digits[0].is_zero(),
            )
        };
        if remainder_is_non_zero
            && ((dividend_sign.is_negative() && divisor_sign.is_positive())
                || (dividend_sign.is_positive() && divisor_sign.is_negative()))
        {
            digits = sum_digits::<Digit, SHIFT>(&digits, &[Digit::one()]);
        }
        Some((sign, digits))
    }
}

pub(crate) fn checked_rem<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    dividend_sign: Sign,
    divisor: &[Digit],
    divisor_sign: Sign,
) -> Option<(Sign, Vec<Digit>)>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularSubtractiveMagma
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma + Oppose,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<OppositionOf<Digit>>,
    usize: TryFrom<Digit>,
{
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero() || digits_lesser_than(dividend, divisor) {
        Some((dividend_sign, dividend.to_vec()))
    } else if divisor.len() == 1 {
        let (_, digit) = div_rem_digits_by_digit::<Digit, SHIFT>(dividend, divisor[0]);
        Some((dividend_sign * ((!digit.is_zero()) as Sign), vec![digit]))
    } else {
        let (_, digits) = div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
        Some((
            dividend_sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
            digits,
        ))
    }
}

pub(crate) fn checked_rem_euclid<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    dividend_sign: Sign,
    divisor: &[Digit],
    divisor_sign: Sign,
) -> Option<(Sign, Vec<Digit>)>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularSubtractiveMagma
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma + Oppose,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<OppositionOf<Digit>>,
    usize: TryFrom<Digit>,
{
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero() {
        Some((dividend_sign, dividend.to_vec()))
    } else if digits_lesser_than(dividend, divisor) {
        Some(
            if (dividend_sign.is_negative() && divisor_sign.is_positive())
                || (dividend_sign.is_positive() && divisor_sign.is_negative())
            {
                let mut sign = dividend_sign;
                let digits = subtract_digits::<Digit, SHIFT>(dividend, divisor, &mut sign);
                (sign, digits)
            } else {
                (dividend_sign, dividend.to_vec())
            },
        )
    } else {
        let (mut sign, mut digits) = if divisor.len() == 1 {
            let (_, digit) = div_rem_digits_by_digit::<Digit, SHIFT>(dividend, divisor[0]);
            (dividend_sign * ((!digit.is_zero()) as Sign), vec![digit])
        } else {
            let (_, digits) = div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
            (
                dividend_sign * ((digits.len() > 1 || !digits[0].is_zero()) as Sign),
                digits,
            )
        };
        if (divisor_sign.is_negative() && sign.is_positive())
            || (divisor_sign.is_positive() && sign.is_negative())
        {
            digits = subtract_digits::<Digit, SHIFT>(&digits, divisor, &mut sign);
        }
        Some((sign, digits))
    }
}

#[inline]
pub(crate) fn digits_lesser_than<Digit: PartialOrd>(left: &[Digit], right: &[Digit]) -> bool {
    left.len() < right.len()
        || left.len() == right.len() && left.iter().rev().lt(right.iter().rev())
}

pub(crate) fn div_rem_digits_by_digit<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    divisor: Digit,
) -> (Vec<Digit>, Digit)
where
    Digit: BinaryDigit + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma,
{
    let mut quotient = vec![Digit::zero(); dividend.len()];
    let mut remainder = DoublePrecisionOf::<Digit>::zero();
    let digits_count = dividend.len();
    let divisor = DoublePrecisionOf::<Digit>::from(divisor);
    for offset in 1..=digits_count {
        remainder = (remainder << SHIFT)
            | DoublePrecisionOf::<Digit>::from(dividend[digits_count - offset]);
        let quotient_digit = unsafe { Digit::try_from(remainder / divisor).unwrap_unchecked() };
        quotient[digits_count - offset] = quotient_digit;
        remainder -= DoublePrecisionOf::<Digit>::from(quotient_digit) * divisor;
    }
    normalize_digits(&mut quotient);
    (quotient, unsafe {
        Digit::try_from(remainder).unwrap_unchecked()
    })
}

pub(crate) fn div_rem_two_or_more_digits<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    divisor: &[Digit],
) -> (Vec<Digit>, Vec<Digit>)
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma + Oppose,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<OppositionOf<Digit>>,
    usize: TryFrom<Digit>,
{
    let dividend_digits_count = dividend.len();
    let divisor_digits_count = divisor.len();
    let mut dividend_normalized = vec![Digit::zero(); dividend_digits_count];
    let mut divisor_normalized = vec![Digit::zero(); divisor_digits_count];
    let shift = SHIFT - utils::bit_length(divisor[divisor.len() - 1]);
    shift_digits_left::<Digit, SHIFT>(divisor, shift, divisor_normalized.as_mut_slice());
    let accumulator =
        shift_digits_left::<Digit, SHIFT>(dividend, shift, dividend_normalized.as_mut_slice());
    let last_divisor_digit_normalized = divisor_normalized[divisor_normalized.len() - 1];
    if !accumulator.is_zero()
        || dividend_normalized[dividend_normalized.len() - 1] >= last_divisor_digit_normalized
    {
        dividend_normalized.push(accumulator);
    }
    let quotient_size = dividend_normalized.len() - divisor_normalized.len();
    let mut quotient = vec![Digit::zero(); quotient_size];
    let penult_divisor_digit_normalized = divisor_normalized[divisor_digits_count - 2];
    let mut quotient_position = quotient_size;
    let base = Digit::one() << SHIFT;
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for offset in (0..quotient_size).rev() {
        let step =
            (DoublePrecisionOf::<Digit>::from(dividend_normalized[offset + divisor_digits_count])
                << SHIFT)
                | DoublePrecisionOf::<Digit>::from(
                    dividend_normalized[offset + divisor_digits_count - 1],
                );
        let mut quotient_digit = unsafe {
            Digit::try_from(step / DoublePrecisionOf::<Digit>::from(last_divisor_digit_normalized))
                .unwrap_unchecked()
        };
        let mut step_remainder = unsafe {
            Digit::try_from(
                step - DoublePrecisionOf::<Digit>::from(last_divisor_digit_normalized)
                    * DoublePrecisionOf::<Digit>::from(quotient_digit),
            )
            .unwrap_unchecked()
        };
        while DoublePrecisionOf::<Digit>::from(penult_divisor_digit_normalized)
            * DoublePrecisionOf::<Digit>::from(quotient_digit)
            > ((DoublePrecisionOf::<Digit>::from(step_remainder) << SHIFT)
                | DoublePrecisionOf::<Digit>::from(
                    dividend_normalized[offset + divisor_digits_count - 2],
                ))
        {
            quotient_digit -= Digit::one();
            step_remainder += last_divisor_digit_normalized;
            if step_remainder >= base {
                break;
            }
        }
        let mut accumulator = OppositionOf::<Digit>::zero();
        for index in 0..divisor_digits_count {
            let step =
                OppositionOf::<DoublePrecisionOf<Digit>>::from(dividend_normalized[offset + index])
                    + OppositionOf::<DoublePrecisionOf<Digit>>::from(accumulator)
                    - OppositionOf::<DoublePrecisionOf<Digit>>::from(quotient_digit)
                        * OppositionOf::<DoublePrecisionOf<Digit>>::from(divisor_normalized[index]);
            dividend_normalized[offset + index] = unsafe {
                Digit::try_from(step & OppositionOf::<DoublePrecisionOf<Digit>>::from(digit_mask))
                    .unwrap_unchecked()
            };
            accumulator =
                unsafe { OppositionOf::<Digit>::try_from(step >> SHIFT).unwrap_unchecked() };
        }
        if unsafe {
            OppositionOf::<Digit>::try_from(dividend_normalized[offset + divisor_digits_count])
                .unwrap_unchecked()
        } + accumulator
            < OppositionOf::<Digit>::zero()
        {
            let mut accumulator = Digit::zero();
            for index in 0..divisor_digits_count {
                accumulator =
                    accumulator + dividend_normalized[offset + index] + divisor_normalized[index];
                dividend_normalized[offset + index] = accumulator & digit_mask;
                accumulator >>= SHIFT;
            }
            quotient_digit -= Digit::one();
        }
        quotient_position -= 1;
        quotient[quotient_position] = quotient_digit;
    }
    if quotient_size.is_zero() {
        quotient = vec![Digit::zero()];
    }
    normalize_digits(&mut quotient);
    let mut remainder = divisor_normalized;
    shift_digits_right::<Digit, SHIFT>(
        &dividend_normalized[..divisor_digits_count],
        shift,
        remainder.as_mut_slice(),
    );
    normalize_digits(&mut remainder);
    (quotient, remainder)
}

fn non_binary_digits_to_binary_base<SourceDigit, TargetDigit>(
    source_digits: &[SourceDigit],
    source_base: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy,
    TargetDigit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>> + Zeroable,
    DoublePrecisionOf<TargetDigit>:
        BinaryDigit + From<SourceDigit> + From<TargetDigit> + TryFrom<usize>,
{
    let target_base = 1usize << target_shift;
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<TargetDigit>>(target_shift);
    static mut BASES_LOGS: [f64; 37] = [0.0; 37];
    static mut INFIMUM_BASES_EXPONENTS: [usize; 37] = [0; 37];
    static mut INFIMUM_BASES_POWERS: [usize; 37] = [0; 37];
    if unsafe { BASES_LOGS[source_base] } == 0.0 {
        let bases_log = (source_base as f64).ln() / (target_base as f64).ln();
        unsafe { BASES_LOGS[source_base] = bases_log };
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
        unsafe { INFIMUM_BASES_POWERS[source_base] = infimum_base_power };
        unsafe { INFIMUM_BASES_EXPONENTS[source_base] = infimum_base_exponent };
    }
    let digits_count_upper_bound =
        (source_digits.len() as f64) * unsafe { BASES_LOGS[source_base] } + 1.0;
    let mut result = Vec::<TargetDigit>::with_capacity(digits_count_upper_bound as usize);
    let infimum_base_exponent = unsafe { INFIMUM_BASES_EXPONENTS[source_base] };
    let infimum_base_power = unsafe { INFIMUM_BASES_POWERS[source_base] };
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
        for result_position in result.iter_mut() {
            digit += DoublePrecisionOf::<TargetDigit>::from(*result_position)
                * unsafe {
                    DoublePrecisionOf::<TargetDigit>::try_from(base_power).unwrap_unchecked()
                };
            *result_position =
                unsafe { TargetDigit::try_from(digit & target_digit_mask).unwrap_unchecked() };
            digit >>= target_shift;
        }
        if !digit.is_zero() {
            result.push(unsafe { TargetDigit::try_from(digit).unwrap_unchecked() });
        }
    }
    if result.is_empty() {
        result.push(TargetDigit::zero());
    }
    result
}

pub(crate) fn multiply_digits<Digit, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
) -> Vec<Digit>
where
    Digit: BinaryDigit
        + DoublePrecision
        + ModularSubtractiveMagma
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit,
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
        return if size_shortest == 1 && shortest[0].is_zero() {
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
    subtract_digits_in_place::<Digit, SHIFT>(&mut result[shift..], &highs_product);
    let shortest_components_sum = sum_digits::<Digit, SHIFT>(&shortest_high, &shortest_low);
    let longest_components_sum = if shortest.as_ptr() == longest.as_ptr() {
        shortest_components_sum.clone()
    } else {
        sum_digits::<Digit, SHIFT>(&longest_high, &longest_low)
    };
    let components_sums_product =
        multiply_digits::<Digit, SHIFT>(&shortest_components_sum, &longest_components_sum);
    sum_digits_in_place::<Digit, SHIFT>(&mut result[shift..], &components_sums_product);
    normalize_digits(&mut result);
    result
}

fn multiply_digits_lopsided<Digit, const SHIFT: usize>(
    shortest: &[Digit],
    longest: &[Digit],
) -> Vec<Digit>
where
    Digit: BinaryDigit
        + DoublePrecision
        + ModularSubtractiveMagma
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit,
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
        sum_digits_in_place::<Digit, SHIFT>(&mut result[processed_digits_count..], &product);
        size_longest -= step_digits_count;
        processed_digits_count += step_digits_count;
    }
    normalize_digits(&mut result);
    result
}

fn multiply_digits_plain<Digit, const SHIFT: usize>(
    shortest: &[Digit],
    longest: &[Digit],
) -> Vec<Digit>
where
    Digit: BinaryDigit + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    let size_shortest = shortest.len();
    let size_longest = longest.len();
    let mut result = vec![Digit::zero(); size_shortest + size_longest];
    let digit_mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
    if shortest.as_ptr() == longest.as_ptr() {
        for index in 0..size_shortest {
            let mut digit = DoublePrecisionOf::<Digit>::from(shortest[index]);
            let mut result_position = index << 1;
            let mut accumulator =
                DoublePrecisionOf::<Digit>::from(result[result_position]) + digit * digit;
            result[result_position] =
                unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
            result_position += 1;
            accumulator >>= SHIFT;
            digit <<= 1;
            for next_index in index + 1..shortest.len() {
                accumulator += DoublePrecisionOf::<Digit>::from(result[result_position])
                    + DoublePrecisionOf::<Digit>::from(shortest[next_index]) * digit;
                result[result_position] =
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
                result_position += 1;
                accumulator >>= SHIFT;
            }
            if !accumulator.is_zero() {
                accumulator += DoublePrecisionOf::<Digit>::from(result[result_position]);
                result[result_position] =
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
                result_position += 1;
                accumulator >>= SHIFT;
            }
            if !accumulator.is_zero() {
                result[result_position] +=
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
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
                accumulator >>= SHIFT;
            }
            if !accumulator.is_zero() {
                result[result_position] +=
                    unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() };
            }
        }
    }
    normalize_digits(&mut result);
    result
}

fn shift_digits_left<Digit, const SHIFT: usize>(
    input_digits: &[Digit],
    shift: usize,
    output_digits: &mut [Digit],
) -> Digit
where
    Digit: BinaryDigit + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    let mut accumulator: Digit = Digit::zero();
    let digit_mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
    for index in 0..input_digits.len() {
        let step = (DoublePrecisionOf::<Digit>::from(input_digits[index]) << shift)
            | DoublePrecisionOf::<Digit>::from(accumulator);
        output_digits[index] = unsafe { Digit::try_from(step & digit_mask).unwrap_unchecked() };
        accumulator = unsafe { Digit::try_from(step >> SHIFT).unwrap_unchecked() };
    }
    accumulator
}

fn shift_digits_right<Digit, const SHIFT: usize>(
    input_digits: &[Digit],
    shift: usize,
    output_digits: &mut [Digit],
) -> Digit
where
    Digit: BinaryDigit + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    let mut accumulator = Digit::zero();
    let mask = to_digit_mask::<DoublePrecisionOf<Digit>>(shift);
    for index in (0..input_digits.len()).rev() {
        let step = (DoublePrecisionOf::<Digit>::from(accumulator) << SHIFT)
            | DoublePrecisionOf::<Digit>::from(input_digits[index]);
        accumulator = unsafe { Digit::try_from(step & mask).unwrap_unchecked() };
        output_digits[index] = unsafe { Digit::try_from(step >> shift).unwrap_unchecked() };
    }
    accumulator
}

fn split_digits<Digit>(digits: &[Digit], size: usize) -> (Vec<Digit>, Vec<Digit>)
where
    Digit: Clone + Zeroable,
{
    let (low, high) = digits.split_at(digits.len().min(size));
    let (mut low, mut high) = (low.to_vec(), high.to_vec());
    normalize_digits(&mut high);
    normalize_digits(&mut low);
    (high, low)
}

pub(crate) fn subtract_signed_digits<Digit, const SHIFT: usize>(
    minuend: &[Digit],
    minuend_sign: Sign,
    subtrahend: &[Digit],
    subtrahend_sign: Sign,
) -> (Sign, Vec<Digit>)
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
{
    if minuend_sign.is_negative() {
        if subtrahend_sign.is_negative() {
            let mut sign = Sign::one();
            let digits = subtract_digits::<Digit, SHIFT>(subtrahend, minuend, &mut sign);
            (sign, digits)
        } else {
            (
                -Sign::one(),
                sum_digits::<Digit, SHIFT>(minuend, subtrahend),
            )
        }
    } else if subtrahend_sign.is_negative() {
        (Sign::one(), sum_digits::<Digit, SHIFT>(minuend, subtrahend))
    } else {
        let mut sign = Sign::one();
        let digits = subtract_digits::<Digit, SHIFT>(minuend, subtrahend, &mut sign);
        (sign, digits)
    }
}

fn subtract_digits<Digit, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
    sign: &mut Sign,
) -> Vec<Digit>
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
{
    let mut longest = &first;
    let mut shortest = &second;
    let mut size_longest = longest.len();
    let mut size_shortest = shortest.len();
    let mut accumulator = Digit::zero();
    match size_longest.cmp(&size_shortest) {
        Ordering::Less => {
            (longest, shortest) = (shortest, longest);
            (size_longest, size_shortest) = (size_shortest, size_longest);
            *sign = -*sign;
        }
        Ordering::Equal => {
            let mut index = size_shortest;
            loop {
                index -= 1;
                if index == 0 || longest[index] != shortest[index] {
                    break;
                }
            }
            if index == 0 && longest[0] == shortest[0] {
                *sign = Sign::zero();
                return vec![Digit::zero()];
            }
            if longest[index] < shortest[index] {
                (longest, shortest) = (shortest, longest);
                *sign = -*sign;
            }
            size_longest = index + 1;
            size_shortest = index + 1;
        }
        _ => {}
    };
    let mut result = Vec::<Digit>::with_capacity(size_longest);
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..size_shortest {
        accumulator = longest[index].wrapping_sub(shortest[index]) - accumulator;
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
        accumulator &= Digit::one();
    }
    for index in size_shortest..size_longest {
        accumulator = longest[index] - accumulator;
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
        accumulator &= Digit::one();
    }
    normalize_digits(&mut result);
    result
}

fn subtract_digits_in_place<Digit, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &[Digit],
) -> Digit
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
{
    let mut accumulator = Digit::zero();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..shortest.len() {
        accumulator = longest[index].wrapping_sub(shortest[index]) - accumulator;
        longest[index] = accumulator & digit_mask;
        accumulator = (accumulator >> SHIFT) & Digit::one();
    }
    for index in shortest.len()..longest.len() {
        if accumulator.is_zero() {
            break;
        }
        accumulator = longest[index].wrapping_sub(accumulator);
        longest[index] = accumulator & digit_mask;
        accumulator = (accumulator >> SHIFT) & Digit::one();
    }
    accumulator
}

pub(crate) fn sum_signed_digits<Digit, const SHIFT: usize>(
    first: &[Digit],
    first_sign: Sign,
    second: &[Digit],
    second_sign: Sign,
) -> (Sign, Vec<Digit>)
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
{
    if first_sign.is_negative() {
        if second_sign.is_negative() {
            (-Sign::one(), sum_digits::<Digit, SHIFT>(first, second))
        } else {
            let mut sign = Sign::one();
            let digits = subtract_digits::<Digit, SHIFT>(second, first, &mut sign);
            (sign, digits)
        }
    } else if second_sign.is_negative() {
        let mut sign = Sign::one();
        let digits = subtract_digits::<Digit, SHIFT>(first, second, &mut sign);
        (sign, digits)
    } else {
        (
            first_sign.max(second_sign),
            sum_digits::<Digit, SHIFT>(first, second),
        )
    }
}

fn sum_digits<Digit, const SHIFT: usize>(first: &[Digit], second: &[Digit]) -> Vec<Digit>
where
    Digit: BinaryDigit,
{
    let mut longest = &first;
    let mut shortest = &second;
    let mut size_longest = longest.len();
    let mut size_shortest = shortest.len();
    if size_longest < size_shortest {
        (size_longest, size_shortest) = (size_shortest, size_longest);
        (longest, shortest) = (shortest, longest);
    }
    let mut result = Vec::<Digit>::with_capacity(size_longest + 1);
    let mut accumulator: Digit = Digit::zero();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..size_shortest {
        accumulator += longest[index] + shortest[index];
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
    }
    for index in size_shortest..size_longest {
        accumulator += longest[index];
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
    }
    result.push(accumulator);
    normalize_digits(&mut result);
    result
}

fn sum_digits_in_place<Digit, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &[Digit],
) -> Digit
where
    Digit: BinaryDigit,
{
    let mut accumulator = Digit::zero();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..shortest.len() {
        accumulator = longest[index] + shortest[index] + accumulator;
        longest[index] = accumulator & digit_mask;
        accumulator >>= SHIFT;
    }
    for index in shortest.len()..longest.len() {
        if accumulator.is_zero() {
            break;
        }
        accumulator += longest[index];
        longest[index] = accumulator & digit_mask;
        accumulator >>= SHIFT;
    }
    accumulator
}

pub(crate) fn normalize_digits<Digit>(digits: &mut Vec<Digit>)
where
    Digit: Clone + Zeroable,
{
    let mut digits_count = digits.len();
    while digits_count > 1 && digits[digits_count - 1].is_zero() {
        digits_count -= 1;
    }
    if digits_count != digits.len() {
        digits.resize(digits_count, Digit::zero());
    }
}

pub(crate) fn reduce_digits<Digit, Output, const SHIFT: usize>(digits: &[Digit]) -> Output
where
    Digit: Copy,
    Output: BinaryDigit + From<Digit>,
{
    let mut result = Output::zero();
    for &digit in digits.iter().rev() {
        result = (result << SHIFT) | <Output as From<Digit>>::from(digit);
    }
    result
}

#[inline]
pub(crate) fn to_digit_mask<Digit>(shift: usize) -> Digit
where
    Digit: BinaryDigit,
{
    (Digit::one() << shift) - Digit::one()
}
