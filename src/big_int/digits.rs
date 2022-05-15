use std::cmp::Ordering;
use std::convert::{FloatToInt, TryFrom};
use std::fmt::{Debug, Display};
use std::mem::size_of;

use crate::traits::{
    AssigningAdditiveGroup, AssigningAdditiveMonoid, AssigningBitwiseConjunctiveMagma,
    AssigningBitwiseDisjunctiveMonoid, AssigningBitwiseExclusiveDisjunctiveMonoid,
    AssigningDivisivePartialMagma, AssigningMultiplicativeMonoid, AssigningShiftableLeftBy,
    AssigningShiftableRightBy, AssigningSubtractiveMagma, BitLength, BitwiseNegatableUnaryAlgebra,
    CheckedShl, DoublePrecision, DoublePrecisionOf, Float, Gcd, ModularPartialMagma,
    ModularSubtractiveMagma, Oppose, OppositionOf, Signed, Unitary, Zeroable,
};

use super::types::{CheckedDivAsFloatError, ShlError, Sign, WindowDigit};

pub trait AdditiveDigit = AssigningAdditiveMonoid
    + AssigningBitwiseConjunctiveMagma
    + AssigningShiftableRightBy<usize>
    + Copy
    + MaskableDigit
    + ModularSubtractiveMagma
    + PartialOrd;

pub trait BinaryDigitConvertibleTo<Target> =
    BinaryDigitConvertibleToBinary<Target> + BinaryDigitConvertibleToNonBinary<Target>;

pub trait BinaryDigitConvertibleToBinary<Target> =
    BinaryDigitDowncastableTo<Target> + BinaryDigitUpcastableTo<Target> where Target: TryFrom<Self>;

pub trait BinaryDigitConvertibleToFloat<Target> = AssigningBitwiseConjunctiveMagma
    + AssigningBitwiseDisjunctiveMonoid
    + BitLength<Output = usize>
    + Oppose
    + From<u8>
    + ShiftableInPlaceDigit
    + TryFrom<OppositionOf<Self>>
    + Unitary
where
    OppositionOf<Self>: AssigningAdditiveMonoid + From<i8> + TryFrom<Self>,
    Target: From<Self>,
    usize: TryFrom<Self>;

pub trait BinaryDigitConvertibleToNonBinary<Target> = Copy
where
    Target: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Target>> + Zeroable,
    DoublePrecisionOf<Target>: AssigningDivisivePartialMagma
        + AssigningBitwiseDisjunctiveMonoid
        + AssigningMultiplicativeMonoid
        + AssigningShiftableLeftBy<usize>
        + AssigningSubtractiveMagma
        + Copy
        + From<Self>
        + From<Target>
        + ModularPartialMagma
        + TryFrom<usize>;

pub trait BinaryDigitDowncastableTo<Target> = Copy + BitLength<Output = usize> + DoublePrecision
where
    Target: TryFrom<DoublePrecisionOf<Self>>,
    DoublePrecisionOf<Self>: AssigningBitwiseConjunctiveMagma
        + AssigningBitwiseDisjunctiveMonoid
        + AssigningShiftableRightBy<usize>
        + Copy
        + MaskableDigit;

pub trait BinaryDigitUpcastableTo<Target> = Copy
where
    Target: DoublePrecision + TryFrom<DoublePrecisionOf<Target>>,
    DoublePrecisionOf<Target>: AssigningBitwiseConjunctiveMagma
        + AssigningBitwiseDisjunctiveMonoid
        + AssigningShiftableRightBy<usize>
        + Copy
        + MaskableDigit
        + From<Self>;

pub trait BitwiseConjunctiveDigit = ComplementableDigit;

pub trait BitwiseDisjunctiveDigit = AssigningBitwiseDisjunctiveMonoid + ComplementableDigit;

pub trait BitwiseExclusiveDisjunctiveDigit = ComplementableDigit;

pub trait ComplementableDigit = AssigningAdditiveMonoid
    + AssigningBitwiseConjunctiveMagma
    + AssigningBitwiseExclusiveDisjunctiveMonoid
    + AssigningShiftableRightBy<usize>
    + Copy
    + MaskableDigit;

pub trait ConstructibleFrom<Source> = AssigningBitwiseConjunctiveMagma
    + AssigningShiftableRightBy<usize>
    + MaskableDigit
    + Copy
    + Oppose
    + TryFrom<Source>
where
    Source: AssigningBitwiseConjunctiveMagma
        + AssigningShiftableRightBy<usize>
        + MaskableDigit
        + Copy
        + Oppose
        + TryFrom<OppositionOf<Source>>,
    OppositionOf<Source>: TryFrom<Source>;

pub trait DigitConvertibleFromF64 = Copy + Zeroable where f64: FloatToInt<Self> + From<Self>;

pub trait DisplayableDigit = AssigningDivisivePartialMagma
    + BinaryDigitConvertibleTo<Self>
    + ModularPartialMagma
    + Zeroable
    + TryFrom<usize>
where usize: TryFrom<Self>;

pub trait DivisibleAsFloatDigit<Target> = AssigningMultiplicativeMonoid
    + BinaryDigitConvertibleToFloat<Target>
    + BitwiseNegatableUnaryAlgebra
    + DivisibleDigit;

pub trait DivisibleDigit = AssigningAdditiveMonoid
    + AssigningBitwiseConjunctiveMagma
    + AssigningShiftableRightBy<usize>
    + BitLength<Output = usize>
    + MaskableDigit
    + Oppose
    + PartialOrd
    + ShiftableInPlaceDigit
    + TryFrom<OppositionOf<DoublePrecisionOf<Self>>>
where
    DoublePrecisionOf<Self>:
        AssigningMultiplicativeMonoid + AssigningDivisivePartialMagma + Oppose + PartialOrd,
    OppositionOf<Self>: AssigningAdditiveMonoid
        + Copy
        + PartialOrd
        + TryFrom<OppositionOf<DoublePrecisionOf<Self>>>
        + TryFrom<Self>,
    OppositionOf<DoublePrecisionOf<Self>>: AssigningAdditiveGroup
        + AssigningBitwiseConjunctiveMagma
        + AssigningMultiplicativeMonoid
        + AssigningShiftableRightBy<usize>
        + Copy
        + From<Self>
        + From<OppositionOf<Self>>;

pub trait EuclidDivisibleDigit = AdditiveDigit + DivisibleDigit;

pub trait ExponentiativeDigit =
    MultiplicativeDigit + From<u8> + BinaryDigitDowncastableTo<WindowDigit>;

pub trait FromStrDigit =
    Oppose where u8: BinaryDigitConvertibleToBinary<Self> + NonBinaryDigitConvertibleToBinary<Self>;

pub trait GcdDigit = DivisibleDigit + ModularSubtractiveMagma
where
    DoublePrecisionOf<Self>: Gcd<Output = DoublePrecisionOf<Self>>
        + ModularPartialMagma
        + TryFrom<OppositionOf<DoublePrecisionOf<Self>>>,
    OppositionOf<DoublePrecisionOf<Self>>: AssigningBitwiseDisjunctiveMonoid
        + AssigningDivisivePartialMagma
        + MaskableDigit
        + ModularPartialMagma
        + PartialOrd
        + TryFrom<DoublePrecisionOf<Self>>;

pub trait InvertibleDigit = AdditiveDigit;

pub trait MaskableDigit<Subtrahend = Self> =
    AssigningShiftableLeftBy<usize> + AssigningSubtractiveMagma + Unitary;

pub trait MaybeReducibleTo<Target> = Copy
where
    Target: AssigningBitwiseDisjunctiveMonoid
        + Display
        + CheckedShl<usize, Output = Option<Target>>
        + TryFrom<Self>
        + Zeroable;

pub trait ModularInvertibleDigit = EuclidDivisibleDigit + MultiplicativeDigit;

pub trait MultiplicativeDigit =
    AdditiveDigit + DoublePrecision + TryFrom<DoublePrecisionOf<Self>> + TryFrom<usize>
    where
        DoublePrecisionOf<Self>: AssigningAdditiveMonoid
            + AssigningBitwiseConjunctiveMagma
            + AssigningMultiplicativeMonoid
            + AssigningShiftableRightBy<usize>
            + Copy
            + MaskableDigit;

pub trait NonBinaryDigitConvertibleToBinary<Target> = Copy
where
    Target: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Target>> + Zeroable,
    DoublePrecisionOf<Target>: AssigningAdditiveMonoid
        + AssigningBitwiseConjunctiveMagma
        + AssigningMultiplicativeMonoid
        + AssigningShiftableRightBy<usize>
        + Copy
        + From<Self>
        + From<Target>
        + MaskableDigit
        + TryFrom<usize>;

pub trait ParitiableDigit = AssigningBitwiseConjunctiveMagma + Copy + Unitary;

pub(super) trait ReducibleTo<Target> = Copy
where
    Target:
        AssigningBitwiseDisjunctiveMonoid + AssigningShiftableLeftBy<usize> + From<Self> + Zeroable;

pub trait ShiftableInPlaceDigit =
    Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Self>> + Zeroable
    where DoublePrecisionOf<Self>: BitwiseDisjunctiveDigit;

pub trait ShiftableLeftDigit = Debug + DivisibleDigit + MaybeReducibleTo<usize> + TryFrom<usize>
where DoublePrecisionOf<Self>: AssigningShiftableLeftBy<Self>;

pub trait ShiftableRightDigit = AssigningBitwiseDisjunctiveMonoid
    + AssigningBitwiseExclusiveDisjunctiveMonoid
    + AssigningShiftableRightBy
    + DivisibleDigit
    + Debug
    + InvertibleDigit
    + MaybeReducibleTo<usize>
    + TryFrom<usize>
where DoublePrecisionOf<Self>: AssigningShiftableLeftBy<Self>;

pub(super) fn binary_digits_to_base<
    SourceDigit: BinaryDigitConvertibleTo<TargetDigit>,
    TargetDigit,
>(
    source: &[SourceDigit],
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit> {
    if target_base & (target_base - 1) == 0 {
        binary_digits_to_binary_base(source, source_shift, floor_log2::<usize>(target_base))
    } else {
        binary_digits_to_non_binary_base(source, source_shift, target_base)
    }
}

pub(super) fn digits_to_binary_base<
    SourceDigit: BinaryDigitConvertibleToBinary<TargetDigit> + NonBinaryDigitConvertibleToBinary<TargetDigit>,
    TargetDigit,
    const TARGET_SHIFT: usize,
>(
    source: &[SourceDigit],
    source_base: usize,
) -> Vec<TargetDigit> {
    if source_base & (source_base - 1) == 0 {
        binary_digits_to_binary_base::<SourceDigit, TargetDigit>(
            source,
            floor_log2::<usize>(source_base),
            TARGET_SHIFT,
        )
    } else if source_base < (1 << TARGET_SHIFT) {
        non_binary_digits_to_greater_binary_base::<SourceDigit, TargetDigit, TARGET_SHIFT>(
            source,
            source_base,
        )
    } else {
        non_binary_digits_to_lesser_binary_base::<SourceDigit, TargetDigit, TARGET_SHIFT>(
            source,
            source_base,
        )
    }
}

pub(super) fn binary_digits_to_binary_base<
    SourceDigit: BinaryDigitConvertibleToBinary<TargetDigit>,
    TargetDigit,
>(
    source: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit> {
    match target_shift.cmp(&source_shift) {
        Ordering::Equal => source
            .iter()
            .map(|&digit| unsafe { TargetDigit::try_from(digit).unwrap_unchecked() })
            .collect(),
        Ordering::Greater => binary_digits_to_greater_binary_base::<SourceDigit, TargetDigit>(
            source,
            source_shift,
            target_shift,
        ),
        Ordering::Less => binary_digits_to_lesser_binary_base::<SourceDigit, TargetDigit>(
            source,
            source_shift,
            target_shift,
        ),
    }
}

fn binary_digits_to_non_binary_base<
    SourceDigit: BinaryDigitConvertibleToNonBinary<TargetDigit>,
    TargetDigit,
>(
    source: &[SourceDigit],
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit> {
    let result_max_digits_count: usize =
        1 + ((((source.len() * source_shift) as f64) / (target_base as f64).log2()) as usize);
    let mut result = Vec::<TargetDigit>::with_capacity(result_max_digits_count);
    let target_base =
        unsafe { DoublePrecisionOf::<TargetDigit>::try_from(target_base).unwrap_unchecked() };
    for digit in source.iter().rev() {
        let mut accumulator: DoublePrecisionOf<TargetDigit> =
            DoublePrecisionOf::<TargetDigit>::from(*digit);
        for result_position in result.iter_mut() {
            let step: DoublePrecisionOf<TargetDigit> =
                (DoublePrecisionOf::<TargetDigit>::from(*result_position) << source_shift)
                    | accumulator;
            accumulator = step / target_base;
            *result_position = unsafe {
                TargetDigit::try_from(step - accumulator * target_base).unwrap_unchecked()
            };
        }
        while !accumulator.is_zero() {
            result.push(unsafe {
                TargetDigit::try_from(accumulator.rem_euclid(target_base)).unwrap_unchecked()
            });
            accumulator /= target_base;
        }
    }
    if result.is_empty() {
        result.push(TargetDigit::zero());
    }
    result
}

fn binary_digits_to_greater_binary_base<
    SourceDigit: BinaryDigitUpcastableTo<TargetDigit>,
    TargetDigit,
>(
    source: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit> {
    debug_assert!(target_shift > source_shift && source_shift > 0);
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<TargetDigit>>(target_shift);
    let result_capacity: usize = (source.len() * target_shift + (target_shift - 1)) / target_shift;
    let mut result = Vec::<TargetDigit>::with_capacity(result_capacity);
    let mut accumulator = DoublePrecisionOf::<TargetDigit>::zero();
    let mut accumulator_bits_count: usize = 0;
    for digit in source {
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

pub(super) fn binary_digits_to_lesser_binary_base<
    SourceDigit: BinaryDigitDowncastableTo<TargetDigit>,
    TargetDigit,
>(
    source: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit> {
    debug_assert!(source_shift > target_shift && target_shift > 0);
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<SourceDigit>>(target_shift);
    let digits_bits_count: usize =
        (source.len() - 1) * source_shift + source[source.len() - 1].bit_length();
    let digits_count: usize = (digits_bits_count + (target_shift - 1)) / target_shift;
    let mut result = Vec::<TargetDigit>::with_capacity(digits_count);
    let mut accumulator = DoublePrecisionOf::<SourceDigit>::from(source[0]);
    let mut accumulator_bits_count = source_shift;
    for &digit in source.iter().skip(1usize) {
        loop {
            result.push(unsafe {
                TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked()
            });
            accumulator >>= target_shift;
            accumulator_bits_count -= target_shift;
            if accumulator_bits_count < target_shift {
                break;
            }
        }
        accumulator |= DoublePrecisionOf::<SourceDigit>::from(digit) << accumulator_bits_count;
        accumulator_bits_count += source_shift;
    }
    loop {
        result.push(unsafe {
            TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked()
        });
        accumulator >>= target_shift;
        if accumulator.is_zero() {
            break;
        }
    }
    result
}

pub(super) fn bitwise_and_components<Digit: BitwiseConjunctiveDigit, const SHIFT: usize>(
    first_sign: Sign,
    first: Vec<Digit>,
    second_sign: Sign,
    second: Vec<Digit>,
) -> (Sign, Vec<Digit>) {
    let (longest_sign, mut longest, shortest_sign, mut shortest) = if first.len() < second.len() {
        (second_sign, second, first_sign, first)
    } else {
        (first_sign, first, second_sign, second)
    };
    if longest_sign.is_negative() {
        complement_in_place::<Digit, SHIFT>(&mut longest);
    };
    if shortest_sign.is_negative() {
        complement_in_place::<Digit, SHIFT>(&mut shortest);
    };
    let mut result = longest;
    if !shortest_sign.is_negative() {
        result.truncate(shortest.len());
    };
    for index in 0..shortest.len() {
        result[index] &= shortest[index];
    }
    let mut sign = longest_sign & shortest_sign;
    if sign.is_negative() {
        result.push(to_digit_mask::<Digit>(SHIFT));
        complement_in_place::<Digit, SHIFT>(&mut result);
    }
    trim_leading_zeros(&mut result);
    sign *= to_digits_sign(&result);
    (sign, result)
}

pub(super) fn bitwise_or_components<Digit: BitwiseDisjunctiveDigit, const SHIFT: usize>(
    first_sign: Sign,
    first: Vec<Digit>,
    second_sign: Sign,
    second: Vec<Digit>,
) -> (Sign, Vec<Digit>) {
    let (longest_sign, mut longest, shortest_sign, mut shortest) = if first.len() < second.len() {
        (second_sign, second, first_sign, first)
    } else {
        (first_sign, first, second_sign, second)
    };
    if longest_sign.is_negative() {
        complement_in_place::<Digit, SHIFT>(&mut longest);
    };
    if shortest_sign.is_negative() {
        complement_in_place::<Digit, SHIFT>(&mut shortest);
    };
    let mut result = longest;
    if shortest_sign.is_negative() {
        result.truncate(shortest.len());
    };
    for index in 0..shortest.len() {
        result[index] |= shortest[index];
    }
    let sign = longest_sign | shortest_sign;
    if sign.is_negative() {
        result.push(to_digit_mask::<Digit>(SHIFT));
        complement_in_place::<Digit, SHIFT>(&mut result);
    }
    trim_leading_zeros(&mut result);
    (sign, result)
}

pub(super) fn bitwise_xor_components<
    Digit: BitwiseExclusiveDisjunctiveDigit,
    const SHIFT: usize,
>(
    first_sign: Sign,
    first: Vec<Digit>,
    second_sign: Sign,
    second: Vec<Digit>,
) -> (Sign, Vec<Digit>) {
    let (longest_sign, mut longest, shortest_sign, mut shortest) = if first.len() < second.len() {
        (second_sign, second, first_sign, first)
    } else {
        (first_sign, first, second_sign, second)
    };
    if longest_sign.is_negative() {
        complement_in_place::<Digit, SHIFT>(&mut longest);
    };
    if shortest_sign.is_negative() {
        complement_in_place::<Digit, SHIFT>(&mut shortest);
    };
    let mut result = longest;
    for index in 0..shortest.len() {
        result[index] ^= shortest[index];
    }
    if shortest_sign.is_negative() {
        let digit_mask = to_digit_mask::<Digit>(SHIFT);
        for index in shortest.len()..result.len() {
            result[index] ^= digit_mask;
        }
    };
    let sign_is_negative = longest_sign.is_negative() ^ shortest_sign.is_negative();
    if sign_is_negative {
        result.push(to_digit_mask::<Digit>(SHIFT));
        complement_in_place::<Digit, SHIFT>(&mut result);
    }
    trim_leading_zeros(&mut result);
    (
        if sign_is_negative {
            -Sign::one()
        } else {
            Sign::one()
        } * to_digits_sign(&result),
        result,
    )
}

pub(super) fn checked_div_as_float<
    Digit: DivisibleAsFloatDigit<Output>,
    Output: Float,
    const SHIFT: usize,
>(
    dividend_digits: &[Digit],
    divisor_digits: &[Digit],
) -> Result<Output, CheckedDivAsFloatError> {
    if divisor_digits.len() == 1 && divisor_digits[0].is_zero() {
        return Err(CheckedDivAsFloatError::ZeroDivision);
    }
    if dividend_digits.len() == 1 && dividend_digits[0].is_zero() {
        return Ok(Output::zero());
    }
    let dividend_digits_count = dividend_digits.len();
    let divisor_digits_count = divisor_digits.len();
    let dividend_is_small = dividend_digits_count <= (Output::MANTISSA_DIGITS / SHIFT)
        || (dividend_digits_count == (Output::MANTISSA_DIGITS / SHIFT) + 1
            && (dividend_digits[(Output::MANTISSA_DIGITS / SHIFT)]
                >> (Output::MANTISSA_DIGITS % SHIFT))
                .is_zero());
    let divisor_is_small = divisor_digits_count <= (Output::MANTISSA_DIGITS / SHIFT)
        || (divisor_digits_count == (Output::MANTISSA_DIGITS / SHIFT) + 1
            && (divisor_digits[(Output::MANTISSA_DIGITS / SHIFT)]
                >> (Output::MANTISSA_DIGITS % SHIFT))
                .is_zero());
    if dividend_is_small && divisor_is_small {
        let reduced_dividend = reduce_digits_to_float::<Digit, Output, SHIFT>(dividend_digits);
        let reduced_divisor = reduce_digits_to_float::<Digit, Output, SHIFT>(divisor_digits);
        return Ok(reduced_dividend / reduced_divisor);
    }
    let digits_count_difference =
        (dividend_digits_count as isize) - (divisor_digits_count as isize);
    if digits_count_difference > (((usize::MAX / SHIFT) - 1) as isize) {
        return Err(CheckedDivAsFloatError::TooLarge);
    } else if digits_count_difference < 1isize - ((usize::MAX / SHIFT) as isize) {
        return Ok(Output::zero());
    }
    let bit_lengths_difference = digits_count_difference * (SHIFT as isize)
        + (((dividend_digits[dividend_digits.len() - 1].bit_length()) as isize)
            - (divisor_digits[divisor_digits.len() - 1].bit_length() as isize));
    if bit_lengths_difference > (Output::MAX_EXP as isize) {
        return Err(CheckedDivAsFloatError::TooLarge);
    } else if bit_lengths_difference
        < (Output::MIN_EXP as isize) - ((Output::MANTISSA_DIGITS as isize) - 1)
    {
        return Ok(Output::zero());
    }
    let shift = bit_lengths_difference.max(Output::MIN_EXP as isize)
        - (Output::MANTISSA_DIGITS as isize)
        - 2;
    let mut inexact = false;
    let mut quotient_digits = if shift <= 0 {
        let shift_digits = ((-shift) as usize) / SHIFT;
        if dividend_digits_count >= ((isize::MAX - 1) as usize) - shift_digits {
            return Err(CheckedDivAsFloatError::TooLarge);
        }
        let quotient_digits_count = dividend_digits_count + shift_digits + 1;
        let mut quotient_data = vec![Digit::zero(); quotient_digits_count];
        let remainder = shift_digits_left_in_place::<Digit, SHIFT>(
            dividend_digits,
            ((-shift) as usize) % SHIFT,
            &mut quotient_data[shift_digits..],
        );
        quotient_data[dividend_digits_count + shift_digits] = remainder;
        quotient_data
    } else {
        let mut shift_digits = (shift as usize) / SHIFT;
        let quotient_digits_count = dividend_digits_count - shift_digits;
        let mut quotient_data = vec![Digit::zero(); quotient_digits_count];
        let remainder = shift_digits_right_in_place::<Digit, SHIFT>(
            &dividend_digits[shift_digits..],
            (shift as usize) % SHIFT,
            &mut quotient_data,
        );
        if !remainder.is_zero() {
            inexact = true;
        }
        while !inexact && shift_digits > 0 {
            shift_digits -= 1;
            if !dividend_digits[shift_digits].is_zero() {
                inexact = true;
            }
        }
        quotient_data
    };
    trim_leading_zeros(&mut quotient_digits);
    if divisor_digits_count == 1 {
        let (next_quotient_digits, remainder) =
            div_rem_digits_by_digit::<Digit, SHIFT>(&quotient_digits, divisor_digits[0]);
        quotient_digits = next_quotient_digits;
        if !remainder.is_zero() {
            inexact = true;
        }
    } else {
        let (next_quotient_digits, remainder) =
            div_rem_two_or_more_digits::<Digit, SHIFT>(&quotient_digits, divisor_digits);
        quotient_digits = next_quotient_digits;
        if !to_digits_sign(&remainder).is_zero() {
            inexact = true;
        }
    }
    let quotient_bit_length = ((quotient_digits.len() - 1) * SHIFT
        + quotient_digits[quotient_digits.len() - 1].bit_length())
        as isize;
    let extra_bits = quotient_bit_length.max((Output::MIN_EXP as isize) - shift)
        - (Output::MANTISSA_DIGITS as isize);
    let mask = Digit::one() << ((extra_bits as usize) - 1);
    let mut quotient_low_digit = quotient_digits[0] | Digit::from(inexact as u8);
    if !(quotient_low_digit & mask).is_zero()
        && !(quotient_low_digit & (Digit::from(3u8) * mask - Digit::from(1u8))).is_zero()
    {
        quotient_low_digit += mask;
    }
    quotient_digits[0] = quotient_low_digit & !(Digit::from(2u8) * mask - Digit::from(1u8));
    let reduced_quotient = reduce_digits_to_float::<Digit, Output, SHIFT>(&quotient_digits);
    if shift + quotient_bit_length >= (Output::MAX_EXP as isize)
        && (shift + quotient_bit_length > (Output::MAX_EXP as isize)
            || reduced_quotient == Output::one().ldexp(quotient_bit_length as i32))
    {
        Err(CheckedDivAsFloatError::TooLarge)
    } else {
        Ok(reduced_quotient.ldexp(shift as i32))
    }
}

pub(super) fn checked_div<Digit: DivisibleDigit, const SHIFT: usize>(
    dividend_sign: Sign,
    dividend: &[Digit],
    divisor_sign: Sign,
    divisor: &[Digit],
) -> Option<(Sign, Vec<Digit>)> {
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
            dividend_sign * divisor_sign * to_digits_sign(&digits),
            digits,
        ))
    }
}

pub(super) fn checked_div_euclid<Digit: EuclidDivisibleDigit, const SHIFT: usize>(
    dividend_sign: Sign,
    dividend: &[Digit],
    divisor_sign: Sign,
    divisor: &[Digit],
) -> Option<(Sign, Vec<Digit>)> {
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
            let (digits, remainder) = div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
            (
                dividend_sign * divisor_sign * to_digits_sign(&digits),
                digits,
                !to_digits_sign(&remainder).is_zero(),
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

pub(super) fn checked_div_rem<Digit: DivisibleDigit, const SHIFT: usize>(
    dividend_sign: Sign,
    dividend: &[Digit],
    divisor_sign: Sign,
    divisor: &[Digit],
) -> Option<(Sign, Vec<Digit>, Sign, Vec<Digit>)> {
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero()
        || dividend.len() < divisor.len()
        || (dividend.len() == divisor.len()
            && dividend[dividend.len() - 1] < divisor[divisor.len() - 1])
    {
        Some((
            Sign::zero(),
            vec![Digit::zero(); 1],
            dividend_sign,
            dividend.to_vec(),
        ))
    } else if divisor.len() == 1 {
        let (quotient_digits, remainder_digit) =
            div_rem_digits_by_digit::<Digit, SHIFT>(dividend, divisor[0]);
        Some((
            dividend_sign * divisor_sign,
            quotient_digits,
            dividend_sign * ((!remainder_digit.is_zero()) as Sign),
            vec![remainder_digit],
        ))
    } else {
        let (quotient_digits, remainder_digits) =
            div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
        Some((
            dividend_sign * divisor_sign * to_digits_sign(&quotient_digits),
            quotient_digits,
            dividend_sign * to_digits_sign(&remainder_digits),
            remainder_digits,
        ))
    }
}

pub(super) fn checked_div_rem_euclid<Digit: EuclidDivisibleDigit, const SHIFT: usize>(
    dividend_sign: Sign,
    dividend: &[Digit],
    divisor_sign: Sign,
    divisor: &[Digit],
) -> Option<(Sign, Vec<Digit>, Sign, Vec<Digit>)> {
    let (mut quotient_sign, mut quotient, mut remainder_sign, mut remainder) =
        checked_div_rem::<Digit, SHIFT>(dividend_sign, dividend, divisor_sign, divisor)?;
    if (divisor_sign.is_negative() && remainder_sign.is_positive())
        || (divisor_sign.is_positive() && remainder_sign.is_negative())
    {
        (quotient_sign, quotient) = subtract_components::<Digit, SHIFT>(
            quotient_sign,
            &quotient,
            Sign::one(),
            &[Digit::one()],
        );
        (remainder_sign, remainder) =
            sum_components::<Digit, SHIFT>(remainder_sign, &remainder, divisor_sign, divisor);
    }
    Some((quotient_sign, quotient, remainder_sign, remainder))
}

pub(super) fn checked_rem<Digit: DivisibleDigit, const SHIFT: usize>(
    dividend_sign: Sign,
    dividend: &[Digit],
    divisor_sign: Sign,
    divisor: &[Digit],
) -> Option<(Sign, Vec<Digit>)> {
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero() || digits_lesser_than(dividend, divisor) {
        Some((dividend_sign, dividend.to_vec()))
    } else if divisor.len() == 1 {
        let (_, digit) = div_rem_digits_by_digit::<Digit, SHIFT>(dividend, divisor[0]);
        Some((dividend_sign * ((!digit.is_zero()) as Sign), vec![digit]))
    } else {
        let (_, digits) = div_rem_two_or_more_digits::<Digit, SHIFT>(dividend, divisor);
        Some((dividend_sign * to_digits_sign(&digits), digits))
    }
}

pub(super) fn checked_rem_euclid<Digit: EuclidDivisibleDigit, const SHIFT: usize>(
    dividend_sign: Sign,
    dividend: &[Digit],
    divisor_sign: Sign,
    divisor: &[Digit],
) -> Option<(Sign, Vec<Digit>)> {
    if divisor_sign.is_zero() {
        None
    } else if dividend_sign.is_zero() {
        Some((dividend_sign, dividend.to_vec()))
    } else if digits_lesser_than(dividend, divisor) {
        Some(
            if (dividend_sign.is_negative() && divisor_sign.is_positive())
                || (dividend_sign.is_positive() && divisor_sign.is_negative())
            {
                subtract_digits::<Digit, SHIFT>(dividend, divisor, dividend_sign)
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
            (dividend_sign * to_digits_sign(&digits), digits)
        };
        if (divisor_sign.is_negative() && sign.is_positive())
            || (divisor_sign.is_positive() && sign.is_negative())
        {
            (sign, digits) = subtract_digits::<Digit, SHIFT>(&digits, divisor, sign);
        }
        Some((sign, digits))
    }
}

pub(super) fn complement_in_place<Digit: ComplementableDigit, const SHIFT: usize>(
    digits: &mut [Digit],
) {
    let mut accumulator = Digit::one();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..digits.len() {
        accumulator += digits[index] ^ digit_mask;
        digits[index] = accumulator & digit_mask;
        accumulator >>= SHIFT;
    }
    debug_assert!(accumulator.is_zero());
}

pub(super) fn digits_from_finite_positive_improper_float<
    Digit: Copy + Zeroable,
    Value: Float,
    const SHIFT: usize,
>(
    value: Value,
) -> Vec<Digit>
where
    Value: FloatToInt<Digit> + From<Digit>,
{
    let (fraction, exponent) = value.frexp();
    let mut result = vec![Digit::zero(); ((exponent as usize) - 1) / SHIFT + 1];
    let mut fraction = fraction.ldexp((exponent - 1) % (SHIFT as i32) + 1);
    for index in (0..result.len()).rev() {
        let digit = unsafe { Value::to_int_unchecked(fraction) };
        result[index] = digit;
        fraction -= Value::from(digit);
        fraction = fraction.ldexp(SHIFT as i32);
    }
    result
}

#[inline]
pub(super) fn digits_lesser_than<Digit: PartialOrd>(left: &[Digit], right: &[Digit]) -> bool {
    left.len() < right.len()
        || left.len() == right.len() && left.iter().rev().lt(right.iter().rev())
}

pub(super) fn div_rem_digits_by_digit<Digit: DivisibleDigit, const SHIFT: usize>(
    dividend: &[Digit],
    divisor: Digit,
) -> (Vec<Digit>, Digit) {
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
    trim_leading_zeros(&mut quotient);
    (quotient, unsafe {
        Digit::try_from(remainder).unwrap_unchecked()
    })
}

pub(super) fn div_rem_two_or_more_digits<Digit: DivisibleDigit, const SHIFT: usize>(
    dividend: &[Digit],
    divisor: &[Digit],
) -> (Vec<Digit>, Vec<Digit>) {
    let dividend_digits_count = dividend.len();
    let divisor_digits_count = divisor.len();
    let mut dividend_normalized = vec![Digit::zero(); dividend_digits_count];
    let mut divisor_normalized = vec![Digit::zero(); divisor_digits_count];
    let shift = SHIFT - divisor[divisor.len() - 1].bit_length();
    shift_digits_left_in_place::<Digit, SHIFT>(divisor, shift, divisor_normalized.as_mut_slice());
    let accumulator = shift_digits_left_in_place::<Digit, SHIFT>(
        dividend,
        shift,
        dividend_normalized.as_mut_slice(),
    );
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
    trim_leading_zeros(&mut quotient);
    let mut remainder = divisor_normalized;
    shift_digits_right_in_place::<Digit, SHIFT>(
        &dividend_normalized[..divisor_digits_count],
        shift,
        remainder.as_mut_slice(),
    );
    trim_leading_zeros(&mut remainder);
    (quotient, remainder)
}

pub(super) fn fraction_exponent_digits<
    Digit: BinaryDigitConvertibleToFloat<Fraction>,
    Fraction: Float,
    const SHIFT: usize,
>(
    digits: &[Digit],
) -> Option<(Fraction, i32)> {
    let mut result_digits =
        vec![Digit::zero(); 2usize + (Fraction::MANTISSA_DIGITS + 1usize) / SHIFT];
    let size = digits.len();
    let mut bits_count = digits[digits.len() - 1].bit_length();
    if size > (usize::MAX - 1) / SHIFT
        && (size > (usize::MAX - 1) / SHIFT + 1 || bits_count > (usize::MAX - 1) % SHIFT + 1)
    {
        return None;
    }
    bits_count += (size - 1) * SHIFT;
    let mut result_digits_count = if bits_count <= Fraction::MANTISSA_DIGITS + 2 {
        let shift_digits = (Fraction::MANTISSA_DIGITS + 2 - bits_count) / SHIFT;
        let shift_bits = (Fraction::MANTISSA_DIGITS + 2 - bits_count) % SHIFT;
        let mut result_size = shift_digits;
        let remainder = shift_digits_left_in_place::<Digit, SHIFT>(
            digits,
            shift_bits,
            &mut result_digits[result_size..],
        );
        result_size += size;
        result_digits[result_size] = remainder;
        result_size += 1;
        result_size
    } else {
        let mut shift_digits = (bits_count - Fraction::MANTISSA_DIGITS - 2) / SHIFT;
        let shift_bits = (bits_count - Fraction::MANTISSA_DIGITS - 2) % SHIFT;
        let remainder = shift_digits_right_in_place::<Digit, SHIFT>(
            &digits[shift_digits..],
            shift_bits,
            &mut result_digits,
        );
        let result_size = size - shift_digits;
        if remainder.is_zero() {
            while shift_digits > 0 {
                shift_digits -= 1;
                if !digits[shift_digits].is_zero() {
                    result_digits[0] |= Digit::one();
                    break;
                }
            }
        } else {
            result_digits[0] |= Digit::one();
        }
        result_size
    };
    const HALF_EVEN_CORRECTION: [i8; 8] = [0, -1, -2, 1, 0, -1, 2, 1];
    result_digits[0] = unsafe {
        Digit::try_from(
            OppositionOf::<Digit>::try_from(result_digits[0]).unwrap_unchecked()
                + OppositionOf::<Digit>::from(
                    HALF_EVEN_CORRECTION
                        [usize::try_from(result_digits[0] & Digit::from(7u8)).unwrap_unchecked()],
                ),
        )
        .unwrap_unchecked()
    };
    result_digits_count -= 1;
    let mut fraction = Fraction::from(result_digits[result_digits_count]);
    while result_digits_count > 0 {
        result_digits_count -= 1;
        fraction = fraction * Fraction::from((1usize << SHIFT) as f32)
            + Fraction::from(result_digits[result_digits_count]);
    }
    fraction /= Fraction::from((1u64 << (Fraction::MANTISSA_DIGITS + 2)) as f32);
    if fraction.is_one() {
        if bits_count == usize::MAX {
            return None;
        }
        fraction = Fraction::from(0.5);
        bits_count += 1;
    }
    let exponent = unsafe { i32::try_from(bits_count).unwrap_unchecked() };
    if exponent > Fraction::MAX_EXP {
        None
    } else {
        Some((fraction, exponent))
    }
}

pub(super) fn invert_components<Digit: InvertibleDigit, const SHIFT: usize>(
    sign: Sign,
    digits: &[Digit],
) -> (Sign, Vec<Digit>) {
    let (sign, digits) = sum_components::<Digit, SHIFT>(sign, digits, Sign::one(), &[Digit::one()]);
    (-sign, digits)
}

pub(super) fn multiply_digits<Digit: MultiplicativeDigit, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
) -> Vec<Digit> {
    let (longest, shortest) = if first.len() < second.len() {
        (&second, &first)
    } else {
        (&first, &second)
    };
    const KARATSUBA_CUTOFF: usize = 70;
    const KARATSUBA_SQUARE_CUTOFF: usize = KARATSUBA_CUTOFF * 2;
    if shortest.len()
        <= if shortest.as_ptr() == longest.as_ptr() {
            KARATSUBA_SQUARE_CUTOFF
        } else {
            KARATSUBA_CUTOFF
        }
    {
        if shortest.len() == 1 && shortest[0].is_zero() {
            vec![Digit::zero()]
        } else {
            multiply_digits_plain::<Digit, SHIFT>(*shortest, *longest)
        }
    } else if 2 * shortest.len() <= longest.len() {
        multiply_digits_lopsided::<Digit, SHIFT>(*shortest, *longest)
    } else {
        let shift = longest.len() >> 1;
        let (shortest_high, shortest_low) = split_digits(*shortest, shift);
        let (longest_high, longest_low) = if shortest.as_ptr() == longest.as_ptr() {
            (shortest_high.clone(), shortest_low.clone())
        } else {
            split_digits(*longest, shift)
        };
        let mut result = vec![Digit::zero(); shortest.len() + longest.len()];
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
        trim_leading_zeros(&mut result);
        result
    }
}

fn multiply_digits_lopsided<Digit: MultiplicativeDigit, const SHIFT: usize>(
    shortest: &[Digit],
    longest: &[Digit],
) -> Vec<Digit> {
    let shortest_size = shortest.len();
    let mut longest_size = longest.len();
    let mut result = vec![Digit::zero(); shortest_size + longest_size];
    let mut processed_digits_count = 0;
    while longest_size > 0 {
        let step_digits_count = longest_size.min(shortest_size);
        let product = multiply_digits::<Digit, SHIFT>(
            shortest,
            &longest[processed_digits_count..processed_digits_count + step_digits_count].to_vec(),
        );
        sum_digits_in_place::<Digit, SHIFT>(&mut result[processed_digits_count..], &product);
        longest_size -= step_digits_count;
        processed_digits_count += step_digits_count;
    }
    trim_leading_zeros(&mut result);
    result
}

fn multiply_digits_plain<Digit: MultiplicativeDigit, const SHIFT: usize>(
    shortest: &[Digit],
    longest: &[Digit],
) -> Vec<Digit> {
    let mut result = vec![Digit::zero(); shortest.len() + longest.len()];
    let digit_mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
    if shortest.as_ptr() == longest.as_ptr() {
        for index in 0..shortest.len() {
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
        for index in 0..shortest.len() {
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
    trim_leading_zeros(&mut result);
    result
}

pub(super) fn negate_digits(digits: &mut [u8]) {
    let mut carry = true;
    for digit in digits {
        *digit = !*digit;
        if carry {
            *digit = digit.wrapping_add(1);
            carry = digit.is_zero();
        }
    }
}

fn non_binary_digits_to_greater_binary_base<
    SourceDigit: NonBinaryDigitConvertibleToBinary<TargetDigit>,
    TargetDigit,
    const TARGET_SHIFT: usize,
>(
    source: &[SourceDigit],
    source_base: usize,
) -> Vec<TargetDigit> {
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<TargetDigit>>(TARGET_SHIFT);
    static mut BASES_LOGS: [f64; 37] = [0.0; 37];
    static mut INFIMUM_BASES_EXPONENTS: [usize; 37] = [0; 37];
    static mut INFIMUM_BASES_POWERS: [usize; 37] = [0; 37];
    if unsafe { BASES_LOGS[source_base] } == 0.0 {
        let bases_log = (source_base as f64).ln() / ((1usize << TARGET_SHIFT) as f64).ln();
        unsafe { BASES_LOGS[source_base] = bases_log };
        let mut infimum_base_power = source_base;
        let mut infimum_base_exponent: usize = 1;
        loop {
            let candidate: usize = infimum_base_power * source_base;
            if candidate > 1usize << TARGET_SHIFT {
                break;
            }
            infimum_base_power = candidate;
            infimum_base_exponent += 1;
        }
        unsafe { INFIMUM_BASES_POWERS[source_base] = infimum_base_power };
        unsafe { INFIMUM_BASES_EXPONENTS[source_base] = infimum_base_exponent };
    }
    let digits_count_upper_bound = (source.len() as f64) * unsafe { BASES_LOGS[source_base] } + 1.0;
    let mut result = Vec::<TargetDigit>::with_capacity(digits_count_upper_bound as usize);
    let infimum_base_exponent = unsafe { INFIMUM_BASES_EXPONENTS[source_base] };
    let infimum_base_power = unsafe { INFIMUM_BASES_POWERS[source_base] };
    let mut reversed_source = source.iter().rev();
    while let Some(&digit) = reversed_source.next() {
        let mut accumulator = DoublePrecisionOf::<TargetDigit>::from(digit);
        let mut base_exponent: usize = 1;
        while base_exponent < infimum_base_exponent {
            if let Some(&digit) = reversed_source.next() {
                base_exponent += 1;
                unsafe {
                    accumulator = accumulator
                        * DoublePrecisionOf::<TargetDigit>::try_from(source_base)
                            .unwrap_unchecked()
                        + DoublePrecisionOf::<TargetDigit>::from(digit);
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
            accumulator += DoublePrecisionOf::<TargetDigit>::from(*result_position)
                * unsafe {
                    DoublePrecisionOf::<TargetDigit>::try_from(base_power).unwrap_unchecked()
                };
            *result_position = unsafe {
                TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked()
            };
            accumulator >>= TARGET_SHIFT;
        }
        if !accumulator.is_zero() {
            result.push(unsafe { TargetDigit::try_from(accumulator).unwrap_unchecked() });
        }
    }
    if result.is_empty() {
        result.push(TargetDigit::zero());
    }
    result
}

fn non_binary_digits_to_lesser_binary_base<
    SourceDigit: NonBinaryDigitConvertibleToBinary<TargetDigit>,
    TargetDigit,
    const TARGET_SHIFT: usize,
>(
    source: &[SourceDigit],
    source_base: usize,
) -> Vec<TargetDigit> {
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<TargetDigit>>(TARGET_SHIFT);
    static mut BASES_LOGS: [f64; 37] = [0.0; 37];
    if unsafe { BASES_LOGS[source_base] } == 0.0 {
        let bases_log = (source_base as f64).ln() / ((1usize << TARGET_SHIFT) as f64).ln();
        unsafe { BASES_LOGS[source_base] = bases_log };
    }
    let digits_count_upper_bound = (source.len() as f64) * unsafe { BASES_LOGS[source_base] } + 1.0;
    let mut result = Vec::<TargetDigit>::with_capacity(digits_count_upper_bound as usize);
    let source_base =
        unsafe { DoublePrecisionOf::<TargetDigit>::try_from(source_base).unwrap_unchecked() };
    for &digit in source.iter().rev() {
        let mut accumulator = DoublePrecisionOf::<TargetDigit>::from(digit);
        for result_position in result.iter_mut() {
            accumulator += DoublePrecisionOf::<TargetDigit>::from(*result_position) * source_base;
            *result_position = unsafe {
                TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked()
            };
            accumulator >>= TARGET_SHIFT;
        }
        while !accumulator.is_zero() {
            result.push(unsafe {
                TargetDigit::try_from(accumulator & target_digit_mask).unwrap_unchecked()
            });
            accumulator >>= TARGET_SHIFT;
        }
    }
    if result.is_empty() {
        result.push(TargetDigit::zero());
    }
    result
}

pub(super) fn reduce_digits<Digit: ReducibleTo<Output>, Output, const SHIFT: usize>(
    digits: &[Digit],
) -> Output {
    let mut result = Output::zero();
    for &digit in digits.iter().rev() {
        result = (result << SHIFT) | Output::from(digit);
    }
    result
}

pub(super) fn maybe_reduce_digits<Digit: MaybeReducibleTo<Output>, Output, const SHIFT: usize>(
    digits: &[Digit],
) -> Option<Output> {
    let mut result = Output::zero();
    for &digit in digits.iter().rev() {
        result = result.checked_shl(SHIFT)? | Output::try_from(digit).ok()?;
    }
    Some(result)
}

pub(super) fn reduce_digits_to_float<Digit, Output, const SHIFT: usize>(digits: &[Digit]) -> Output
where
    Digit: Copy,
    Output: Float + From<Digit>,
{
    let mut result = Output::zero();
    for &digit in digits.iter().rev() {
        result = result * Output::from((1u64 << SHIFT) as f32) + Output::from(digit);
    }
    result
}

pub(super) fn primitive_shift_digits_left<Digit: ShiftableLeftDigit, const SHIFT: usize>(
    digits: &[Digit],
    shift_quotient: usize,
    shift_remainder: Digit,
) -> Option<Vec<Digit>> {
    let mut result = Vec::<Digit>::new();
    result
        .try_reserve_exact(shift_quotient + ((!shift_remainder.is_zero()) as usize) + digits.len())
        .ok()?;
    for _ in 0..shift_quotient {
        result.push(Digit::zero());
    }
    let mut accumulator = DoublePrecisionOf::<Digit>::zero();
    let digit_mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
    for &digit in digits {
        accumulator |= DoublePrecisionOf::<Digit>::from(digit) << shift_remainder;
        result.push(unsafe { Digit::try_from(accumulator & digit_mask).unwrap_unchecked() });
        accumulator >>= SHIFT;
    }
    if !shift_remainder.is_zero() {
        result.push(unsafe { Digit::try_from(accumulator).unwrap_unchecked() });
    }
    trim_leading_zeros(&mut result);
    Some(result)
}

pub(super) fn shift_digits_left<Digit: ShiftableLeftDigit, const SHIFT: usize>(
    base: &[Digit],
    shift: &[Digit],
) -> Result<Vec<Digit>, ShlError> {
    let (shift_quotient_digits, shift_remainder) =
        div_rem_digits_by_digit::<Digit, SHIFT>(&shift, unsafe {
            Digit::try_from(SHIFT).unwrap_unchecked()
        });
    let shift_quotient = maybe_reduce_digits::<Digit, usize, SHIFT>(&shift_quotient_digits)
        .ok_or(ShlError::TooLarge)?;
    if shift_quotient >= usize::MAX / size_of::<Digit>() {
        Err(ShlError::TooLarge)
    } else {
        primitive_shift_digits_left::<Digit, SHIFT>(&base, shift_quotient, shift_remainder)
            .ok_or(ShlError::OutOfMemory)
    }
}

fn shift_digits_left_in_place<Digit: ShiftableInPlaceDigit, const SHIFT: usize>(
    input: &[Digit],
    shift: usize,
    output: &mut [Digit],
) -> Digit {
    let mut accumulator: Digit = Digit::zero();
    let mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
    for index in 0..input.len() {
        let step = (DoublePrecisionOf::<Digit>::from(input[index]) << shift)
            | DoublePrecisionOf::<Digit>::from(accumulator);
        output[index] = unsafe { Digit::try_from(step & mask).unwrap_unchecked() };
        accumulator = unsafe { Digit::try_from(step >> SHIFT).unwrap_unchecked() };
    }
    accumulator
}

fn shift_digits_right_in_place<Digit: ShiftableInPlaceDigit, const SHIFT: usize>(
    input: &[Digit],
    shift: usize,
    output: &mut [Digit],
) -> Digit {
    let mut accumulator = Digit::zero();
    let mask = to_digit_mask::<DoublePrecisionOf<Digit>>(shift);
    for index in (0..input.len()).rev() {
        let step = (DoublePrecisionOf::<Digit>::from(accumulator) << SHIFT)
            | DoublePrecisionOf::<Digit>::from(input[index]);
        accumulator = unsafe { Digit::try_from(step & mask).unwrap_unchecked() };
        output[index] = unsafe { Digit::try_from(step >> shift).unwrap_unchecked() };
    }
    accumulator
}

pub(super) fn primitive_shift_digits_right<Digit: ShiftableRightDigit, const SHIFT: usize>(
    digits: &[Digit],
    shift_quotient: usize,
    shift_remainder: Digit,
) -> Vec<Digit> {
    if digits.len() <= shift_quotient {
        return vec![Digit::zero()];
    }
    let result_digits_count = digits.len() - shift_quotient;
    let high_shift = SHIFT - unsafe { usize::try_from(shift_remainder).unwrap_unchecked() };
    let low_mask = to_digit_mask::<Digit>(high_shift);
    let high_mask = to_digit_mask::<Digit>(SHIFT) ^ low_mask;
    let mut result = vec![Digit::zero(); result_digits_count];
    let mut position = shift_quotient;
    for index in 0..result_digits_count {
        result[index] = (digits[position] >> shift_remainder) & low_mask;
        if index + 1 < result_digits_count {
            result[index] |= (digits[position + 1] << high_shift) & high_mask;
        }
        position += 1;
    }
    trim_leading_zeros(&mut result);
    result
}

pub(super) fn shift_digits_right<Digit: ShiftableRightDigit, const SHIFT: usize>(
    base_sign: Sign,
    base: &[Digit],
    shift: &[Digit],
) -> (Sign, Vec<Digit>) {
    let (shift_quotient_digits, shift_remainder) =
        div_rem_digits_by_digit::<Digit, SHIFT>(&shift, unsafe {
            Digit::try_from(SHIFT).unwrap_unchecked()
        });
    let shift_quotient = maybe_reduce_digits::<Digit, usize, SHIFT>(&shift_quotient_digits)
        .unwrap_or(usize::MAX / size_of::<Digit>());
    if shift_quotient >= usize::MAX / size_of::<Digit>() {
        if base_sign.is_negative() {
            (-Sign::one(), vec![Digit::one(); 1])
        } else {
            (Sign::zero(), vec![Digit::zero(); 1])
        }
    } else if base_sign.is_negative() {
        let (inverted_sign, inverted_digits) = invert_components::<Digit, SHIFT>(base_sign, &base);
        let digits = primitive_shift_digits_right::<Digit, SHIFT>(
            &inverted_digits,
            shift_quotient,
            shift_remainder,
        );
        invert_components::<Digit, SHIFT>(inverted_sign * to_digits_sign(&digits), &digits)
    } else {
        let digits =
            primitive_shift_digits_right::<Digit, SHIFT>(&base, shift_quotient, shift_remainder);
        (base_sign * to_digits_sign(&digits), digits)
    }
}

fn split_digits<Digit>(digits: &[Digit], size: usize) -> (Vec<Digit>, Vec<Digit>)
where
    Digit: Clone + Zeroable,
{
    let (low, high) = digits.split_at(digits.len().min(size));
    let (mut low, mut high) = (low.to_vec(), high.to_vec());
    trim_leading_zeros(&mut high);
    trim_leading_zeros(&mut low);
    (high, low)
}

pub(super) fn subtract_components<Digit: AdditiveDigit, const SHIFT: usize>(
    minuend_sign: Sign,
    minuend: &[Digit],
    subtrahend_sign: Sign,
    subtrahend: &[Digit],
) -> (Sign, Vec<Digit>) {
    if minuend_sign.is_negative() {
        if subtrahend_sign.is_negative() {
            subtract_digits::<Digit, SHIFT>(subtrahend, minuend, Sign::one())
        } else {
            (
                -Sign::one(),
                sum_digits::<Digit, SHIFT>(minuend, subtrahend),
            )
        }
    } else if subtrahend_sign.is_negative() {
        (Sign::one(), sum_digits::<Digit, SHIFT>(minuend, subtrahend))
    } else {
        subtract_digits::<Digit, SHIFT>(minuend, subtrahend, Sign::one())
    }
}

fn subtract_digits<Digit: AdditiveDigit, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
    mut sign: Sign,
) -> (Sign, Vec<Digit>) {
    let mut longest = &first;
    let mut shortest = &second;
    let mut longest_size = longest.len();
    let mut shortest_size = shortest.len();
    match longest_size.cmp(&shortest_size) {
        Ordering::Less => {
            (longest, shortest) = (shortest, longest);
            (longest_size, shortest_size) = (shortest_size, longest_size);
            sign = -sign;
        }
        Ordering::Equal => {
            let mut index = shortest_size;
            loop {
                index -= 1;
                if index == 0 || longest[index] != shortest[index] {
                    break;
                }
            }
            if index == 0 && longest[0] == shortest[0] {
                return (Sign::zero(), vec![Digit::zero()]);
            }
            if longest[index] < shortest[index] {
                (longest, shortest) = (shortest, longest);
                sign = -sign;
            }
            longest_size = index + 1;
            shortest_size = index + 1;
        }
        _ => {}
    };
    let mut result = Vec::<Digit>::with_capacity(longest_size);
    let mut accumulator = Digit::zero();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..shortest_size {
        accumulator = longest[index]
            .wrapping_sub(shortest[index])
            .wrapping_sub(accumulator);
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
        accumulator &= Digit::one();
    }
    for index in shortest_size..longest_size {
        accumulator = longest[index].wrapping_sub(accumulator);
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
        accumulator &= Digit::one();
    }
    trim_leading_zeros(&mut result);
    (sign, result)
}

fn subtract_digits_in_place<Digit: AdditiveDigit, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &[Digit],
) -> Digit {
    let mut accumulator = Digit::zero();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..shortest.len() {
        accumulator = longest[index]
            .wrapping_sub(shortest[index])
            .wrapping_sub(accumulator);
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

pub(super) fn sum_components<Digit: AdditiveDigit, const SHIFT: usize>(
    first_sign: Sign,
    first: &[Digit],
    second_sign: Sign,
    second: &[Digit],
) -> (Sign, Vec<Digit>) {
    if first_sign.is_negative() {
        if second_sign.is_negative() {
            (-Sign::one(), sum_digits::<Digit, SHIFT>(first, second))
        } else {
            subtract_digits::<Digit, SHIFT>(second, first, Sign::one())
        }
    } else if second_sign.is_negative() {
        subtract_digits::<Digit, SHIFT>(first, second, Sign::one())
    } else {
        (
            first_sign.max(second_sign),
            sum_digits::<Digit, SHIFT>(first, second),
        )
    }
}

fn sum_digits<Digit: AdditiveDigit, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
) -> Vec<Digit> {
    let (longest, shortest) = if first.len() < second.len() {
        (&second, &first)
    } else {
        (&first, &second)
    };
    let mut result = Vec::<Digit>::with_capacity(longest.len() + 1);
    let mut accumulator: Digit = Digit::zero();
    let digit_mask = to_digit_mask::<Digit>(SHIFT);
    for index in 0..shortest.len() {
        accumulator += longest[index] + shortest[index];
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
    }
    for index in shortest.len()..longest.len() {
        accumulator += longest[index];
        result.push(accumulator & digit_mask);
        accumulator >>= SHIFT;
    }
    result.push(accumulator);
    trim_leading_zeros(&mut result);
    result
}

fn sum_digits_in_place<Digit: AdditiveDigit, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &[Digit],
) -> Digit {
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

#[inline]
pub(super) fn to_digit_mask<Digit: MaskableDigit>(shift: usize) -> Digit {
    (Digit::one() << shift) - Digit::one()
}

#[inline]
pub(super) fn to_digits_sign<Digit: Zeroable>(digits: &[Digit]) -> Sign {
    (digits.len() > 1 || !digits[0].is_zero()) as Sign
}

pub(super) fn to_gcd<Digit: GcdDigit, const SHIFT: usize>(
    first: Vec<Digit>,
    second: Vec<Digit>,
) -> (Sign, Vec<Digit>) {
    let (mut largest, mut smallest) = if digits_lesser_than(&first, &second) {
        (second, first)
    } else {
        (first, second)
    };
    loop {
        let largest_digits_count = largest.len();
        if largest_digits_count <= 2 {
            break;
        }
        let smallest_digits_count = smallest.len();
        if smallest_digits_count == 1 && smallest[0].is_zero() {
            return (Sign::one(), largest);
        }
        let highest_digit_bit_length = largest[largest.len() - 1].bit_length();
        let mut largest_leading_bits =
            (OppositionOf::<DoublePrecisionOf<Digit>>::from(largest[largest_digits_count - 1])
                << (2 * SHIFT - highest_digit_bit_length))
                | (OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    largest[largest_digits_count - 2],
                ) << (SHIFT - highest_digit_bit_length))
                | OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    largest[largest_digits_count - 3] >> highest_digit_bit_length,
                );
        let mut smallest_leading_bits = if smallest_digits_count >= largest_digits_count - 2 {
            OppositionOf::<DoublePrecisionOf<Digit>>::from(
                smallest[largest_digits_count - 3] >> highest_digit_bit_length,
            )
        } else {
            OppositionOf::<DoublePrecisionOf<Digit>>::zero()
        } | if smallest_digits_count >= largest_digits_count - 1 {
            OppositionOf::<DoublePrecisionOf<Digit>>::from(smallest[largest_digits_count - 2])
                << (SHIFT - highest_digit_bit_length)
        } else {
            OppositionOf::<DoublePrecisionOf<Digit>>::zero()
        } | if smallest_digits_count >= largest_digits_count {
            OppositionOf::<DoublePrecisionOf<Digit>>::from(smallest[largest_digits_count - 1])
                << (2 * SHIFT - highest_digit_bit_length)
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
            let next_smallest_leading_bits = largest_leading_bits - scale * smallest_leading_bits;
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
            (largest, smallest) = if smallest_digits_count == 1 {
                let (_, remainder) = div_rem_digits_by_digit::<Digit, SHIFT>(&largest, smallest[0]);
                (smallest, vec![remainder])
            } else {
                let (_, remainder) =
                    div_rem_two_or_more_digits::<Digit, SHIFT>(&largest, &smallest);
                (smallest, remainder)
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
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest[index]))
                - (second_coefficient
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(smallest[index]));
            next_smallest_accumulator = next_smallest_accumulator
                + (fourth_coefficient
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(smallest[index]))
                - (third_coefficient
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest[index]));
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
            next_largest_accumulator +=
                first_coefficient * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest[index]);
            next_smallest_accumulator -=
                third_coefficient * OppositionOf::<DoublePrecisionOf<Digit>>::from(largest[index]);
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
        largest = next_largest_digits;
        smallest = next_smallest_digits;
    }
    let reduced_result =
        reduce_digits::<Digit, DoublePrecisionOf<Digit>, SHIFT>(&largest).gcd(reduce_digits::<
            Digit,
            DoublePrecisionOf<Digit>,
            SHIFT,
        >(&smallest));
    if reduced_result.is_zero() {
        (Sign::zero(), vec![Digit::zero(); 1])
    } else {
        (
            Sign::one(),
            non_zero_value_to_digits::<DoublePrecisionOf<Digit>, Digit, SHIFT>(reduced_result),
        )
    }
}

pub(super) fn trim_leading_zeros<Digit>(digits: &mut Vec<Digit>)
where
    Digit: Zeroable,
{
    let mut digits_count = digits.len();
    while digits_count > 1 && digits[digits_count - 1].is_zero() {
        digits_count -= 1;
    }
    if digits_count != digits.len() {
        digits.truncate(digits_count);
    }
}

pub(super) fn non_zero_value_to_digits<
    Source,
    Digit: ConstructibleFrom<Source>,
    const SHIFT: usize,
>(
    value: Source,
) -> Vec<Digit> {
    if size_of::<Source>() < size_of::<Digit>()
        || (size_of::<Source>() == size_of::<Digit>()
            && crate::contracts::is_signed::<Source>()
            && crate::contracts::is_unsigned::<Digit>())
    {
        let mut value = if crate::contracts::is_signed::<Source>() {
            let value = unsafe { OppositionOf::<Source>::try_from(value).unwrap_unchecked() };
            unsafe {
                Digit::try_from(
                    Source::try_from(if value.is_negative() { -value } else { value })
                        .unwrap_unchecked(),
                )
                .unwrap_unchecked()
            }
        } else {
            unsafe { Digit::try_from(value).unwrap_unchecked() }
        };
        let mut digits = Vec::<Digit>::new();
        let digit_mask = to_digit_mask::<Digit>(SHIFT);
        while !value.is_zero() {
            digits.push(value & digit_mask);
            value >>= SHIFT;
        }
        digits
    } else {
        let mut value = if crate::contracts::is_signed::<Source>() {
            let value = unsafe { OppositionOf::<Source>::try_from(value).unwrap_unchecked() };
            if value.is_negative() {
                unsafe { Source::try_from(-value).unwrap_unchecked() }
            } else {
                unsafe { Source::try_from(value).unwrap_unchecked() }
            }
        } else {
            value
        };
        let mut digits = Vec::<Digit>::new();
        let digit_mask = to_digit_mask::<Source>(SHIFT);
        while !value.is_zero() {
            digits.push(unsafe { Digit::try_from(value & digit_mask).unwrap_unchecked() });
            value >>= SHIFT;
        }
        digits
    }
}

#[inline]
pub(super) fn value_to_sign<Source>(value: Source) -> Sign
where
    Source: Oppose + Zeroable,
    OppositionOf<Source>: TryFrom<Source>,
{
    if value.is_zero() {
        Sign::zero()
    } else {
        non_zero_value_to_sign(value)
    }
}

#[inline]
pub(super) fn non_zero_value_to_sign<Source>(value: Source) -> Sign
where
    Source: Oppose,
    OppositionOf<Source>: TryFrom<Source>,
{
    if crate::contracts::is_signed::<Source>()
        && unsafe { OppositionOf::<Source>::try_from(value).unwrap_unchecked() }.is_negative()
    {
        -Sign::one()
    } else {
        Sign::one()
    }
}

#[inline]
fn floor_log2<T: BitLength<Output = usize> + Zeroable>(value: T) -> usize {
    debug_assert!(!value.is_zero());
    value.bit_length() - 1
}
