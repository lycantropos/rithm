use core::fmt;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, Sub, SubAssign};
use std::str::Chars;

use crate::digits::*;
use crate::traits::{
    Abs, AssigningDivisivePartialMagma, CheckedDiv, CheckedDivEuclid, CheckedRem, CheckedRemEuclid,
    DivEuclid, DivisivePartialMagma, DoublePrecision, DoublePrecisionOf, Gcd, ModularPartialMagma,
    ModularSubtractiveMagma, Oppose, OppositionOf, Oppositive, RemEuclid, Unitary, Zeroable,
};
use crate::utils;

#[derive(Clone, PartialEq, Eq)]
pub struct BigInt<Digit, const SEPARATOR: char, const SHIFT: usize> {
    sign: Sign,
    digits: Vec<Digit>,
}

const MAX_REPRESENTABLE_BASE: u8 = 36;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<u8>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<DoublePrecisionOf<Digit>>
        + Zeroable,
    DoublePrecisionOf<Digit>: BinaryDigit + From<u8> + TryFrom<usize>,
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

    pub fn new(string: &str, mut base: u8) -> Result<Self, String> {
        debug_assert!(Self::ASCII_CODES_DIGIT_VALUES[SEPARATOR as usize] >= MAX_REPRESENTABLE_BASE);
        if (base != 0 && base < 2) || base > MAX_REPRESENTABLE_BASE {
            return Err(format!(
                "Base should be zero or in range from 2 to {}.",
                MAX_REPRESENTABLE_BASE
            ));
        }
        let mut characters = string.trim().chars().peekable();
        let sign = if characters.peek() == Some(&'-') {
            characters.next();
            -Sign::one()
        } else if characters.peek() == Some(&'+') {
            characters.next();
            Sign::one()
        } else {
            Sign::one()
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
        if characters.peek() == Some(&SEPARATOR) {
            return Err(String::from("Should not start with separator."));
        }
        let mut result: Vec<u8> = Vec::new();
        let mut prev: char = SEPARATOR;
        for character in characters {
            if character != SEPARATOR {
                let digit = Self::ASCII_CODES_DIGIT_VALUES[character as usize];
                if digit >= base {
                    return Err(format!("Invalid digit in base {}: {}.", base, character));
                }
                result.push(digit);
            } else if prev == SEPARATOR {
                return Err(String::from("Consecutive separators found."));
            }
            prev = character;
        }
        if prev == SEPARATOR {
            return Err(String::from("Should not end with separator."));
        }
        result.reverse();
        Ok(result)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT> {
    pub(crate) fn digits(&self) -> &[Digit] {
        &self.digits
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
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
    pub(crate) fn checked_div_rem(self, divisor: &Self) -> Option<(Self, Self)> {
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularSubtractiveMagma
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<u8>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + From<u8> + Oppose + TryFrom<usize>,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<OppositionOf<Digit>>,
    usize: TryFrom<Digit>,
{
    pub fn from_str_radix(string: &str, radix: u32) -> Result<Self, String> {
        Self::new(string, radix as u8)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision + TryFrom<usize> + TryFrom<DoublePrecisionOf<Digit>> + Zeroable,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    fn from(mut value: DoublePrecisionOf<Digit>) -> Self {
        if value.is_zero() {
            Self::zero()
        } else {
            let sign = Sign::one();
            let mut digits = Vec::<Digit>::new();
            let digit_mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
            while !value.is_zero() {
                digits.push(unsafe { Digit::try_from(value & digit_mask).unwrap_unchecked() });
                value >>= SHIFT;
            }
            Self { sign, digits }
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: AssigningDivisivePartialMagma
        + BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularPartialMagma
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>:
        AssigningDivisivePartialMagma + BinaryDigit + ModularPartialMagma + TryFrom<usize>,
    <DoublePrecisionOf<Digit> as TryFrom<usize>>::Error: fmt::Debug,
    <Digit as TryFrom<DoublePrecisionOf<Digit>>>::Error: fmt::Debug,
    usize: TryFrom<Digit>,
{
    const DIGIT_VALUES_ASCII_CODES: [char; MAX_REPRESENTABLE_BASE as usize] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    fn to_base_string(&self, base: usize) -> String {
        let shift = utils::floor_log(1 << SHIFT, base).unwrap();
        let digits =
            binary_digits_to_base::<Digit, Digit>(&self.digits, SHIFT, utils::power(base, shift));
        let characters_count = (self.is_negative() as usize)
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
                    Self::DIGIT_VALUES_ASCII_CODES[unsafe {
                        usize::try_from(remainder.rem_euclid(target_base)).unwrap_unchecked()
                    }],
                );
                remainder /= target_base;
            }
        }
        let mut remainder = *digits.last().unwrap();
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Abs for BigInt<Digit, SEPARATOR, SHIFT> {
    type Output = Self;

    fn abs(self) -> Self {
        Self {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) =
            sum_signed_digits::<Digit, SHIFT>(&self.digits, self.sign, &other.digits, other.sign);
        Self { sign, digits }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> AddAssign for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            sum_signed_digits::<Digit, SHIFT>(&self.digits, self.sign, &other.digits, other.sign);
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Gcd for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularSubtractiveMagma
        + Oppose
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + DivisivePartialMagma + ModularPartialMagma + Oppose,
    OppositionOf<Digit>:
        BinaryDigit + TryFrom<OppositionOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    OppositionOf<DoublePrecisionOf<Digit>>: BinaryDigit
        + DivisivePartialMagma
        + From<Digit>
        + From<OppositionOf<Digit>>
        + ModularPartialMagma,
    usize: TryFrom<Digit>,
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
            normalize_digits(&mut next_largest_digits);
            normalize_digits(&mut next_smallest_digits);
            largest_digits = next_largest_digits;
            smallest_digits = next_smallest_digits;
        }
        Self::from(utils::gcd::<DoublePrecisionOf<Digit>>(
            reduce_digits::<Digit, DoublePrecisionOf<Digit>, SHIFT>(&largest_digits),
            reduce_digits::<Digit, DoublePrecisionOf<Digit>, SHIFT>(&smallest_digits),
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Display for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: AssigningDivisivePartialMagma
        + BinaryDigit
        + DoublePrecision
        + From<u8>
        + ModularPartialMagma
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>:
        AssigningDivisivePartialMagma + BinaryDigit + ModularPartialMagma + TryFrom<usize>,
    <DoublePrecisionOf<Digit> as TryFrom<usize>>::Error: fmt::Debug,
    <Digit as TryFrom<DoublePrecisionOf<Digit>>>::Error: fmt::Debug,
    usize: TryFrom<Digit>,
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(self.to_base_string(10).as_str())
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for BigInt<Digit, SEPARATOR, SHIFT>
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
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_div::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)?;
        Some(Self { sign, digits })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
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
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_div_euclid::<Digit, SHIFT>(
            &self.digits,
            self.sign,
            &divisor.digits,
            divisor.sign,
        )?;
        Some(Self { sign, digits })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
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
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        let (sign, digits) = checked_rem_euclid::<Digit, SHIFT>(
            &self.digits,
            self.sign,
            &divisor.digits,
            divisor.sign,
        )?;
        Some(Self { sign, digits })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for BigInt<Digit, SEPARATOR, SHIFT>
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
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)?;
        Some(Self { sign, digits })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Div for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivEuclid for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> DivAssign for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit
        + DoublePrecision
        + ModularSubtractiveMagma
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> MulAssign for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit
        + DoublePrecision
        + ModularSubtractiveMagma
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Not for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
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
        } else if other.lt(self) {
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
        } else if other.lt(self) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem for BigInt<Digit, SEPARATOR, SHIFT>
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
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(&self.digits, self.sign, &divisor.digits, divisor.sign)
                .unwrap();
        Self { sign, digits }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> RemEuclid for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> SubAssign for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + ModularSubtractiveMagma,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> TryFrom<&str>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<u8>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<DoublePrecisionOf<Digit>>
        + Zeroable,
    DoublePrecisionOf<Digit>: BinaryDigit + From<u8> + TryFrom<usize>,
{
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::new(string, 0)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Unitary for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: Unitary + Zeroable,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Zeroable for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: Zeroable,
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
