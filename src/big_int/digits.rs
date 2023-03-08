use std::cmp::Ordering;
use std::convert::TryFrom;
use std::mem::{size_of, transmute};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub, SubAssign,
};

use traiter::numbers::{
    BitLength, CheckedShl, FloatInfo, Floor, FractExp, Gcd, LoadExp,
    RemEuclid, Signed, Unitary, Zeroable,
};

use crate::contracts::{is_signed, is_unsigned};
use crate::traits::{
    DoublePrecision, DoublePrecisionOf, HasSignBit, MantissaDigits, MaxExp,
    MinExp, Oppose, OppositionOf, WrappingSub,
};

use super::types::{CheckedDivAsFloatError, ShlError, Sign};

pub trait BaseFromBinaryDigits<Source>: Sized {
    fn base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_base: usize,
    ) -> Vec<Self>;
}

impl<
        Source,
        Target: BinaryBaseFromBinaryDigits<Source>
            + NonBinaryBaseFromBinaryDigits<Source>,
    > BaseFromBinaryDigits<Source> for Target
{
    fn base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_base: usize,
    ) -> Vec<Self> {
        if target_base & (target_base - 1) == 0 {
            Self::binary_base_from_binary_digits(
                source,
                source_bitness,
                floor_log2::<usize>(target_base),
            )
        } else {
            Self::non_binary_base_from_binary_digits(
                source,
                source_bitness,
                target_base,
            )
        }
    }
}

pub(super) trait BinaryBaseFromDigits<Source>: Sized {
    fn binary_base_from_digits<const TARGET_BITNESS: usize>(
        source: &[Source],
        source_base: usize,
    ) -> Vec<Self>;
}

impl<
        Source,
        Target: BinaryBaseFromBinaryDigits<Source>
            + GreaterBinaryBaseFromNonBinaryDigits<Source>
            + LesserBinaryBaseFromNonBinaryDigits<Source>,
    > BinaryBaseFromDigits<Source> for Target
{
    fn binary_base_from_digits<const TARGET_BITNESS: usize>(
        source: &[Source],
        source_base: usize,
    ) -> Vec<Self> {
        if source_base & (source_base - 1) == 0 {
            Self::binary_base_from_binary_digits(
                source,
                floor_log2::<usize>(source_base),
                TARGET_BITNESS,
            )
        } else if source_base < (1 << TARGET_BITNESS) {
            Self::greater_binary_base_from_non_binary_digits::<TARGET_BITNESS>(
                source,
                source_base,
            )
        } else {
            Self::lesser_binary_base_from_non_binary_digits::<TARGET_BITNESS>(
                source,
                source_base,
            )
        }
    }
}

pub(super) trait BinaryBaseFromBinaryDigits<Source>: Sized {
    fn binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_bitness: usize,
    ) -> Vec<Self>;
}

impl<
        Source: Copy,
        Target: GreaterBinaryBaseFromBinaryDigits<Source>
            + LesserBinaryBaseFromBinaryDigits<Source>
            + TryFrom<Source>,
    > BinaryBaseFromBinaryDigits<Source> for Target
{
    fn binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_bitness: usize,
    ) -> Vec<Self> {
        match target_bitness.cmp(&source_bitness) {
            Ordering::Equal => source
                .iter()
                .map(|&digit| unsafe {
                    Self::try_from(digit).unwrap_unchecked()
                })
                .collect(),
            Ordering::Greater => Self::greater_binary_base_from_binary_digits(
                source,
                source_bitness,
                target_bitness,
            ),
            Ordering::Less => Self::lesser_binary_base_from_binary_digits(
                source,
                source_bitness,
                target_bitness,
            ),
        }
    }
}

pub(super) trait NonBinaryBaseFromBinaryDigits<Source>: Sized {
    fn non_binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_base: usize,
    ) -> Vec<Self>;
}

impl<
        Source: Copy,
        Target: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Target>> + Zeroable,
    > NonBinaryBaseFromBinaryDigits<Source> for Target
where
    DoublePrecisionOf<Target>: BitOr<Output = DoublePrecisionOf<Target>>
        + Copy
        + Div<Output = DoublePrecisionOf<Target>>
        + DivAssign
        + Mul<Output = DoublePrecisionOf<Target>>
        + Shl<usize, Output = DoublePrecisionOf<Target>>
        + ShlAssign<usize>
        + Sub<Output = DoublePrecisionOf<Target>>
        + From<Source>
        + RemEuclid<Output = DoublePrecisionOf<Target>>
        + TryFrom<usize>
        + Zeroable,
{
    fn non_binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_base: usize,
    ) -> Vec<Self> {
        let result_max_digits_count: usize = 1
            + ((((source.len() * source_bitness) as f64)
                / (target_base as f64).log2()) as usize);
        let mut result = Vec::<Self>::with_capacity(result_max_digits_count);
        let target_base = unsafe {
            DoublePrecisionOf::<Self>::try_from(target_base).unwrap_unchecked()
        };
        for digit in source.iter().rev() {
            let mut accumulator: DoublePrecisionOf<Self> =
                DoublePrecisionOf::<Self>::from(*digit);
            for result_position in &mut result {
                let step: DoublePrecisionOf<Self> =
                    (<DoublePrecisionOf<Self> as From<Self>>::from(
                        *result_position,
                    ) << source_bitness)
                        | accumulator;
                accumulator = step / target_base;
                *result_position = unsafe {
                    Self::try_from(step - accumulator * target_base)
                        .unwrap_unchecked()
                };
            }
            while !accumulator.is_zero() {
                result.push(unsafe {
                    Self::try_from(accumulator.rem_euclid(target_base))
                        .unwrap_unchecked()
                });
                accumulator /= target_base;
            }
        }
        if result.is_empty() {
            result.push(Self::zero());
        }
        result
    }
}

trait GreaterBinaryBaseFromBinaryDigits<Source>: Sized {
    fn greater_binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_bitness: usize,
    ) -> Vec<Self>;
}

impl<
        Source: Copy,
        Target: DoublePrecision + TryFrom<DoublePrecisionOf<Target>>,
    > GreaterBinaryBaseFromBinaryDigits<Source> for Target
where
    DoublePrecisionOf<Target>: BitAnd<Output = DoublePrecisionOf<Target>>
        + BitOr<Output = DoublePrecisionOf<Target>>
        + BitOrAssign
        + Copy
        + From<Source>
        + Shl<usize, Output = DoublePrecisionOf<Target>>
        + ShlAssign<usize>
        + Shr<usize, Output = DoublePrecisionOf<Target>>
        + ShrAssign<usize>
        + Sub<Output = DoublePrecisionOf<Target>>
        + SubAssign
        + Unitary
        + Zeroable,
{
    fn greater_binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_bitness: usize,
    ) -> Vec<Self> {
        debug_assert!(target_bitness > source_bitness && source_bitness > 0);
        let target_digit_mask =
            DoublePrecisionOf::<Self>::digit_mask(target_bitness);
        let result_capacity: usize = (source.len() * target_bitness
            + (target_bitness - 1))
            / target_bitness;
        let mut result = Vec::<Self>::with_capacity(result_capacity);
        let mut accumulator = DoublePrecisionOf::<Self>::zero();
        let mut accumulator_bits_count: usize = 0;
        for digit in source {
            accumulator |= DoublePrecisionOf::<Self>::from(*digit)
                << accumulator_bits_count;
            accumulator_bits_count += source_bitness;
            if accumulator_bits_count >= target_bitness {
                unsafe {
                    result.push(
                        Self::try_from(accumulator & target_digit_mask)
                            .unwrap_unchecked(),
                    );
                }
                accumulator >>= target_bitness;
                accumulator_bits_count -= target_bitness;
            }
        }
        if !accumulator.is_zero() || result.is_empty() {
            unsafe {
                result.push(Self::try_from(accumulator).unwrap_unchecked());
            }
        }
        result
    }
}

pub(super) trait LesserBinaryBaseFromBinaryDigits<Source>:
    Sized
{
    fn lesser_binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_bitness: usize,
    ) -> Vec<Self>;
}

impl<
        Source: Copy + BitLength<Output = usize> + DoublePrecision,
        Target: TryFrom<DoublePrecisionOf<Source>>,
    > LesserBinaryBaseFromBinaryDigits<Source> for Target
where
    DoublePrecisionOf<Source>: BitAnd<Output = DoublePrecisionOf<Source>>
        + BitOrAssign
        + Copy
        + DigitMask
        + Shl<usize, Output = DoublePrecisionOf<Source>>
        + Shr<usize, Output = DoublePrecisionOf<Source>>
        + ShrAssign<usize>
        + Zeroable,
{
    fn lesser_binary_base_from_binary_digits(
        source: &[Source],
        source_bitness: usize,
        target_bitness: usize,
    ) -> Vec<Self> {
        debug_assert!(source_bitness > target_bitness && target_bitness > 0);
        let target_digit_mask =
            DoublePrecisionOf::<Source>::digit_mask(target_bitness);
        let digits_bits_count: usize = (source.len() - 1) * source_bitness
            + source[source.len() - 1].bit_length();
        let digits_count: usize =
            (digits_bits_count + (target_bitness - 1)) / target_bitness;
        let mut result = Vec::<Self>::with_capacity(digits_count);
        let mut accumulator = DoublePrecisionOf::<Source>::from(source[0]);
        let mut accumulator_bits_count = source_bitness;
        for &digit in source.iter().skip(1) {
            loop {
                result.push(unsafe {
                    Self::try_from(accumulator & target_digit_mask)
                        .unwrap_unchecked()
                });
                accumulator >>= target_bitness;
                accumulator_bits_count -= target_bitness;
                if accumulator_bits_count < target_bitness {
                    break;
                }
            }
            accumulator |= DoublePrecisionOf::<Source>::from(digit)
                << accumulator_bits_count;
            accumulator_bits_count += source_bitness;
        }
        loop {
            result.push(unsafe {
                Self::try_from(accumulator & target_digit_mask)
                    .unwrap_unchecked()
            });
            accumulator >>= target_bitness;
            if accumulator.is_zero() {
                break;
            }
        }
        result
    }
}

pub(super) trait BitwiseAndComponents: Sized {
    fn bitwise_and_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: Vec<Self>,
        second_sign: Sign,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>);
}

impl<
        Digit: BitAndAssign + ComplementInPlace + Copy + DigitMask + Zeroable,
    > BitwiseAndComponents for Digit
{
    fn bitwise_and_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: Vec<Self>,
        second_sign: Sign,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>) {
        let (longest_sign, mut longest, shortest_sign, mut shortest) =
            if first.len() < second.len() {
                (second_sign, second, first_sign, first)
            } else {
                (first_sign, first, second_sign, second)
            };
        if longest_sign.is_negative() {
            Digit::complement_in_place::<DIGIT_BITNESS>(&mut longest);
        };
        if shortest_sign.is_negative() {
            Digit::complement_in_place::<DIGIT_BITNESS>(&mut shortest);
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
            result.push(Digit::digit_mask(DIGIT_BITNESS));
            Digit::complement_in_place::<DIGIT_BITNESS>(&mut result);
        }
        trim_leading_zeros(&mut result);
        sign *= to_digits_sign(&result);
        (sign, result)
    }
}

pub(super) trait BitwiseOrComponents: Sized {
    fn bitwise_or_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: Vec<Self>,
        second_sign: Sign,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>);
}

impl<Digit: BitOrAssign + ComplementInPlace + Copy + DigitMask + Zeroable>
    BitwiseOrComponents for Digit
{
    fn bitwise_or_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: Vec<Self>,
        second_sign: Sign,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>) {
        let (longest_sign, mut longest, shortest_sign, mut shortest) =
            if first.len() < second.len() {
                (second_sign, second, first_sign, first)
            } else {
                (first_sign, first, second_sign, second)
            };
        if longest_sign.is_negative() {
            Self::complement_in_place::<DIGIT_BITNESS>(&mut longest);
        };
        if shortest_sign.is_negative() {
            Self::complement_in_place::<DIGIT_BITNESS>(&mut shortest);
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
            result.push(Self::digit_mask(DIGIT_BITNESS));
            Self::complement_in_place::<DIGIT_BITNESS>(&mut result);
        }
        trim_leading_zeros(&mut result);
        (sign, result)
    }
}

pub(super) trait BitwiseXorComponents: Sized {
    fn bitwise_xor_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: Vec<Self>,
        second_sign: Sign,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>);
}

impl<
        Digit: BitXorAssign + ComplementInPlace + Copy + DigitMask + Zeroable,
    > BitwiseXorComponents for Digit
{
    fn bitwise_xor_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: Vec<Self>,
        second_sign: Sign,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>) {
        let (longest_sign, mut longest, shortest_sign, mut shortest) =
            if first.len() < second.len() {
                (second_sign, second, first_sign, first)
            } else {
                (first_sign, first, second_sign, second)
            };
        if longest_sign.is_negative() {
            Self::complement_in_place::<DIGIT_BITNESS>(&mut longest);
        };
        if shortest_sign.is_negative() {
            Self::complement_in_place::<DIGIT_BITNESS>(&mut shortest);
        };
        let mut result = longest;
        for index in 0..shortest.len() {
            result[index] ^= shortest[index];
        }
        if shortest_sign.is_negative() {
            let digit_mask = Self::digit_mask(DIGIT_BITNESS);
            for index in shortest.len()..result.len() {
                result[index] ^= digit_mask;
            }
        };
        let sign_is_negative =
            longest_sign.is_negative() ^ shortest_sign.is_negative();
        if sign_is_negative {
            result.push(Self::digit_mask(DIGIT_BITNESS));
            Self::complement_in_place::<DIGIT_BITNESS>(&mut result);
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
}

pub trait TryDivDigitsAsFloat<Output>: Sized {
    type Error;

    fn checked_div_digits_as_float<const DIGIT_BITNESS: usize>(
        dividend_digits: &[Self],
        divisor_digits: &[Self],
    ) -> Result<Output, Self::Error>;
}

macro_rules! checked_div_digits_as_float_impl {
    ($($float:ty)*) => ($(
        impl<
                Digit: AddAssign
                    + BitAnd<Output = Digit>
                    + BitLength<Output = usize>
                    + BitOr<Digit, Output = Digit>
                    + Copy
                    + From<u8>
                    + DivRemDigitsByDigit
                    + DivRemDigitsByTwoOrMoreDigits
                    + Mul<Output = Digit>
                    + Not<Output = Digit>
                    + ShiftDigitsLeftInPlace
                    + ShiftDigitsRightInPlace
                    + Shl<usize, Output = Digit>
                    + Shr<usize, Output = Digit>
                    + ReduceDigitsToFloat<$float>
                    + Sub<Output = Digit>
                    + Unitary
                    + Zeroable,
            > TryDivDigitsAsFloat<$float> for Digit
        {
            type Error = CheckedDivAsFloatError;

            fn checked_div_digits_as_float<const DIGIT_BITNESS: usize>(
                dividend_digits: &[Self],
                divisor_digits: &[Self],
            ) -> Result<$float, Self::Error> {
                const NON_EXPONENT_BITS_COUNT: usize =
                    <$float>::TOTAL_BITS_COUNT - <$float>::EXPONENT_BITS_COUNT;
                if divisor_digits.len() == 1 && divisor_digits[0].is_zero() {
                    return Err(CheckedDivAsFloatError::ZeroDivision);
                }
                if dividend_digits.len() == 1 && dividend_digits[0].is_zero() {
                    return Ok(<$float>::zero());
                }
                let dividend_digits_count = dividend_digits.len();
                let divisor_digits_count = divisor_digits.len();
                let dividend_is_small = dividend_digits_count
                    <= (NON_EXPONENT_BITS_COUNT / DIGIT_BITNESS)
                    || (dividend_digits_count
                        == (NON_EXPONENT_BITS_COUNT / DIGIT_BITNESS) + 1
                        && (dividend_digits
                            [(NON_EXPONENT_BITS_COUNT / DIGIT_BITNESS)]
                            >> (NON_EXPONENT_BITS_COUNT % DIGIT_BITNESS))
                            .is_zero());
                let divisor_is_small = divisor_digits_count
                    <= (NON_EXPONENT_BITS_COUNT / DIGIT_BITNESS)
                    || (divisor_digits_count
                        == (NON_EXPONENT_BITS_COUNT / DIGIT_BITNESS) + 1
                        && (divisor_digits
                            [(NON_EXPONENT_BITS_COUNT / DIGIT_BITNESS)]
                            >> (NON_EXPONENT_BITS_COUNT % DIGIT_BITNESS))
                            .is_zero());
                if dividend_is_small && divisor_is_small {
                    let reduced_dividend =
                        Self::reduce_digits_to_float::<DIGIT_BITNESS>(
                            dividend_digits,
                        );
                    let reduced_divisor = Self::reduce_digits_to_float::<
                        DIGIT_BITNESS,
                    >(divisor_digits);
                    return Ok(reduced_dividend / reduced_divisor);
                }
                let digits_count_difference = (dividend_digits_count as isize)
                    - (divisor_digits_count as isize);
                if digits_count_difference
                    > (((usize::MAX / DIGIT_BITNESS) - 1) as isize)
                {
                    return Err(CheckedDivAsFloatError::TooLarge);
                } else if digits_count_difference
                    < 1isize - ((usize::MAX / DIGIT_BITNESS) as isize)
                {
                    return Ok(<$float>::zero());
                }
                let bit_lengths_difference = digits_count_difference
                    * (DIGIT_BITNESS as isize)
                    + (((dividend_digits[dividend_digits.len() - 1]
                        .bit_length()) as isize)
                        - (divisor_digits[divisor_digits.len() - 1]
                            .bit_length()
                            as isize));
                if bit_lengths_difference > (<$float>::MAX_EXP as isize) {
                    return Err(CheckedDivAsFloatError::TooLarge);
                } else if bit_lengths_difference
                    < (<$float>::MIN_EXP as isize)
                        - ((NON_EXPONENT_BITS_COUNT as isize) - 1)
                {
                    return Ok(<$float>::zero());
                }
                let shift = bit_lengths_difference
                    .max(<$float>::MIN_EXP as isize)
                    - (NON_EXPONENT_BITS_COUNT as isize)
                    - 2;
                let mut inexact = false;
                let mut quotient_digits = if shift <= 0 {
                    let shift_digits = ((-shift) as usize) / DIGIT_BITNESS;
                    if dividend_digits_count
                        >= ((isize::MAX - 1) as usize) - shift_digits
                    {
                        return Err(CheckedDivAsFloatError::TooLarge);
                    }
                    let quotient_digits_count =
                        dividend_digits_count + shift_digits + 1;
                    let mut quotient_data =
                        vec![Self::zero(); quotient_digits_count];
                    let remainder =
                        Self::shift_digits_left_in_place::<DIGIT_BITNESS>(
                            dividend_digits,
                            ((-shift) as usize) % DIGIT_BITNESS,
                            &mut quotient_data[shift_digits..],
                        );
                    quotient_data[dividend_digits_count + shift_digits] =
                        remainder;
                    quotient_data
                } else {
                    let mut shift_digits = (shift as usize) / DIGIT_BITNESS;
                    let quotient_digits_count =
                        dividend_digits_count - shift_digits;
                    let mut quotient_data =
                        vec![Self::zero(); quotient_digits_count];
                    let remainder =
                        Self::shift_digits_right_in_place::<DIGIT_BITNESS>(
                            &dividend_digits[shift_digits..],
                            (shift as usize) % DIGIT_BITNESS,
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
                        Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(
                            &quotient_digits,
                            divisor_digits[0],
                        );
                    quotient_digits = next_quotient_digits;
                    if !remainder.is_zero() {
                        inexact = true;
                    }
                } else {
                    let (next_quotient_digits, remainder) =
                        Self::div_rem_by_two_or_more_digits::<DIGIT_BITNESS>(
                            &quotient_digits,
                            divisor_digits,
                        );
                    quotient_digits = next_quotient_digits;
                    if !to_digits_sign(&remainder).is_zero() {
                        inexact = true;
                    }
                }
                let quotient_bit_length = ((quotient_digits.len() - 1)
                    * DIGIT_BITNESS
                    + quotient_digits[quotient_digits.len() - 1].bit_length())
                    as isize;
                let extra_bits = quotient_bit_length
                    .max((<$float>::MIN_EXP as isize) - shift)
                    - (NON_EXPONENT_BITS_COUNT as isize);
                let mask = Self::one() << ((extra_bits as usize) - 1);
                let mut quotient_low_digit =
                    quotient_digits[0] | Self::from(inexact as u8);
                if !(quotient_low_digit & mask).is_zero()
                    && !(quotient_low_digit
                        & (Self::from(3u8) * mask - Self::from(1u8)))
                    .is_zero()
                {
                    quotient_low_digit += mask;
                }
                quotient_digits[0] = quotient_low_digit
                    & !(Self::from(2u8) * mask - Self::from(1u8));
                let reduced_quotient = Self::reduce_digits_to_float::<
                    DIGIT_BITNESS,
                >(&quotient_digits);
                if shift + quotient_bit_length >= (<$float>::MAX_EXP as isize)
                    && (shift + quotient_bit_length
                        > (<$float>::MAX_EXP as isize)
                        || reduced_quotient
                            == (1 as $float)
                                .load_exp(quotient_bit_length as i32))
                {
                    Err(CheckedDivAsFloatError::TooLarge)
                } else {
                    Ok(reduced_quotient.load_exp(shift as i32))
                }
            }
        }
    )*)
}

checked_div_digits_as_float_impl!(f32 f64);

pub(super) trait CheckedDivComponents: Sized {
    fn checked_div_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)>;
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + DivRemDigitsByTwoOrMoreDigits
            + PartialOrd
            + Zeroable,
    > CheckedDivComponents for Digit
{
    fn checked_div_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)> {
        if divisor_sign.is_zero() {
            None
        } else if dividend_sign.is_zero()
            || digits_lesser_than(dividend, divisor)
        {
            Some((Sign::zero(), vec![Self::zero()]))
        } else if divisor.len() == 1 {
            let (digits, _) = Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(
                dividend, divisor[0],
            );
            Some((dividend_sign * divisor_sign, digits))
        } else {
            let (digits, _) = Self::div_rem_by_two_or_more_digits::<
                DIGIT_BITNESS,
            >(dividend, divisor);
            Some((
                dividend_sign * divisor_sign * to_digits_sign(&digits),
                digits,
            ))
        }
    }
}

pub(super) trait CheckedDivEuclidComponents: Sized {
    fn checked_div_euclid_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)>;
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + DivRemDigitsByTwoOrMoreDigits
            + PartialOrd
            + SumDigits
            + Unitary
            + Zeroable,
    > CheckedDivEuclidComponents for Digit
{
    fn checked_div_euclid_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)> {
        if divisor_sign.is_zero() {
            None
        } else if dividend_sign.is_zero() {
            Some((Sign::zero(), vec![Self::zero()]))
        } else if digits_lesser_than(dividend, divisor) {
            Some(
                if (dividend_sign.is_negative() && divisor_sign.is_positive())
                    || (dividend_sign.is_positive()
                        && divisor_sign.is_negative())
                {
                    (-Sign::one(), vec![Self::one()])
                } else {
                    (Sign::zero(), vec![Self::zero()])
                },
            )
        } else {
            let (sign, mut digits, remainder_is_non_zero) = if divisor.len()
                == 1
            {
                let (digits, remainder_digit) =
                    Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(
                        dividend, divisor[0],
                    );
                (
                    dividend_sign * divisor_sign,
                    digits,
                    !remainder_digit.is_zero(),
                )
            } else {
                let (digits, remainder) = Self::div_rem_by_two_or_more_digits::<
                    DIGIT_BITNESS,
                >(dividend, divisor);
                (
                    dividend_sign * divisor_sign * to_digits_sign(&digits),
                    digits,
                    !to_digits_sign(&remainder).is_zero(),
                )
            };
            if remainder_is_non_zero
                && ((dividend_sign.is_negative()
                    && divisor_sign.is_positive())
                    || (dividend_sign.is_positive()
                        && divisor_sign.is_negative()))
            {
                digits =
                    Self::sum_digits::<DIGIT_BITNESS>(&digits, &[Self::one()]);
            }
            Some((sign, digits))
        }
    }
}

pub(super) trait CheckedDivRemComponents: Sized {
    fn checked_div_rem_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>, Sign, Vec<Self>)>;
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + DivRemDigitsByTwoOrMoreDigits
            + PartialOrd
            + Zeroable,
    > CheckedDivRemComponents for Digit
{
    fn checked_div_rem_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>, Sign, Vec<Self>)> {
        if divisor_sign.is_zero() {
            None
        } else if dividend_sign.is_zero()
            || dividend.len() < divisor.len()
            || (dividend.len() == divisor.len()
                && dividend[dividend.len() - 1] < divisor[divisor.len() - 1])
        {
            Some((
                Sign::zero(),
                vec![Self::zero(); 1],
                dividend_sign,
                dividend.to_vec(),
            ))
        } else if divisor.len() == 1 {
            let (quotient_digits, remainder_digit) =
                Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(
                    dividend, divisor[0],
                );
            Some((
                dividend_sign * divisor_sign,
                quotient_digits,
                dividend_sign * Sign::from(!remainder_digit.is_zero()),
                vec![remainder_digit],
            ))
        } else {
            let (quotient_digits, remainder_digits) =
                Self::div_rem_by_two_or_more_digits::<DIGIT_BITNESS>(
                    dividend, divisor,
                );
            Some((
                dividend_sign
                    * divisor_sign
                    * to_digits_sign(&quotient_digits),
                quotient_digits,
                dividend_sign * to_digits_sign(&remainder_digits),
                remainder_digits,
            ))
        }
    }
}

pub(super) trait CheckedDivRemEuclidComponents: Sized {
    fn checked_div_rem_euclid_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>, Sign, Vec<Self>)>;
}

impl<
        Digit: CheckedDivRemComponents + SubtractComponents + SumComponents + Unitary,
    > CheckedDivRemEuclidComponents for Digit
{
    fn checked_div_rem_euclid_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Digit],
        divisor_sign: Sign,
        divisor: &[Digit],
    ) -> Option<(Sign, Vec<Digit>, Sign, Vec<Digit>)> {
        let (
            mut quotient_sign,
            mut quotient,
            mut remainder_sign,
            mut remainder,
        ) = Digit::checked_div_rem_components::<DIGIT_BITNESS>(
            dividend_sign,
            dividend,
            divisor_sign,
            divisor,
        )?;
        if (divisor_sign.is_negative() && remainder_sign.is_positive())
            || (divisor_sign.is_positive() && remainder_sign.is_negative())
        {
            (quotient_sign, quotient) =
                Digit::subtract_components::<DIGIT_BITNESS>(
                    quotient_sign,
                    &quotient,
                    Sign::one(),
                    &[Digit::one()],
                );
            (remainder_sign, remainder) = Digit::sum_components::<DIGIT_BITNESS>(
                remainder_sign,
                &remainder,
                divisor_sign,
                divisor,
            );
        }
        Some((quotient_sign, quotient, remainder_sign, remainder))
    }
}

pub(super) trait CheckedRemComponents: Sized {
    fn checked_rem_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)>;
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + DivRemDigitsByTwoOrMoreDigits
            + PartialOrd
            + Zeroable,
    > CheckedRemComponents for Digit
{
    fn checked_rem_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)> {
        if divisor_sign.is_zero() {
            None
        } else if dividend_sign.is_zero()
            || digits_lesser_than(dividend, divisor)
        {
            Some((dividend_sign, dividend.to_vec()))
        } else if divisor.len() == 1 {
            let (_, remainder) = Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(
                dividend, divisor[0],
            );
            Some((
                dividend_sign * Sign::from(!remainder.is_zero()),
                vec![remainder],
            ))
        } else {
            let (_, remainder) = Self::div_rem_by_two_or_more_digits::<
                DIGIT_BITNESS,
            >(dividend, divisor);
            Some((dividend_sign * to_digits_sign(&remainder), remainder))
        }
    }
}

pub(super) trait CheckedRemEuclidComponents: Sized {
    fn checked_rem_euclid_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)>;
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + DivRemDigitsByTwoOrMoreDigits
            + PartialOrd
            + SubtractDigits
            + Zeroable,
    > CheckedRemEuclidComponents for Digit
{
    fn checked_rem_euclid_components<const DIGIT_BITNESS: usize>(
        dividend_sign: Sign,
        dividend: &[Self],
        divisor_sign: Sign,
        divisor: &[Self],
    ) -> Option<(Sign, Vec<Self>)> {
        if divisor_sign.is_zero() {
            None
        } else if dividend_sign.is_zero() {
            Some((dividend_sign, dividend.to_vec()))
        } else if digits_lesser_than(dividend, divisor) {
            Some(
                if (dividend_sign.is_negative() && divisor_sign.is_positive())
                    || (dividend_sign.is_positive()
                        && divisor_sign.is_negative())
                {
                    Digit::subtract_digits::<DIGIT_BITNESS>(
                        dividend,
                        divisor,
                        dividend_sign,
                    )
                } else {
                    (dividend_sign, dividend.to_vec())
                },
            )
        } else {
            let (mut sign, mut digits) = if divisor.len() == 1 {
                let (_, digit) = Digit::div_rem_digits_by_digit::<DIGIT_BITNESS>(
                    dividend, divisor[0],
                );
                (dividend_sign * Sign::from(!digit.is_zero()), vec![digit])
            } else {
                let (_, digits) = Digit::div_rem_by_two_or_more_digits::<
                    DIGIT_BITNESS,
                >(dividend, divisor);
                (dividend_sign * to_digits_sign(&digits), digits)
            };
            if (divisor_sign.is_negative() && sign.is_positive())
                || (divisor_sign.is_positive() && sign.is_negative())
            {
                (sign, digits) = Digit::subtract_digits::<DIGIT_BITNESS>(
                    &digits, divisor, sign,
                );
            }
            Some((sign, digits))
        }
    }
}

pub(super) trait ComplementInPlace: Sized {
    fn complement_in_place<const DIGIT_BITNESS: usize>(digits: &mut [Self]);
}

impl<
        Digit: AddAssign
            + BitAnd<Output = Digit>
            + BitXor<Output = Digit>
            + Copy
            + DigitMask
            + ShrAssign<usize>
            + Unitary
            + Zeroable,
    > ComplementInPlace for Digit
{
    fn complement_in_place<const DIGIT_BITNESS: usize>(digits: &mut [Self]) {
        let mut accumulator = Self::one();
        let digit_mask = Self::digit_mask(DIGIT_BITNESS);
        for index in 0..digits.len() {
            accumulator += digits[index] ^ digit_mask;
            digits[index] = accumulator & digit_mask;
            accumulator >>= DIGIT_BITNESS;
        }
        debug_assert!(accumulator.is_zero());
    }
}

#[inline]
pub(super) fn compare_digits<Digit: Ord>(
    left: &[Digit],
    right: &[Digit],
) -> Ordering {
    match left.len().cmp(&right.len()) {
        Ordering::Equal => left.iter().rev().cmp(right.iter().rev()),
        value => value,
    }
}

#[inline]
fn digits_lesser_than<Digit: PartialOrd>(
    left: &[Digit],
    right: &[Digit],
) -> bool {
    left.len() < right.len()
        || left.len() == right.len()
            && left.iter().rev().lt(right.iter().rev())
}

pub(super) trait DivRemDigitsByDigit: Sized {
    fn div_rem_digits_by_digit<const DIGIT_BITNESS: usize>(
        dividend: &[Self],
        divisor: Self,
    ) -> (Vec<Self>, Self);
}

impl<
        Digit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>> + Zeroable,
    > DivRemDigitsByDigit for Digit
where
    DoublePrecisionOf<Digit>: Copy
        + Mul<Output = DoublePrecisionOf<Digit>>
        + Div<Output = DoublePrecisionOf<Digit>>
        + BitOrAssign
        + ShlAssign<usize>
        + SubAssign
        + Zeroable,
{
    fn div_rem_digits_by_digit<const DIGIT_BITNESS: usize>(
        dividend: &[Self],
        divisor: Self,
    ) -> (Vec<Self>, Self) {
        let mut quotient = vec![Self::zero(); dividend.len()];
        let mut remainder = DoublePrecisionOf::<Self>::zero();
        let digits_count = dividend.len();
        let divisor = DoublePrecisionOf::<Self>::from(divisor);
        for offset in 1..=digits_count {
            remainder <<= DIGIT_BITNESS;
            remainder |= DoublePrecisionOf::<Self>::from(
                dividend[digits_count - offset],
            );
            let quotient_digit = unsafe {
                Self::try_from(remainder / divisor).unwrap_unchecked()
            };
            quotient[digits_count - offset] = quotient_digit;
            remainder -=
                DoublePrecisionOf::<Self>::from(quotient_digit) * divisor;
        }
        trim_leading_zeros(&mut quotient);
        (quotient, unsafe {
            Self::try_from(remainder).unwrap_unchecked()
        })
    }
}

pub(super) trait DivRemDigitsByTwoOrMoreDigits: Sized {
    fn div_rem_by_two_or_more_digits<const DIGIT_BITNESS: usize>(
        dividend: &[Self],
        divisor: &[Self],
    ) -> (Vec<Self>, Vec<Self>);
}

impl<
        Digit: Add<Output = Digit>
            + AddAssign
            + BitAnd<Output = Digit>
            + BitLength<Output = usize>
            + Copy
            + DigitMask
            + DoublePrecision
            + Oppose
            + PartialOrd
            + ShiftDigitsLeftInPlace
            + ShiftDigitsRightInPlace
            + Shl<usize, Output = Digit>
            + Shr<usize, Output = Digit>
            + ShrAssign<usize>
            + SubAssign
            + TryFrom<DoublePrecisionOf<Digit>>
            + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
            + Unitary
            + Zeroable,
    > DivRemDigitsByTwoOrMoreDigits for Digit
where
    DoublePrecisionOf<Digit>: BitOr<Output = DoublePrecisionOf<Digit>>
        + Copy
        + Div<Output = DoublePrecisionOf<Digit>>
        + Mul<Output = DoublePrecisionOf<Digit>>
        + Oppose
        + PartialOrd
        + Shl<usize, Output = DoublePrecisionOf<Digit>>
        + Sub<Output = DoublePrecisionOf<Digit>>,
    OppositionOf<Digit>: Add<Output = OppositionOf<Digit>>
        + Copy
        + PartialOrd
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: Add<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + BitAnd<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + Copy
        + From<Digit>
        + From<OppositionOf<Digit>>
        + Mul<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + Shr<usize, Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + Sub<Output = OppositionOf<DoublePrecisionOf<Digit>>>,
{
    fn div_rem_by_two_or_more_digits<const DIGIT_BITNESS: usize>(
        dividend: &[Digit],
        divisor: &[Digit],
    ) -> (Vec<Digit>, Vec<Digit>) {
        let dividend_digits_count = dividend.len();
        let divisor_digits_count = divisor.len();
        let mut dividend_normalized =
            vec![Digit::zero(); dividend_digits_count];
        let mut divisor_normalized = vec![Digit::zero(); divisor_digits_count];
        let shift = DIGIT_BITNESS - divisor[divisor.len() - 1].bit_length();
        Digit::shift_digits_left_in_place::<DIGIT_BITNESS>(
            divisor,
            shift,
            &mut divisor_normalized,
        );
        let accumulator = Digit::shift_digits_left_in_place::<DIGIT_BITNESS>(
            dividend,
            shift,
            &mut dividend_normalized,
        );
        let last_divisor_digit_normalized =
            divisor_normalized[divisor_normalized.len() - 1];
        if !accumulator.is_zero()
            || dividend_normalized[dividend_normalized.len() - 1]
                >= last_divisor_digit_normalized
        {
            dividend_normalized.push(accumulator);
        }
        let quotient_size =
            dividend_normalized.len() - divisor_normalized.len();
        let mut quotient = vec![Digit::zero(); quotient_size];
        let penult_divisor_digit_normalized =
            divisor_normalized[divisor_digits_count - 2];
        let mut quotient_index = quotient_size;
        let base = Digit::one() << DIGIT_BITNESS;
        let digit_mask = Digit::digit_mask(DIGIT_BITNESS);
        for offset in (0..quotient_size).rev() {
            let step = (DoublePrecisionOf::<Digit>::from(
                dividend_normalized[offset + divisor_digits_count],
            ) << DIGIT_BITNESS)
                | DoublePrecisionOf::<Digit>::from(
                    dividend_normalized[offset + divisor_digits_count - 1],
                );
            let mut quotient_digit = unsafe {
                Digit::try_from(
                    step / DoublePrecisionOf::<Digit>::from(
                        last_divisor_digit_normalized,
                    ),
                )
                .unwrap_unchecked()
            };
            let mut step_remainder = unsafe {
                Digit::try_from(
                    step - DoublePrecisionOf::<Digit>::from(
                        last_divisor_digit_normalized,
                    ) * DoublePrecisionOf::<Digit>::from(
                        quotient_digit,
                    ),
                )
                .unwrap_unchecked()
            };
            while DoublePrecisionOf::<Digit>::from(
                penult_divisor_digit_normalized,
            ) * DoublePrecisionOf::<Digit>::from(quotient_digit)
                > ((DoublePrecisionOf::<Digit>::from(step_remainder)
                    << DIGIT_BITNESS)
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
                let step = OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    dividend_normalized[offset + index],
                ) + OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    accumulator,
                ) - OppositionOf::<DoublePrecisionOf<Digit>>::from(
                    quotient_digit,
                )
                    * OppositionOf::<DoublePrecisionOf<Digit>>::from(
                        divisor_normalized[index],
                    );
                dividend_normalized[offset + index] = unsafe {
                    Digit::try_from(
                        step & OppositionOf::<DoublePrecisionOf<Digit>>::from(
                            digit_mask,
                        ),
                    )
                    .unwrap_unchecked()
                };
                accumulator = unsafe {
                    OppositionOf::<Digit>::try_from(step >> DIGIT_BITNESS)
                        .unwrap_unchecked()
                };
            }
            if unsafe {
                OppositionOf::<Digit>::try_from(
                    dividend_normalized[offset + divisor_digits_count],
                )
                .unwrap_unchecked()
            } + accumulator
                < OppositionOf::<Digit>::zero()
            {
                let mut accumulator = Digit::zero();
                for index in 0..divisor_digits_count {
                    accumulator = accumulator
                        + dividend_normalized[offset + index]
                        + divisor_normalized[index];
                    dividend_normalized[offset + index] =
                        accumulator & digit_mask;
                    accumulator >>= DIGIT_BITNESS;
                }
                quotient_digit -= Digit::one();
            }
            quotient_index -= 1;
            quotient[quotient_index] = quotient_digit;
        }
        if quotient_size.is_zero() {
            quotient = vec![Digit::zero()];
        }
        trim_leading_zeros(&mut quotient);
        let mut remainder = divisor_normalized;
        Digit::shift_digits_right_in_place::<DIGIT_BITNESS>(
            &dividend_normalized[..divisor_digits_count],
            shift,
            remainder.as_mut_slice(),
        );
        trim_leading_zeros(&mut remainder);
        (quotient, remainder)
    }
}

pub(super) trait FractExpDigits<Fraction>: Sized {
    fn fract_exp_digits<const DIGIT_BITNESS: usize>(
        digits: &[Self],
    ) -> Option<(Fraction, i32)>;
}

impl<
        Digit: BitAnd<Output = Digit>
            + BitOrAssign<Digit>
            + BitLength<Output = usize>
            + Copy
            + From<u8>
            + TryFrom<OppositionOf<Digit>>
            + Oppose
            + ShiftDigitsLeftInPlace
            + ShiftDigitsRightInPlace
            + Unitary
            + Zeroable,
        Fraction: Add<Output = Fraction>
            + Div<Output = Fraction>
            + Mul<Output = Fraction>
            + Sub<Output = Fraction>
            + SubAssign
            + Copy
            + Floor<Output = Fraction>
            + FractExp<Output = (Fraction, i32)>
            + From<Digit>
            + From<f32>
            + LoadExp<i32, Output = Fraction>
            + MantissaDigits
            + MaxExp
            + MinExp
            + PartialEq
            + DivAssign
            + Unitary
            + Zeroable,
    > FractExpDigits<Fraction> for Digit
where
    OppositionOf<Digit>:
        Add<Output = OppositionOf<Digit>> + From<i8> + TryFrom<Digit>,
    usize: TryFrom<Digit>,
{
    fn fract_exp_digits<const DIGIT_BITNESS: usize>(
        digits: &[Self],
    ) -> Option<(Fraction, i32)> {
        let mut result_digits =
            vec![
                Self::zero();
                2usize + (Fraction::MANTISSA_DIGITS + 1usize) / DIGIT_BITNESS
            ];
        let size = digits.len();
        let mut bits_count = digits[digits.len() - 1].bit_length();
        if size > (usize::MAX - 1) / DIGIT_BITNESS
            && (size > (usize::MAX - 1) / DIGIT_BITNESS + 1
                || bits_count > (usize::MAX - 1) % DIGIT_BITNESS + 1)
        {
            return None;
        }
        bits_count += (size - 1) * DIGIT_BITNESS;
        let mut result_digits_count = if bits_count
            <= Fraction::MANTISSA_DIGITS + 2
        {
            let shift_digits =
                (Fraction::MANTISSA_DIGITS + 2 - bits_count) / DIGIT_BITNESS;
            let shift_bits =
                (Fraction::MANTISSA_DIGITS + 2 - bits_count) % DIGIT_BITNESS;
            let mut result_size = shift_digits;
            let remainder = Self::shift_digits_left_in_place::<DIGIT_BITNESS>(
                digits,
                shift_bits,
                &mut result_digits[result_size..],
            );
            result_size += size;
            result_digits[result_size] = remainder;
            result_size += 1;
            result_size
        } else {
            let mut shift_digits =
                (bits_count - Fraction::MANTISSA_DIGITS - 2) / DIGIT_BITNESS;
            let shift_bits =
                (bits_count - Fraction::MANTISSA_DIGITS - 2) % DIGIT_BITNESS;
            let remainder = Self::shift_digits_right_in_place::<DIGIT_BITNESS>(
                &digits[shift_digits..],
                shift_bits,
                &mut result_digits,
            );
            let result_size = size - shift_digits;
            if remainder.is_zero() {
                while shift_digits > 0 {
                    shift_digits -= 1;
                    if !digits[shift_digits].is_zero() {
                        result_digits[0] |= Self::one();
                        break;
                    }
                }
            } else {
                result_digits[0] |= Self::one();
            }
            result_size
        };
        {
            const HALF_EVEN_CORRECTION: [i8; 8] = [0, -1, -2, 1, 0, -1, 2, 1];
            result_digits[0] = unsafe {
                Self::try_from(
                    OppositionOf::<Self>::try_from(result_digits[0])
                        .unwrap_unchecked()
                        + OppositionOf::<Self>::from(
                            HALF_EVEN_CORRECTION[usize::try_from(
                                result_digits[0] & Self::from(7u8),
                            )
                            .unwrap_unchecked()],
                        ),
                )
                .unwrap_unchecked()
            };
        }
        result_digits_count -= 1;
        let mut fraction = Fraction::from(result_digits[result_digits_count]);
        while result_digits_count > 0 {
            result_digits_count -= 1;
            fraction = fraction
                * Fraction::from((1usize << DIGIT_BITNESS) as f32)
                + Fraction::from(result_digits[result_digits_count]);
        }
        fraction /=
            Fraction::from((1u64 << (Fraction::MANTISSA_DIGITS + 2)) as f32);
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
}

pub(super) trait InvertComponents: Sized {
    fn invert_components<const DIGIT_BITNESS: usize>(
        sign: Sign,
        digits: &[Self],
    ) -> (Sign, Vec<Self>);
}

impl<Digit: SumComponents + Unitary> InvertComponents for Digit {
    fn invert_components<const DIGIT_BITNESS: usize>(
        sign: Sign,
        digits: &[Digit],
    ) -> (Sign, Vec<Digit>) {
        let (sign, digits) = Digit::sum_components::<DIGIT_BITNESS>(
            sign,
            digits,
            Sign::one(),
            &[Digit::one()],
        );
        (-sign, digits)
    }
}

pub(super) trait MultiplyDigits: Sized {
    fn multiply_digits<const DIGIT_BITNESS: usize>(
        first: &[Self],
        second: &[Self],
    ) -> Vec<Self>;
}

impl<
        Digit: Copy
            + MultiplyDigitsPlain
            + SubtractDigitsInPlace
            + SumDigits
            + SumDigitsInPlace
            + Zeroable,
    > MultiplyDigits for Digit
{
    fn multiply_digits<const DIGIT_BITNESS: usize>(
        first: &[Self],
        second: &[Self],
    ) -> Vec<Self> {
        const KARATSUBA_CUTOFF: usize = 70;
        const KARATSUBA_SQUARE_CUTOFF: usize = KARATSUBA_CUTOFF * 2;
        let (longest, shortest) = if first.len() < second.len() {
            (&second, &first)
        } else {
            (&first, &second)
        };
        if shortest.len()
            <= if shortest.as_ptr() == longest.as_ptr() {
                KARATSUBA_SQUARE_CUTOFF
            } else {
                KARATSUBA_CUTOFF
            }
        {
            if shortest.len() == 1 && shortest[0].is_zero() {
                vec![Self::zero()]
            } else {
                Self::multiply_digits_plain::<DIGIT_BITNESS>(shortest, longest)
            }
        } else {
            let are_digits_lopsided = 2 * shortest.len() <= longest.len();
            if are_digits_lopsided {
                let shortest_size = shortest.len();
                let mut longest_size = longest.len();
                let mut result =
                    vec![Self::zero(); shortest_size + longest_size];
                let mut processed_digits_count = 0;
                while longest_size > 0 {
                    let step_digits_count = longest_size.min(shortest_size);
                    let product = Self::multiply_digits::<DIGIT_BITNESS>(
                        shortest,
                        &longest[processed_digits_count
                            ..processed_digits_count + step_digits_count],
                    );
                    Self::sum_digits_in_place::<DIGIT_BITNESS>(
                        &mut result[processed_digits_count..],
                        &product,
                    );
                    longest_size -= step_digits_count;
                    processed_digits_count += step_digits_count;
                }
                trim_leading_zeros(&mut result);
                result
            } else {
                let shift = longest.len() >> 1;
                let (shortest_high, shortest_low) =
                    split_digits(shortest, shift);
                let (longest_high, longest_low) =
                    if shortest.as_ptr() == longest.as_ptr() {
                        (shortest_high.clone(), shortest_low.clone())
                    } else {
                        split_digits(longest, shift)
                    };
                let mut result =
                    vec![Self::zero(); shortest.len() + longest.len()];
                let highs_product = Self::multiply_digits::<DIGIT_BITNESS>(
                    &shortest_high,
                    &longest_high,
                );
                for (index, &digit) in highs_product.iter().enumerate() {
                    result[index + 2 * shift] = digit;
                }
                let lows_product = Self::multiply_digits::<DIGIT_BITNESS>(
                    &shortest_low,
                    &longest_low,
                );
                for (index, &digit) in lows_product.iter().enumerate() {
                    result[index] = digit;
                }
                Self::subtract_digits_in_place::<DIGIT_BITNESS>(
                    &mut result[shift..],
                    &lows_product,
                );
                Self::subtract_digits_in_place::<DIGIT_BITNESS>(
                    &mut result[shift..],
                    &highs_product,
                );
                let shortest_components_sum = Self::sum_digits::<DIGIT_BITNESS>(
                    &shortest_high,
                    &shortest_low,
                );
                let longest_components_sum =
                    if shortest.as_ptr() == longest.as_ptr() {
                        shortest_components_sum.clone()
                    } else {
                        Self::sum_digits::<DIGIT_BITNESS>(
                            &longest_high,
                            &longest_low,
                        )
                    };
                let components_sums_product =
                    Self::multiply_digits::<DIGIT_BITNESS>(
                        &shortest_components_sum,
                        &longest_components_sum,
                    );
                Self::sum_digits_in_place::<DIGIT_BITNESS>(
                    &mut result[shift..],
                    &components_sums_product,
                );
                trim_leading_zeros(&mut result);
                result
            }
        }
    }
}

pub(super) trait MultiplyDigitsPlain: Sized {
    fn multiply_digits_plain<const DIGIT_BITNESS: usize>(
        shortest: &[Self],
        longest: &[Self],
    ) -> Vec<Self>;
}

impl<
        Digit: AddAssign
            + Copy
            + DoublePrecision
            + TryFrom<DoublePrecisionOf<Digit>>
            + Zeroable,
    > MultiplyDigitsPlain for Digit
where
    DoublePrecisionOf<Digit>: Add<Output = DoublePrecisionOf<Digit>>
        + AddAssign
        + BitAnd<Output = DoublePrecisionOf<Digit>>
        + Copy
        + DigitMask
        + Mul<Output = DoublePrecisionOf<Digit>>
        + ShlAssign<usize>
        + ShrAssign<usize>
        + Zeroable,
{
    fn multiply_digits_plain<const DIGIT_BITNESS: usize>(
        shortest: &[Self],
        longest: &[Self],
    ) -> Vec<Self> {
        let mut result = vec![Self::zero(); shortest.len() + longest.len()];
        let digit_mask = DoublePrecisionOf::<Self>::digit_mask(DIGIT_BITNESS);
        if shortest.as_ptr() == longest.as_ptr() {
            for index in 0..shortest.len() {
                let mut digit =
                    DoublePrecisionOf::<Self>::from(shortest[index]);
                let mut result_position = index << 1;
                let mut accumulator =
                    DoublePrecisionOf::<Self>::from(result[result_position])
                        + digit * digit;
                result[result_position] = unsafe {
                    Self::try_from(accumulator & digit_mask).unwrap_unchecked()
                };
                result_position += 1;
                accumulator >>= DIGIT_BITNESS;
                digit <<= 1;
                for next_index in index + 1..shortest.len() {
                    accumulator += DoublePrecisionOf::<Self>::from(
                        result[result_position],
                    ) + DoublePrecisionOf::<Self>::from(
                        shortest[next_index],
                    ) * digit;
                    result[result_position] = unsafe {
                        Self::try_from(accumulator & digit_mask)
                            .unwrap_unchecked()
                    };
                    result_position += 1;
                    accumulator >>= DIGIT_BITNESS;
                }
                if !accumulator.is_zero() {
                    accumulator += DoublePrecisionOf::<Self>::from(
                        result[result_position],
                    );
                    result[result_position] = unsafe {
                        Self::try_from(accumulator & digit_mask)
                            .unwrap_unchecked()
                    };
                    result_position += 1;
                    accumulator >>= DIGIT_BITNESS;
                }
                if !accumulator.is_zero() {
                    result[result_position] += unsafe {
                        Self::try_from(accumulator & digit_mask)
                            .unwrap_unchecked()
                    };
                }
            }
        } else {
            for index in 0..shortest.len() {
                let mut accumulator = DoublePrecisionOf::<Self>::zero();
                let digit = DoublePrecisionOf::<Self>::from(shortest[index]);
                let mut result_position = index;
                for &second_digit in longest {
                    accumulator = accumulator
                        + DoublePrecisionOf::<Self>::from(
                            result[result_position],
                        )
                        + DoublePrecisionOf::<Self>::from(second_digit)
                            * digit;
                    result[result_position] = unsafe {
                        Self::try_from(accumulator & digit_mask)
                            .unwrap_unchecked()
                    };
                    result_position += 1;
                    accumulator >>= DIGIT_BITNESS;
                }
                if !accumulator.is_zero() {
                    result[result_position] += unsafe {
                        Self::try_from(accumulator & digit_mask)
                            .unwrap_unchecked()
                    };
                }
            }
        }
        trim_leading_zeros(&mut result);
        result
    }
}

pub(super) fn negate_bytes(digits: &mut [u8]) {
    let mut carry = true;
    for digit in digits {
        *digit = !*digit;
        if carry {
            *digit = digit.wrapping_add(1);
            carry = digit.is_zero();
        }
    }
}

pub(super) trait GreaterBinaryBaseFromNonBinaryDigits<Source>:
    Sized
{
    fn greater_binary_base_from_non_binary_digits<
        const TARGET_BITNESS: usize,
    >(
        source: &[Source],
        source_base: usize,
    ) -> Vec<Self>;
}

impl<
        Source: Copy,
        Target: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Target>> + Zeroable,
    > GreaterBinaryBaseFromNonBinaryDigits<Source> for Target
where
    DoublePrecisionOf<Target>: AddAssign
        + BitAnd<Output = DoublePrecisionOf<Target>>
        + Copy
        + DigitMask
        + From<Source>
        + Mul<Output = DoublePrecisionOf<Target>>
        + MulAssign
        + ShrAssign<usize>
        + TryFrom<usize>
        + Zeroable,
{
    fn greater_binary_base_from_non_binary_digits<
        const TARGET_BITNESS: usize,
    >(
        source: &[Source],
        source_base: usize,
    ) -> Vec<Self> {
        static mut BASES_LOGS: [f64; 37] = [0.0; 37];
        static mut INFIMUM_BASES_EXPONENTS: [usize; 37] = [0; 37];
        static mut INFIMUM_BASES_POWERS: [usize; 37] = [0; 37];
        let target_digit_mask =
            DoublePrecisionOf::<Self>::digit_mask(TARGET_BITNESS);
        if unsafe { BASES_LOGS[source_base] } == 0.0 {
            let bases_log = (source_base as f64).ln()
                / ((1usize << TARGET_BITNESS) as f64).ln();
            unsafe { BASES_LOGS[source_base] = bases_log };
            let mut infimum_base_power = source_base;
            let mut infimum_base_exponent: usize = 1;
            loop {
                let candidate: usize = infimum_base_power * source_base;
                if candidate > 1usize << TARGET_BITNESS {
                    break;
                }
                infimum_base_power = candidate;
                infimum_base_exponent += 1;
            }
            unsafe { INFIMUM_BASES_POWERS[source_base] = infimum_base_power };
            unsafe {
                INFIMUM_BASES_EXPONENTS[source_base] = infimum_base_exponent;
            };
        }
        let digits_count_upper_bound =
            (source.len() as f64) * unsafe { BASES_LOGS[source_base] } + 1.0;
        let mut result =
            Vec::<Self>::with_capacity(digits_count_upper_bound as usize);
        let infimum_base_exponent =
            unsafe { INFIMUM_BASES_EXPONENTS[source_base] };
        let infimum_base_power = unsafe { INFIMUM_BASES_POWERS[source_base] };
        let mut reversed_source = source.iter().rev();
        while let Some(&digit) = reversed_source.next() {
            let mut accumulator = DoublePrecisionOf::<Self>::from(digit);
            let mut base_exponent: usize = 1;
            while base_exponent < infimum_base_exponent {
                if let Some(&digit) = reversed_source.next() {
                    base_exponent += 1;
                    accumulator *= unsafe {
                        DoublePrecisionOf::<Self>::try_from(source_base)
                            .unwrap_unchecked()
                    };
                    accumulator += DoublePrecisionOf::<Self>::from(digit);
                } else {
                    break;
                }
            }
            let base_power = if base_exponent == infimum_base_exponent {
                infimum_base_power
            } else {
                source_base.pow(base_exponent as u32)
            };
            for result_position in &mut result {
                accumulator += <DoublePrecisionOf<Self> as From<Self>>::from(
                    *result_position,
                ) * unsafe {
                    DoublePrecisionOf::<Self>::try_from(base_power)
                        .unwrap_unchecked()
                };
                *result_position = unsafe {
                    Self::try_from(accumulator & target_digit_mask)
                        .unwrap_unchecked()
                };
                accumulator >>= TARGET_BITNESS;
            }
            if !accumulator.is_zero() {
                result.push(unsafe {
                    Self::try_from(accumulator).unwrap_unchecked()
                });
            }
        }
        if result.is_empty() {
            result.push(Self::zero());
        }
        result
    }
}

pub(super) trait LesserBinaryBaseFromNonBinaryDigits<Source>:
    Sized
{
    fn lesser_binary_base_from_non_binary_digits<const TARGET_BITNESS: usize>(
        source: &[Source],
        source_base: usize,
    ) -> Vec<Self>;
}

impl<
        Source: Copy,
        Target: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Target>> + Zeroable,
    > LesserBinaryBaseFromNonBinaryDigits<Source> for Target
where
    DoublePrecisionOf<Target>: AddAssign
        + BitAnd<Output = DoublePrecisionOf<Target>>
        + Copy
        + DigitMask
        + From<Source>
        + Mul<Output = DoublePrecisionOf<Target>>
        + ShrAssign<usize>
        + TryFrom<usize>
        + Zeroable,
{
    fn lesser_binary_base_from_non_binary_digits<
        const TARGET_BITNESS: usize,
    >(
        source: &[Source],
        source_base: usize,
    ) -> Vec<Self> {
        static mut BASES_LOGS: [f64; 37] = [0.0; 37];
        let target_digit_mask =
            DoublePrecisionOf::<Self>::digit_mask(TARGET_BITNESS);
        if unsafe { BASES_LOGS[source_base] } == 0.0 {
            let bases_log = (source_base as f64).ln()
                / ((1usize << TARGET_BITNESS) as f64).ln();
            unsafe { BASES_LOGS[source_base] = bases_log };
        }
        let digits_count_upper_bound =
            (source.len() as f64) * unsafe { BASES_LOGS[source_base] } + 1.0;
        let mut result =
            Vec::<Self>::with_capacity(digits_count_upper_bound as usize);
        let source_base = unsafe {
            DoublePrecisionOf::<Self>::try_from(source_base).unwrap_unchecked()
        };
        for &digit in source.iter().rev() {
            let mut accumulator = DoublePrecisionOf::<Self>::from(digit);
            for result_position in &mut result {
                accumulator += <DoublePrecisionOf<Self> as From<Self>>::from(
                    *result_position,
                ) * source_base;
                *result_position = unsafe {
                    Self::try_from(accumulator & target_digit_mask)
                        .unwrap_unchecked()
                };
                accumulator >>= TARGET_BITNESS;
            }
            while !accumulator.is_zero() {
                result.push(unsafe {
                    Self::try_from(accumulator & target_digit_mask)
                        .unwrap_unchecked()
                });
                accumulator >>= TARGET_BITNESS;
            }
        }
        if result.is_empty() {
            result.push(Self::zero());
        }
        result
    }
}

pub(super) trait ReduceDigits<Output>: Sized {
    fn reduce_digits<const DIGIT_BITNESS: usize>(digits: &[Self]) -> Output;
}

impl<
        Digit: Copy,
        Output: BitOr<Output = Output>
            + Shl<usize, Output = Output>
            + From<Digit>
            + Zeroable,
    > ReduceDigits<Output> for Digit
{
    fn reduce_digits<const DIGIT_BITNESS: usize>(digits: &[Self]) -> Output {
        let mut result = Output::zero();
        for &digit in digits.iter().rev() {
            result = (result << DIGIT_BITNESS) | Output::from(digit);
        }
        result
    }
}

pub(super) trait MaybeReduceDigits<Output>: Sized {
    fn maybe_reduce_digits<const DIGIT_BITNESS: usize>(
        digits: &[Self],
    ) -> Option<Output>;
}

impl<
        Digit: Copy,
        Output: CheckedShl<u32, Output = Option<Output>>
            + BitOr<Output = Output>
            + TryFrom<Digit>
            + Zeroable,
    > MaybeReduceDigits<Output> for Digit
{
    fn maybe_reduce_digits<const DIGIT_BITNESS: usize>(
        digits: &[Self],
    ) -> Option<Output> {
        let mut result = Output::zero();
        for &digit in digits.iter().rev() {
            result = result.checked_shl(unsafe {
                u32::try_from(DIGIT_BITNESS).unwrap_unchecked()
            })? | Output::try_from(digit).ok()?;
        }
        Some(result)
    }
}

pub(super) trait ReduceDigitsToFloat<Output>: Sized {
    fn reduce_digits_to_float<const DIGIT_BITNESS: usize>(
        digits: &[Self],
    ) -> Output;
}

macro_rules! reduce_digits_to_float_impl {
    ($($float:ty)*) => ($(
        impl<Digit: Copy> ReduceDigitsToFloat<$float> for Digit
        where
            $float: From<Digit>,
        {
            fn reduce_digits_to_float<const DIGIT_BITNESS: usize>(
                digits: &[Self],
            ) -> $float {
                const EXPONENT_BASE: u32 =
                    (1 << (<f32>::EXPONENT_BITS_COUNT - 1usize)) - 1;
                debug_assert!(DIGIT_BITNESS < ((1 << f32::EXPONENT_BITS_COUNT) - 1));
                let mut result = 0 as $float;
                let scale = unsafe {
                    transmute::<u32, f32>(
                        (EXPONENT_BASE + (DIGIT_BITNESS as u32))
                            << f32::SIGNIFICAND_BITS_COUNT,
                    )
                } as $float;
                for &digit in digits.iter().rev() {
                    result = result * scale + <$float>::from(digit);
                }
                result
            }
        }
    )*)
}

reduce_digits_to_float_impl!(f32 f64);

pub(super) trait PrimitiveShiftDigitsLeft: Sized {
    fn primitive_shift_digits_left<const DIGIT_BITNESS: usize>(
        digits: &[Self],
        shift_quotient: usize,
        shift_remainder: Self,
    ) -> Option<Vec<Self>>;
}

impl<
        Digit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>> + Zeroable,
    > PrimitiveShiftDigitsLeft for Digit
where
    DoublePrecisionOf<Digit>: BitAnd<Output = DoublePrecisionOf<Digit>>
        + BitOrAssign
        + Copy
        + DigitMask
        + Shl<Digit, Output = DoublePrecisionOf<Digit>>
        + ShrAssign<usize>
        + Zeroable,
{
    fn primitive_shift_digits_left<const DIGIT_BITNESS: usize>(
        digits: &[Self],
        shift_quotient: usize,
        shift_remainder: Self,
    ) -> Option<Vec<Self>> {
        let mut result = Vec::<Self>::new();
        result
            .try_reserve_exact(
                digits.len()
                    + shift_quotient
                    + usize::from(!shift_remainder.is_zero()),
            )
            .ok()?;
        for _ in 0..shift_quotient {
            result.push(Self::zero());
        }
        let mut accumulator = DoublePrecisionOf::<Self>::zero();
        let digit_mask = DoublePrecisionOf::<Self>::digit_mask(DIGIT_BITNESS);
        for &digit in digits {
            accumulator |=
                DoublePrecisionOf::<Self>::from(digit) << shift_remainder;
            result.push(unsafe {
                Self::try_from(accumulator & digit_mask).unwrap_unchecked()
            });
            accumulator >>= DIGIT_BITNESS;
        }
        if !shift_remainder.is_zero() {
            result.push(unsafe {
                Self::try_from(accumulator).unwrap_unchecked()
            });
        }
        trim_leading_zeros(&mut result);
        Some(result)
    }
}

pub(super) trait ShiftDigitsLeft: Sized {
    fn shift_digits_left<const DIGIT_BITNESS: usize>(
        base: &[Self],
        shift: &[Self],
    ) -> Result<Vec<Self>, ShlError>;
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + MaybeReduceDigits<usize>
            + PrimitiveShiftDigitsLeft
            + TryFrom<usize>,
    > ShiftDigitsLeft for Digit
{
    fn shift_digits_left<const DIGIT_BITNESS: usize>(
        base: &[Self],
        shift: &[Self],
    ) -> Result<Vec<Self>, ShlError> {
        let (shift_quotient_digits, shift_remainder) =
            Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(shift, unsafe {
                Self::try_from(DIGIT_BITNESS).unwrap_unchecked()
            });
        let shift_quotient =
            Self::maybe_reduce_digits::<DIGIT_BITNESS>(&shift_quotient_digits)
                .ok_or(ShlError::TooLarge)?;
        if shift_quotient >= usize::MAX / size_of::<Self>() {
            Err(ShlError::TooLarge)
        } else {
            Self::primitive_shift_digits_left::<DIGIT_BITNESS>(
                base,
                shift_quotient,
                shift_remainder,
            )
            .ok_or(ShlError::OutOfMemory)
        }
    }
}

pub(super) trait ShiftDigitsLeftInPlace: Sized {
    fn shift_digits_left_in_place<const DIGIT_BITNESS: usize>(
        input: &[Self],
        shift: usize,
        output: &mut [Self],
    ) -> Self;
}

impl<
        Digit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Self>> + Zeroable,
    > ShiftDigitsLeftInPlace for Digit
where
    DoublePrecisionOf<Digit>: BitAnd<Output = DoublePrecisionOf<Digit>>
        + BitOrAssign
        + Copy
        + DigitMask
        + Shl<usize, Output = DoublePrecisionOf<Digit>>
        + ShrAssign<usize>
        + Zeroable,
{
    fn shift_digits_left_in_place<const DIGIT_BITNESS: usize>(
        input: &[Self],
        shift: usize,
        output: &mut [Self],
    ) -> Self {
        let mut accumulator = DoublePrecisionOf::<Self>::zero();
        let mask = DoublePrecisionOf::<Self>::digit_mask(DIGIT_BITNESS);
        for index in 0..input.len() {
            accumulator |=
                DoublePrecisionOf::<Self>::from(input[index]) << shift;
            output[index] = unsafe {
                Self::try_from(accumulator & mask).unwrap_unchecked()
            };
            accumulator >>= DIGIT_BITNESS;
        }
        unsafe { Self::try_from(accumulator).unwrap_unchecked() }
    }
}

pub(super) trait ShiftDigitsRightInPlace: Sized {
    fn shift_digits_right_in_place<const DIGIT_BITNESS: usize>(
        input: &[Self],
        shift: usize,
        output: &mut [Self],
    ) -> Self;
}

impl<
        Digit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<Self>> + Zeroable,
    > ShiftDigitsRightInPlace for Digit
where
    DoublePrecisionOf<Digit>: BitAndAssign
        + BitOrAssign
        + Copy
        + DigitMask
        + Shr<usize, Output = DoublePrecisionOf<Digit>>
        + ShlAssign<usize>
        + Zeroable,
{
    fn shift_digits_right_in_place<const DIGIT_BITNESS: usize>(
        input: &[Self],
        shift: usize,
        output: &mut [Self],
    ) -> Self {
        let mut accumulator = DoublePrecisionOf::<Self>::zero();
        let mask = DoublePrecisionOf::<Self>::digit_mask(shift);
        for index in (0..input.len()).rev() {
            accumulator <<= DIGIT_BITNESS;
            accumulator |= DoublePrecisionOf::<Self>::from(input[index]);
            output[index] = unsafe {
                Self::try_from(accumulator >> shift).unwrap_unchecked()
            };
            accumulator &= mask;
        }
        unsafe { Self::try_from(accumulator).unwrap_unchecked() }
    }
}

pub(super) trait PrimitiveShiftDigitsRight: Sized {
    fn primitive_shift_digits_right<const DIGIT_BITNESS: usize>(
        digits: &[Self],
        shift_quotient: usize,
        shift_remainder: Self,
    ) -> Vec<Self>;
}

impl<
        Digit: BitAnd<Output = Digit>
            + BitOrAssign
            + BitXor<Output = Digit>
            + Copy
            + DigitMask
            + Shl<usize, Output = Digit>
            + Shr<usize, Output = Digit>
            + Zeroable,
    > PrimitiveShiftDigitsRight for Digit
where
    usize: TryFrom<Digit>,
{
    fn primitive_shift_digits_right<const DIGIT_BITNESS: usize>(
        digits: &[Self],
        shift_quotient: usize,
        shift_remainder: Self,
    ) -> Vec<Self> {
        if digits.len() <= shift_quotient {
            return vec![Self::zero()];
        }
        let result_digits_count = digits.len() - shift_quotient;
        let shift_remainder =
            unsafe { usize::try_from(shift_remainder).unwrap_unchecked() };
        let high_shift = DIGIT_BITNESS - shift_remainder;
        let low_mask = Self::digit_mask(high_shift);
        let high_mask = Self::digit_mask(DIGIT_BITNESS) ^ low_mask;
        let mut result = vec![Self::zero(); result_digits_count];
        let mut position = shift_quotient;
        for index in 0..result_digits_count {
            result[index] = (digits[position] >> shift_remainder) & low_mask;
            if index + 1 < result_digits_count {
                result[index] |=
                    (digits[position + 1] << high_shift) & high_mask;
            }
            position += 1;
        }
        trim_leading_zeros(&mut result);
        result
    }
}

pub(super) trait ShiftDigitsRight: Sized {
    fn shift_digits_right<const DIGIT_BITNESS: usize>(
        base_sign: Sign,
        base: &[Self],
        shift: &[Self],
    ) -> (Sign, Vec<Self>);
}

impl<
        Digit: Copy
            + DivRemDigitsByDigit
            + InvertComponents
            + MaybeReduceDigits<usize>
            + PrimitiveShiftDigitsRight
            + TryFrom<usize>
            + Unitary
            + Zeroable,
    > ShiftDigitsRight for Digit
{
    fn shift_digits_right<const DIGIT_BITNESS: usize>(
        base_sign: Sign,
        base: &[Self],
        shift: &[Self],
    ) -> (Sign, Vec<Self>) {
        let (shift_quotient_digits, shift_remainder) =
            Self::div_rem_digits_by_digit::<DIGIT_BITNESS>(shift, unsafe {
                Self::try_from(DIGIT_BITNESS).unwrap_unchecked()
            });
        let shift_quotient =
            Self::maybe_reduce_digits::<DIGIT_BITNESS>(&shift_quotient_digits)
                .unwrap_or(usize::MAX / size_of::<Self>());
        if shift_quotient >= usize::MAX / size_of::<Self>() {
            if base_sign.is_negative() {
                (-Sign::one(), vec![Self::one(); 1])
            } else {
                (Sign::zero(), vec![Self::zero(); 1])
            }
        } else if base_sign.is_negative() {
            let (inverted_sign, inverted_digits) =
                Self::invert_components::<DIGIT_BITNESS>(base_sign, base);
            let digits = Self::primitive_shift_digits_right::<DIGIT_BITNESS>(
                &inverted_digits,
                shift_quotient,
                shift_remainder,
            );
            Self::invert_components::<DIGIT_BITNESS>(
                inverted_sign * to_digits_sign(&digits),
                &digits,
            )
        } else {
            let digits = Self::primitive_shift_digits_right::<DIGIT_BITNESS>(
                base,
                shift_quotient,
                shift_remainder,
            );
            (base_sign * to_digits_sign(&digits), digits)
        }
    }
}

fn split_digits<Digit>(
    digits: &[Digit],
    size: usize,
) -> (Vec<Digit>, Vec<Digit>)
where
    Digit: Clone + Zeroable,
{
    let (low, high) = digits.split_at(digits.len().min(size));
    let (mut low, mut high) = (low.to_vec(), high.to_vec());
    trim_leading_zeros(&mut high);
    trim_leading_zeros(&mut low);
    (high, low)
}

pub(super) trait SubtractComponents: Sized {
    fn subtract_components<const DIGIT_BITNESS: usize>(
        minuend_sign: Sign,
        minuend: &[Self],
        subtrahend_sign: Sign,
        subtrahend: &[Self],
    ) -> (Sign, Vec<Self>);
}

impl<Digit: SubtractDigits + SumDigits> SubtractComponents for Digit {
    fn subtract_components<const DIGIT_BITNESS: usize>(
        minuend_sign: Sign,
        minuend: &[Self],
        subtrahend_sign: Sign,
        subtrahend: &[Self],
    ) -> (Sign, Vec<Self>) {
        if minuend_sign.is_negative() {
            if subtrahend_sign.is_negative() {
                Self::subtract_digits::<DIGIT_BITNESS>(
                    subtrahend,
                    minuend,
                    Sign::one(),
                )
            } else {
                (
                    -Sign::one(),
                    Self::sum_digits::<DIGIT_BITNESS>(minuend, subtrahend),
                )
            }
        } else if subtrahend_sign.is_negative() {
            (
                Sign::one(),
                Self::sum_digits::<DIGIT_BITNESS>(minuend, subtrahend),
            )
        } else {
            Self::subtract_digits::<DIGIT_BITNESS>(
                minuend,
                subtrahend,
                Sign::one(),
            )
        }
    }
}

pub(super) trait SubtractDigits: Sized {
    fn subtract_digits<const DIGIT_BITNESS: usize>(
        minuend: &[Self],
        subtrahend: &[Self],
        sign: Sign,
    ) -> (Sign, Vec<Self>);
}

impl<
        Digit: BitAnd<Output = Digit>
            + BitAndAssign
            + Copy
            + DigitMask
            + PartialOrd
            + ShrAssign<usize>
            + Unitary
            + WrappingSub<Output = Digit>
            + Zeroable,
    > SubtractDigits for Digit
{
    fn subtract_digits<const DIGIT_BITNESS: usize>(
        minuend: &[Self],
        subtrahend: &[Self],
        mut sign: Sign,
    ) -> (Sign, Vec<Self>) {
        let mut longest = &minuend;
        let mut shortest = &subtrahend;
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
                    return (Sign::zero(), vec![Self::zero()]);
                }
                if longest[index] < shortest[index] {
                    (longest, shortest) = (shortest, longest);
                    sign = -sign;
                }
                longest_size = index + 1;
                shortest_size = index + 1;
            }
            Ordering::Greater => {}
        };
        let mut result = Vec::<Self>::with_capacity(longest_size);
        let mut accumulator = Self::zero();
        let digit_mask = Self::digit_mask(DIGIT_BITNESS);
        for index in 0..shortest_size {
            accumulator = longest[index]
                .wrapping_sub(shortest[index])
                .wrapping_sub(accumulator);
            result.push(accumulator & digit_mask);
            accumulator >>= DIGIT_BITNESS;
            accumulator &= Self::one();
        }
        for index in shortest_size..longest_size {
            accumulator = longest[index].wrapping_sub(accumulator);
            result.push(accumulator & digit_mask);
            accumulator >>= DIGIT_BITNESS;
            accumulator &= Self::one();
        }
        trim_leading_zeros(&mut result);
        (sign, result)
    }
}

pub(super) trait SubtractDigitsInPlace: Sized {
    fn subtract_digits_in_place<const DIGIT_BITNESS: usize>(
        longest: &mut [Self],
        shortest: &[Self],
    ) -> Self;
}

impl<
        Digit: BitAnd<Output = Digit>
            + BitAndAssign
            + Copy
            + DigitMask
            + ShrAssign<usize>
            + Unitary
            + WrappingSub<Output = Digit>
            + Zeroable,
    > SubtractDigitsInPlace for Digit
{
    fn subtract_digits_in_place<const DIGIT_BITNESS: usize>(
        longest: &mut [Self],
        shortest: &[Self],
    ) -> Self {
        let mut accumulator = Self::zero();
        let digit_mask = Self::digit_mask(DIGIT_BITNESS);
        for index in 0..shortest.len() {
            accumulator = longest[index]
                .wrapping_sub(shortest[index])
                .wrapping_sub(accumulator);
            longest[index] = accumulator & digit_mask;
            accumulator >>= DIGIT_BITNESS;
            accumulator &= Self::one();
        }
        for index in shortest.len()..longest.len() {
            if accumulator.is_zero() {
                break;
            }
            accumulator = longest[index].wrapping_sub(accumulator);
            longest[index] = accumulator & digit_mask;
            accumulator >>= DIGIT_BITNESS;
            accumulator &= Self::one();
        }
        accumulator
    }
}

pub(super) trait SumComponents: Sized {
    fn sum_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: &[Self],
        second_sign: Sign,
        second: &[Self],
    ) -> (Sign, Vec<Self>);
}

impl<Digit: SubtractDigits + SumDigits> SumComponents for Digit {
    fn sum_components<const DIGIT_BITNESS: usize>(
        first_sign: Sign,
        first: &[Self],
        second_sign: Sign,
        second: &[Self],
    ) -> (Sign, Vec<Self>) {
        if first_sign.is_negative() {
            if second_sign.is_negative() {
                (
                    -Sign::one(),
                    Self::sum_digits::<DIGIT_BITNESS>(first, second),
                )
            } else {
                Self::subtract_digits::<DIGIT_BITNESS>(
                    second,
                    first,
                    Sign::one(),
                )
            }
        } else if second_sign.is_negative() {
            Self::subtract_digits::<DIGIT_BITNESS>(first, second, Sign::one())
        } else {
            (
                first_sign.max(second_sign),
                Self::sum_digits::<DIGIT_BITNESS>(first, second),
            )
        }
    }
}

pub(super) trait SumDigits: Sized {
    fn sum_digits<const DIGIT_BITNESS: usize>(
        first: &[Self],
        second: &[Self],
    ) -> Vec<Self>;
}

impl<
        Digit: Add<Output = Digit>
            + AddAssign
            + BitAnd<Output = Digit>
            + Copy
            + DigitMask
            + ShrAssign<usize>
            + Zeroable,
    > SumDigits for Digit
{
    fn sum_digits<const DIGIT_BITNESS: usize>(
        first: &[Self],
        second: &[Self],
    ) -> Vec<Self> {
        let (longest, shortest) = if first.len() < second.len() {
            (&second, &first)
        } else {
            (&first, &second)
        };
        let mut result = Vec::<Self>::with_capacity(longest.len() + 1);
        let mut accumulator: Self = Self::zero();
        let digit_mask = Self::digit_mask(DIGIT_BITNESS);
        for index in 0..shortest.len() {
            accumulator += longest[index] + shortest[index];
            result.push(accumulator & digit_mask);
            accumulator >>= DIGIT_BITNESS;
        }
        for index in shortest.len()..longest.len() {
            accumulator += longest[index];
            result.push(accumulator & digit_mask);
            accumulator >>= DIGIT_BITNESS;
        }
        result.push(accumulator);
        trim_leading_zeros(&mut result);
        result
    }
}

pub(super) trait SumDigitsInPlace: Sized {
    fn sum_digits_in_place<const DIGIT_BITNESS: usize>(
        longest: &mut [Self],
        shortest: &[Self],
    ) -> Self;
}

impl<
        Digit: Add<Output = Digit>
            + AddAssign
            + BitAnd<Output = Digit>
            + Copy
            + DigitMask
            + ShrAssign<usize>
            + Zeroable,
    > SumDigitsInPlace for Digit
{
    fn sum_digits_in_place<const DIGIT_BITNESS: usize>(
        longest: &mut [Self],
        shortest: &[Self],
    ) -> Self {
        let mut accumulator = Self::zero();
        let digit_mask = Self::digit_mask(DIGIT_BITNESS);
        for index in 0..shortest.len() {
            accumulator = longest[index] + shortest[index] + accumulator;
            longest[index] = accumulator & digit_mask;
            accumulator >>= DIGIT_BITNESS;
        }
        for index in shortest.len()..longest.len() {
            if accumulator.is_zero() {
                break;
            }
            accumulator += longest[index];
            longest[index] = accumulator & digit_mask;
            accumulator >>= DIGIT_BITNESS;
        }
        accumulator
    }
}

pub trait DigitMask {
    fn digit_mask(bitness: usize) -> Self;
}

impl<Digit: Shl<usize, Output = Digit> + Sub<Output = Digit> + Unitary>
    DigitMask for Digit
{
    #[inline]
    fn digit_mask(bitness: usize) -> Self {
        (Self::one() << bitness) - Self::one()
    }
}

#[inline]
pub(super) fn to_digits_sign<Digit: Zeroable>(digits: &[Digit]) -> Sign {
    Sign::from(digits.len() > 1 || !digits[0].is_zero())
}

pub(super) trait GcdDigits: Sized {
    fn gcd_digits<const DIGIT_BITNESS: usize>(
        first: Vec<Self>,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>);
}

impl<
        Digit: BitLength<Output = usize>
            + Copy
            + DigitsFromNonZeroValue<DoublePrecisionOf<Digit>>
            + DivRemDigitsByDigit
            + DivRemDigitsByTwoOrMoreDigits
            + DoublePrecision
            + HasSignBit
            + PartialOrd
            + ReduceDigits<DoublePrecisionOf<Digit>>
            + Shr<usize, Output = Digit>
            + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
            + WrappingSub<Output = Digit>
            + Zeroable,
    > GcdDigits for Digit
where
    DoublePrecisionOf<Digit>: Gcd<Output = DoublePrecisionOf<Digit>>
        + HasSignBit
        + Oppose
        + RemEuclid<Output = DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + Zeroable,
    OppositionOf<DoublePrecisionOf<Digit>>: Add<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + AddAssign
        + BitAnd<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + BitOr<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + BitOrAssign
        + Copy
        + DigitMask
        + Div<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + From<Digit>
        + Mul<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + PartialOrd
        + RemEuclid<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + Shl<usize, Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + Shr<usize, Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + ShrAssign<usize>
        + Sub<Output = OppositionOf<DoublePrecisionOf<Digit>>>
        + SubAssign
        + TryFrom<DoublePrecisionOf<Digit>>
        + Unitary
        + Zeroable,
{
    fn gcd_digits<const DIGIT_BITNESS: usize>(
        first: Vec<Self>,
        second: Vec<Self>,
    ) -> (Sign, Vec<Self>) {
        let (mut largest, mut smallest) =
            if digits_lesser_than(&first, &second) {
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
            let highest_digit_bit_length =
                largest[largest.len() - 1].bit_length();
            let mut largest_leading_bits =
                (OppositionOf::<DoublePrecisionOf<Self>>::from(
                    largest[largest_digits_count - 1],
                ) << (2 * DIGIT_BITNESS - highest_digit_bit_length))
                    | (OppositionOf::<DoublePrecisionOf<Self>>::from(
                        largest[largest_digits_count - 2],
                    ) << (DIGIT_BITNESS - highest_digit_bit_length))
                    | OppositionOf::<DoublePrecisionOf<Self>>::from(
                        largest[largest_digits_count - 3]
                            >> highest_digit_bit_length,
                    );
            let mut smallest_leading_bits =
                if smallest_digits_count >= largest_digits_count - 2 {
                    OppositionOf::<DoublePrecisionOf<Self>>::from(
                        smallest[largest_digits_count - 3]
                            >> highest_digit_bit_length,
                    )
                } else {
                    OppositionOf::<DoublePrecisionOf<Self>>::zero()
                } | if smallest_digits_count >= largest_digits_count - 1 {
                    OppositionOf::<DoublePrecisionOf<Self>>::from(
                        smallest[largest_digits_count - 2],
                    ) << (DIGIT_BITNESS - highest_digit_bit_length)
                } else {
                    OppositionOf::<DoublePrecisionOf<Self>>::zero()
                } | if smallest_digits_count >= largest_digits_count {
                    OppositionOf::<DoublePrecisionOf<Self>>::from(
                        smallest[largest_digits_count - 1],
                    ) << (2 * DIGIT_BITNESS - highest_digit_bit_length)
                } else {
                    OppositionOf::<DoublePrecisionOf<Self>>::zero()
                };
            let mut first_coefficient =
                OppositionOf::<DoublePrecisionOf<Self>>::one();
            let mut second_coefficient =
                OppositionOf::<DoublePrecisionOf<Self>>::zero();
            let mut third_coefficient =
                OppositionOf::<DoublePrecisionOf<Self>>::zero();
            let mut fourth_coefficient =
                OppositionOf::<DoublePrecisionOf<Self>>::one();
            let mut iterations_count = 0usize;
            loop {
                if third_coefficient == smallest_leading_bits {
                    break;
                }
                let scale = (largest_leading_bits
                    + (first_coefficient
                        - OppositionOf::<DoublePrecisionOf<Self>>::one()))
                    / (smallest_leading_bits - third_coefficient);
                let next_third_coefficient =
                    second_coefficient + scale * fourth_coefficient;
                let next_smallest_leading_bits =
                    largest_leading_bits - scale * smallest_leading_bits;
                if next_third_coefficient > next_smallest_leading_bits {
                    break;
                }
                largest_leading_bits = smallest_leading_bits;
                smallest_leading_bits = next_smallest_leading_bits;
                let next_fourth_coefficient =
                    first_coefficient + scale * third_coefficient;
                first_coefficient = fourth_coefficient;
                second_coefficient = third_coefficient;
                third_coefficient = next_third_coefficient;
                fourth_coefficient = next_fourth_coefficient;
                iterations_count += 1;
            }
            if iterations_count == 0 {
                (largest, smallest) = if smallest_digits_count == 1 {
                    let (_, remainder) = Self::div_rem_digits_by_digit::<
                        DIGIT_BITNESS,
                    >(
                        &largest, smallest[0]
                    );
                    (smallest, vec![remainder])
                } else {
                    let (_, remainder) =
                        Self::div_rem_by_two_or_more_digits::<DIGIT_BITNESS>(
                            &largest, &smallest,
                        );
                    (smallest, remainder)
                };
                continue;
            }
            if iterations_count % 2 != 0 {
                (first_coefficient, second_coefficient) =
                    (-second_coefficient, -first_coefficient);
                (third_coefficient, fourth_coefficient) =
                    (-fourth_coefficient, -third_coefficient);
            }
            let digit_mask =
                OppositionOf::<DoublePrecisionOf<Self>>::digit_mask(
                    DIGIT_BITNESS,
                );
            let mut next_largest_accumulator =
                OppositionOf::<DoublePrecisionOf<Self>>::zero();
            let mut next_smallest_accumulator =
                OppositionOf::<DoublePrecisionOf<Self>>::zero();
            let mut next_largest_digits =
                Vec::<Self>::with_capacity(largest_digits_count);
            let mut next_smallest_digits =
                Vec::<Self>::with_capacity(largest_digits_count);
            for index in 0..smallest_digits_count {
                next_largest_accumulator = next_largest_accumulator
                    + (first_coefficient
                        * OppositionOf::<DoublePrecisionOf<Self>>::from(
                            largest[index],
                        ))
                    - (second_coefficient
                        * OppositionOf::<DoublePrecisionOf<Self>>::from(
                            smallest[index],
                        ));
                next_smallest_accumulator = next_smallest_accumulator
                    + (fourth_coefficient
                        * OppositionOf::<DoublePrecisionOf<Self>>::from(
                            smallest[index],
                        ))
                    - (third_coefficient
                        * OppositionOf::<DoublePrecisionOf<Self>>::from(
                            largest[index],
                        ));
                next_largest_digits.push(unsafe {
                    Self::try_from(next_largest_accumulator & digit_mask)
                        .unwrap_unchecked()
                });
                next_smallest_digits.push(unsafe {
                    Self::try_from(next_smallest_accumulator & digit_mask)
                        .unwrap_unchecked()
                });
                next_largest_accumulator >>= DIGIT_BITNESS;
                next_smallest_accumulator >>= DIGIT_BITNESS;
            }
            for index in smallest_digits_count..largest_digits_count {
                next_largest_accumulator += first_coefficient
                    * OppositionOf::<DoublePrecisionOf<Self>>::from(
                        largest[index],
                    );
                next_smallest_accumulator -= third_coefficient
                    * OppositionOf::<DoublePrecisionOf<Self>>::from(
                        largest[index],
                    );
                next_largest_digits.push(unsafe {
                    Self::try_from(next_largest_accumulator & digit_mask)
                        .unwrap_unchecked()
                });
                next_smallest_digits.push(unsafe {
                    Self::try_from(next_smallest_accumulator & digit_mask)
                        .unwrap_unchecked()
                });
                next_largest_accumulator >>= DIGIT_BITNESS;
                next_smallest_accumulator >>= DIGIT_BITNESS;
            }
            trim_leading_zeros(&mut next_largest_digits);
            trim_leading_zeros(&mut next_smallest_digits);
            largest = next_largest_digits;
            smallest = next_smallest_digits;
        }
        let reduced_result = Self::reduce_digits::<DIGIT_BITNESS>(&largest)
            .gcd(Self::reduce_digits::<DIGIT_BITNESS>(&smallest));
        if reduced_result.is_zero() {
            (Sign::zero(), vec![Self::zero(); 1])
        } else {
            (
                Sign::one(),
                Self::digits_from_non_zero_value::<DIGIT_BITNESS>(
                    reduced_result,
                ),
            )
        }
    }
}

pub(super) fn trim_leading_zeros<Digit: Zeroable>(digits: &mut Vec<Digit>) {
    let mut digits_count = digits.len();
    while digits_count > 1 && digits[digits_count - 1].is_zero() {
        digits_count -= 1;
    }
    if digits_count != digits.len() {
        digits.truncate(digits_count);
    }
}

pub(super) trait DigitsFromNonZeroValue<Source>: Sized {
    fn digits_from_non_zero_value<const DIGIT_BITNESS: usize>(
        value: Source,
    ) -> Vec<Self>;
}

impl<
        Digit: BitAnd<Output = Digit>
            + Copy
            + DigitMask
            + HasSignBit
            + Oppose
            + ShrAssign<usize>
            + TryFrom<Source>
            + Zeroable,
        Source: BitAnd<Output = Source>
            + Copy
            + DigitMask
            + HasSignBit
            + Oppose
            + ShrAssign<usize>
            + TryFrom<OppositionOf<Source>>
            + Zeroable,
    > DigitsFromNonZeroValue<Source> for Digit
where
    OppositionOf<Source>: TryFrom<Source>,
{
    fn digits_from_non_zero_value<const DIGIT_BITNESS: usize>(
        value: Source,
    ) -> Vec<Self> {
        if size_of::<Source>() < size_of::<Self>()
            || (size_of::<Source>() == size_of::<Self>()
                && is_signed::<Source>()
                && is_unsigned::<Self>())
        {
            let mut value = if is_signed::<Source>() {
                let value = unsafe {
                    OppositionOf::<Source>::try_from(value).unwrap_unchecked()
                };
                unsafe {
                    Self::try_from(
                        Source::try_from(if value.is_negative() {
                            -value
                        } else {
                            value
                        })
                        .unwrap_unchecked(),
                    )
                    .unwrap_unchecked()
                }
            } else {
                unsafe { Self::try_from(value).unwrap_unchecked() }
            };
            let mut digits = Vec::<Self>::new();
            let digit_mask = Self::digit_mask(DIGIT_BITNESS);
            while !value.is_zero() {
                digits.push(value & digit_mask);
                value >>= DIGIT_BITNESS;
            }
            digits
        } else {
            let mut value = if is_signed::<Source>() {
                let value = unsafe {
                    OppositionOf::<Source>::try_from(value).unwrap_unchecked()
                };
                if value.is_negative() {
                    unsafe { Source::try_from(-value).unwrap_unchecked() }
                } else {
                    unsafe { Source::try_from(value).unwrap_unchecked() }
                }
            } else {
                value
            };
            let mut digits = Vec::<Self>::new();
            let digit_mask = Source::digit_mask(DIGIT_BITNESS);
            while !value.is_zero() {
                digits.push(unsafe {
                    Self::try_from(value & digit_mask).unwrap_unchecked()
                });
                value >>= DIGIT_BITNESS;
            }
            digits
        }
    }
}

#[inline]
pub(super) fn value_to_sign<Source>(value: Source) -> Sign
where
    Source: HasSignBit + Oppose + Zeroable,
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
    Source: HasSignBit + Oppose,
    OppositionOf<Source>: TryFrom<Source>,
{
    if is_signed::<Source>()
        && unsafe {
            OppositionOf::<Source>::try_from(value).unwrap_unchecked()
        }
        .is_negative()
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
