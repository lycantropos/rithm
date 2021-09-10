use core::fmt;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::f64;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::ops::{Add, BitAnd, Div, Mul, Neg, Rem, Sub};
use std::str::Chars;

use crate::traits::{
    AssigningAdditiveMonoid, AssigningMultiplicativeMonoid, AssigningShiftingLeftMonoid,
    AssigningShiftingRightMonoid, BitwiseOrMonoid, DoublePrecision, DoublePrecisionOf, Gcd,
    ModularSub, One, Signed, SignedOf, Zero,
};
use crate::utils;

pub trait BinaryDigit = AssigningAdditiveMonoid
    + AssigningMultiplicativeMonoid
    + AssigningShiftingLeftMonoid<usize>
    + AssigningShiftingRightMonoid<usize>
    + BitAnd<Output = Self>
    + BitwiseOrMonoid
    + Copy
    + Div<Output = Self>
    + One
    + PartialOrd
    + Rem<Output = Self>
    + Sub<Output = Self>
    + Sized;

#[derive(Clone, PartialEq, Eq)]
pub struct BigInt<Digit, const SEPARATOR: char, const SHIFT: usize> {
    sign: Sign,
    digits: Vec<Digit>,
}

#[inline]
fn digits_lesser_than<Digit: PartialOrd>(left: &[Digit], right: &[Digit]) -> bool {
    left.len() < right.len()
        || left.len() == right.len() && left.iter().rev().lt(right.iter().rev())
}

impl<Digit: PartialOrd, const SEPARATOR: char, const SHIFT: usize> PartialOrd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
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

    fn gt(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && if self.sign > 0 {
                    digits_lesser_than(&other.digits, &self.digits)
                } else {
                    digits_lesser_than(&self.digits, &other.digits)
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

    fn lt(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && if self.sign > 0 {
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Gcd for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit
        + DoublePrecision
        + From<u8>
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit + Signed,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: BinaryDigit + From<Digit> + From<SignedOf<Digit>>,
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
                    sign: 1,
                    digits: largest_digits,
                };
            }
            let highest_digit_bit_length =
                utils::bit_length(largest_digits[largest_digits.len() - 1]);
            let mut largest_leading_bits = (SignedOf::<DoublePrecisionOf<Digit>>::from(
                largest_digits[largest_digits_count - 1],
            ) << (2 * SHIFT - highest_digit_bit_length))
                | (SignedOf::<DoublePrecisionOf<Digit>>::from(
                    largest_digits[largest_digits_count - 2],
                ) << (SHIFT - highest_digit_bit_length))
                | SignedOf::<DoublePrecisionOf<Digit>>::from(
                    largest_digits[largest_digits_count - 3] >> highest_digit_bit_length,
                );
            let mut smallest_leading_bits = if smallest_digits_count >= largest_digits_count - 2 {
                SignedOf::<DoublePrecisionOf<Digit>>::from(
                    smallest_digits[largest_digits_count - 3] >> highest_digit_bit_length,
                )
            } else {
                SignedOf::<DoublePrecisionOf<Digit>>::zero()
            } | if smallest_digits_count >= largest_digits_count - 1
            {
                SignedOf::<DoublePrecisionOf<Digit>>::from(
                    smallest_digits[largest_digits_count - 2],
                ) << (SHIFT - highest_digit_bit_length)
            } else {
                SignedOf::<DoublePrecisionOf<Digit>>::zero()
            } | if smallest_digits_count >= largest_digits_count {
                SignedOf::<DoublePrecisionOf<Digit>>::from(
                    smallest_digits[largest_digits_count - 1],
                ) << (2 * SHIFT - highest_digit_bit_length)
            } else {
                SignedOf::<DoublePrecisionOf<Digit>>::zero()
            };
            let mut first_coefficient = SignedOf::<DoublePrecisionOf<Digit>>::one();
            let mut second_coefficient = SignedOf::<DoublePrecisionOf<Digit>>::zero();
            let mut third_coefficient = SignedOf::<DoublePrecisionOf<Digit>>::zero();
            let mut fourth_coefficient = SignedOf::<DoublePrecisionOf<Digit>>::one();
            let mut iterations_count = 0usize;
            loop {
                if third_coefficient == smallest_leading_bits {
                    break;
                }
                let scale = (largest_leading_bits
                    + (first_coefficient - SignedOf::<DoublePrecisionOf<Digit>>::one()))
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
                    let (_, remainder) =
                        divrem_digits_by_digit::<Digit, SHIFT>(&largest_digits, smallest_digits[0]);
                    (smallest_digits, vec![remainder])
                } else {
                    let (_, remainder) = divrem_two_or_more_digits::<Digit, SHIFT>(
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
            let digit_mask = to_digit_mask::<SignedOf<DoublePrecisionOf<Digit>>>(SHIFT);
            let mut next_largest_accumulator = SignedOf::<DoublePrecisionOf<Digit>>::zero();
            let mut next_smallest_accumulator = SignedOf::<DoublePrecisionOf<Digit>>::zero();
            let mut next_largest_digits = Vec::<Digit>::with_capacity(largest_digits_count);
            let mut next_smallest_digits = Vec::<Digit>::with_capacity(largest_digits_count);
            for index in 0..smallest_digits_count {
                next_largest_accumulator = next_largest_accumulator
                    + (first_coefficient
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]))
                    - (second_coefficient
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(smallest_digits[index]));
                next_smallest_accumulator = next_smallest_accumulator
                    + (fourth_coefficient
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(smallest_digits[index]))
                    - (third_coefficient
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]));
                next_largest_digits.push(unsafe {
                    Digit::try_from(next_largest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_smallest_digits.push(unsafe {
                    Digit::try_from(next_smallest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_largest_accumulator = next_largest_accumulator >> SHIFT;
                next_smallest_accumulator = next_smallest_accumulator >> SHIFT;
            }
            for index in smallest_digits_count..largest_digits_count {
                next_largest_accumulator = next_largest_accumulator
                    + first_coefficient
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]);
                next_smallest_accumulator = next_smallest_accumulator
                    - third_coefficient
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(largest_digits[index]);
                next_largest_digits.push(unsafe {
                    Digit::try_from(next_largest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_smallest_digits.push(unsafe {
                    Digit::try_from(next_smallest_accumulator & digit_mask).unwrap_unchecked()
                });
                next_largest_accumulator = next_largest_accumulator >> SHIFT;
                next_smallest_accumulator = next_smallest_accumulator >> SHIFT;
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

impl<Digit: Eq + PartialOrd, const SEPARATOR: char, const SHIFT: usize> Ord
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

const MAX_REPRESENTABLE_BASE: u8 = 36;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<u8>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<DoublePrecisionOf<Digit>>
        + Zero,
    DoublePrecisionOf<Digit>: From<u8> + BinaryDigit + TryFrom<usize>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Display for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + TryFrom<usize>,
    <DoublePrecisionOf<Digit> as TryFrom<usize>>::Error: fmt::Debug,
    <Digit as TryFrom<DoublePrecisionOf<Digit>>>::Error: fmt::Debug,
    usize: TryFrom<Digit>,
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(self.to_base_string(10).as_str())
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Div for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit + Signed,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: From<Digit> + From<SignedOf<Digit>> + BinaryDigit,
    usize: TryFrom<Digit>,
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.divrem(&divisor).unwrap().0
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>
        + TryFrom<u8>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: From<u8> + BinaryDigit + Signed + TryFrom<usize>,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: From<Digit> + From<SignedOf<Digit>> + BinaryDigit,
    usize: TryFrom<Digit>,
{
    pub fn from_str_radix(string: &str, radix: u32) -> Result<Self, String> {
        Self::new(string, radix as u8)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit + Signed,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: From<Digit> + From<SignedOf<Digit>> + BinaryDigit,
    usize: TryFrom<Digit>,
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.divrem(&divisor).unwrap().1
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>,
    DoublePrecisionOf<Digit>: BinaryDigit + TryFrom<usize>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: From<bool> + BinaryDigit,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + TryFrom<usize> + ModularSub<Output = Digit>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.sign < 0 {
            if other.sign < 0 {
                Self {
                    sign: -1,
                    digits: sum_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits),
                }
            } else {
                let mut sign: Sign = 1;
                let digits = subtract_digits::<Digit, SEPARATOR, SHIFT>(
                    &other.digits,
                    &self.digits,
                    &mut sign,
                );
                Self { sign, digits }
            }
        } else if other.sign < 0 {
            let mut sign: Sign = 1;
            let digits =
                subtract_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits, &mut sign);
            Self { sign, digits }
        } else {
            Self {
                sign: self.sign | other.sign,
                digits: sum_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits),
            }
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + BinaryDigit
        + TryFrom<usize>
        + TryFrom<DoublePrecisionOf<Digit>>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    fn from(mut value: DoublePrecisionOf<Digit>) -> Self {
        if value.is_zero() {
            Self::zero()
        } else {
            let digit_mask = to_digit_mask::<DoublePrecisionOf<Digit>>(SHIFT);
            let sign = Sign::one();
            let mut digits: Vec<Digit> = Vec::new();
            while !value.is_zero() {
                digits.push(unsafe { Digit::try_from(value & digit_mask).unwrap_unchecked() });
                value = value >> SHIFT;
            }
            Self { sign, digits }
        }
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
        + Zero,
    DoublePrecisionOf<Digit>: From<u8> + BinaryDigit + TryFrom<usize>,
{
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::new(string, 0)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + BinaryDigit
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: multiply_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits),
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<DoublePrecisionOf<u8>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>
        + TryFrom<u8>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: From<u8> + BinaryDigit + Signed + TryFrom<usize>,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: From<Digit> + From<SignedOf<Digit>> + BinaryDigit,
    usize: TryFrom<Digit>,
{
    pub fn abs(&self) -> Self {
        Self {
            sign: self.sign.abs(),
            digits: self.digits.clone(),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.sign.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.sign.is_negative()
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit + Signed,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: From<Digit> + From<SignedOf<Digit>> + BinaryDigit,
    usize: TryFrom<Digit>,
{
    pub(crate) fn divrem(self, divisor: &Self) -> Result<(Self, Self), &'static str> {
        let digits_count = self.digits.len();
        let divisor_digits_count = divisor.digits.len();
        if divisor.sign == 0 {
            Err("Division by zero is undefined.")
        } else if self.sign == 0
            || digits_count < divisor_digits_count
            || (digits_count == divisor_digits_count
                && self.digits[self.digits.len() - 1] < divisor.digits[divisor.digits.len() - 1])
        {
            Ok((Self::zero(), self))
        } else if divisor_digits_count == 1 {
            let (quotient_digits, remainder_digit) =
                divrem_digits_by_digit::<Digit, SHIFT>(&self.digits, divisor.digits[0]);
            Ok((
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
                divrem_two_or_more_digits::<Digit, SHIFT>(&self.digits, &divisor.digits);
            Ok((
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + TryFrom<usize> + ModularSub<Output = Digit>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        if self.sign < 0 {
            if other.sign < 0 {
                let mut sign: Sign = 1;
                let digits = subtract_digits::<Digit, SEPARATOR, SHIFT>(
                    &other.digits,
                    &self.digits,
                    &mut sign,
                );
                Self { sign, digits }
            } else {
                Self {
                    sign: -1,
                    digits: sum_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits),
                }
            }
        } else if other.sign < 0 {
            Self {
                sign: 1,
                digits: sum_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits),
            }
        } else {
            let mut sign: Sign = 1;
            let digits =
                subtract_digits::<Digit, SEPARATOR, SHIFT>(&self.digits, &other.digits, &mut sign);
            Self { sign, digits }
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> One for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: DoublePrecision
        + BinaryDigit
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    fn one() -> Self {
        Self {
            sign: Sign::one(),
            digits: vec![Digit::one()],
        }
    }

    fn is_one(&self) -> bool {
        self.sign.is_positive() && self.digits.len() == 1 && self.digits[0].is_one()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Zero for BigInt<Digit, SEPARATOR, SHIFT>
where
    Digit: BinaryDigit + TryFrom<usize> + ModularSub<Output = Digit>,
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

pub(crate) type Sign = i8;

fn binary_digits_to_base<SourceDigit, TargetDigit>(
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_base: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: Copy + DoublePrecision + From<u8> + BinaryDigit,
    TargetDigit: Copy
        + DoublePrecision
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + Zero,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    DoublePrecisionOf<TargetDigit>:
        From<SourceDigit> + From<TargetDigit> + BinaryDigit + TryFrom<usize>,
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

fn digits_to_binary_base<SourceDigit, TargetDigit>(
    source_digits: &[SourceDigit],
    source_base: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: DoublePrecision + From<u8> + BinaryDigit,
    TargetDigit: Copy
        + DoublePrecision
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>
        + Zero,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    DoublePrecisionOf<TargetDigit>:
        From<SourceDigit> + From<TargetDigit> + BinaryDigit + TryFrom<usize>,
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
    SourceDigit: DoublePrecision + From<u8> + BinaryDigit,
    TargetDigit: DoublePrecision
        + TryFrom<SourceDigit>
        + TryFrom<DoublePrecisionOf<SourceDigit>>
        + TryFrom<DoublePrecisionOf<TargetDigit>>,
    DoublePrecisionOf<SourceDigit>: BinaryDigit,
    DoublePrecisionOf<TargetDigit>: From<SourceDigit> + BinaryDigit,
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
    TargetDigit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>> + Zero,
    DoublePrecisionOf<TargetDigit>:
        From<SourceDigit> + From<TargetDigit> + BinaryDigit + TryFrom<usize>,
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
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: BinaryDigit,
    TargetDigit: DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>>,
    DoublePrecisionOf<TargetDigit>: From<SourceDigit> + BinaryDigit,
{
    let target_digit_mask = to_digit_mask::<DoublePrecisionOf<TargetDigit>>(target_shift);
    let result_capacity: usize =
        (source_digits.len() * target_shift + (target_shift - 1)) / target_shift;
    let mut result = Vec::<TargetDigit>::with_capacity(result_capacity);
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
    source_digits: &[SourceDigit],
    source_shift: usize,
    target_shift: usize,
) -> Vec<TargetDigit>
where
    SourceDigit: DoublePrecision + From<u8> + BinaryDigit,
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

fn divrem_digits_by_digit<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    divisor: Digit,
) -> (Vec<Digit>, Digit)
where
    Digit: DoublePrecision + BinaryDigit + TryFrom<DoublePrecisionOf<Digit>>,
    DoublePrecisionOf<Digit>: BinaryDigit,
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
        remainder = remainder - DoublePrecisionOf::<Digit>::from(quotient_digit) * divisor;
    }
    normalize_digits(&mut quotient);
    (quotient, unsafe {
        Digit::try_from(remainder).unwrap_unchecked()
    })
}

fn divrem_two_or_more_digits<Digit, const SHIFT: usize>(
    dividend: &[Digit],
    divisor: &[Digit],
) -> (Vec<Digit>, Vec<Digit>)
where
    Digit: DoublePrecision
        + From<u8>
        + BinaryDigit
        + Signed
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<SignedOf<DoublePrecisionOf<Digit>>>,
    DoublePrecisionOf<Digit>: BinaryDigit + Signed,
    SignedOf<Digit>: BinaryDigit + TryFrom<SignedOf<DoublePrecisionOf<Digit>>> + TryFrom<Digit>,
    SignedOf<DoublePrecisionOf<Digit>>: From<Digit> + From<SignedOf<Digit>> + BinaryDigit,
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
            quotient_digit = quotient_digit - Digit::one();
            step_remainder = step_remainder + last_divisor_digit_normalized;
            if step_remainder >= base {
                break;
            }
        }
        let mut accumulator = SignedOf::<Digit>::zero();
        for index in 0..divisor_digits_count {
            let step =
                SignedOf::<DoublePrecisionOf<Digit>>::from(dividend_normalized[offset + index])
                    + SignedOf::<DoublePrecisionOf<Digit>>::from(accumulator)
                    - SignedOf::<DoublePrecisionOf<Digit>>::from(quotient_digit)
                        * SignedOf::<DoublePrecisionOf<Digit>>::from(divisor_normalized[index]);
            dividend_normalized[offset + index] = unsafe {
                Digit::try_from(step & SignedOf::<DoublePrecisionOf<Digit>>::from(digit_mask))
                    .unwrap_unchecked()
            };
            accumulator = unsafe { SignedOf::<Digit>::try_from(step >> SHIFT).unwrap_unchecked() };
        }
        if unsafe {
            SignedOf::<Digit>::try_from(dividend_normalized[offset + divisor_digits_count])
                .unwrap_unchecked()
        } + accumulator
            < SignedOf::<Digit>::zero()
        {
            let mut accumulator = Digit::zero();
            for index in 0..divisor_digits_count {
                accumulator =
                    accumulator + dividend_normalized[offset + index] + divisor_normalized[index];
                dividend_normalized[offset + index] = accumulator & digit_mask;
                accumulator = accumulator >> SHIFT;
            }
            quotient_digit = quotient_digit - Digit::one();
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
    TargetDigit: Copy + DoublePrecision + TryFrom<DoublePrecisionOf<TargetDigit>> + Zero,
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
            digit = digit
                + DoublePrecisionOf::<TargetDigit>::from(*result_position)
                    * unsafe {
                        DoublePrecisionOf::<TargetDigit>::try_from(base_power).unwrap_unchecked()
                    };
            *result_position =
                unsafe { TargetDigit::try_from(digit & target_digit_mask).unwrap_unchecked() };
            digit = digit >> target_shift;
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

fn multiply_digits<Digit, const SEPARATOR: char, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
) -> Vec<Digit>
where
    Digit: DoublePrecision
        + BinaryDigit
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
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
            multiply_digits_plain::<Digit, SEPARATOR, SHIFT>(*shortest, *longest)
        };
    };
    if 2 * size_shortest <= size_longest {
        return multiply_digits_lopsided::<Digit, SEPARATOR, SHIFT>(*shortest, *longest);
    }
    let shift = size_longest >> 1;
    let (shortest_high, shortest_low) = split_digits(*shortest, shift);
    let (longest_high, longest_low) = if shortest.as_ptr() == longest.as_ptr() {
        (shortest_high.clone(), shortest_low.clone())
    } else {
        split_digits(*longest, shift)
    };
    let mut result = vec![Digit::zero(); size_shortest + size_longest];
    let highs_product = multiply_digits::<Digit, SEPARATOR, SHIFT>(&shortest_high, &longest_high);
    for (index, &digit) in highs_product.iter().enumerate() {
        result[index + 2 * shift] = digit;
    }
    let lows_product = multiply_digits::<Digit, SEPARATOR, SHIFT>(&shortest_low, &longest_low);
    for (index, &digit) in lows_product.iter().enumerate() {
        result[index] = digit;
    }
    subtract_digits_in_place::<Digit, SEPARATOR, SHIFT>(&mut result[shift..], &lows_product);
    subtract_digits_in_place::<Digit, SEPARATOR, SHIFT>(&mut result[shift..], &highs_product);
    let shortest_components_sum =
        sum_digits::<Digit, SEPARATOR, SHIFT>(&shortest_high, &shortest_low);
    let longest_components_sum = if shortest.as_ptr() == longest.as_ptr() {
        shortest_components_sum.clone()
    } else {
        sum_digits::<Digit, SEPARATOR, SHIFT>(&longest_high, &longest_low)
    };
    let components_sums_product = multiply_digits::<Digit, SEPARATOR, SHIFT>(
        &shortest_components_sum,
        &longest_components_sum,
    );
    sum_digits_in_place::<Digit, SEPARATOR, SHIFT>(&mut result[shift..], &components_sums_product);
    normalize_digits(&mut result);
    result
}

fn multiply_digits_lopsided<Digit, const SEPARATOR: char, const SHIFT: usize>(
    shortest: &[Digit],
    longest: &[Digit],
) -> Vec<Digit>
where
    Digit: DoublePrecision
        + BinaryDigit
        + TryFrom<DoublePrecisionOf<Digit>>
        + TryFrom<usize>
        + ModularSub<Output = Digit>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    let size_shortest = shortest.len();
    let mut size_longest = longest.len();
    let mut result = vec![Digit::zero(); size_shortest + size_longest];
    let mut processed_digits_count = 0;
    while size_longest > 0 {
        let step_digits_count = size_longest.min(size_shortest);
        let product = multiply_digits::<Digit, SEPARATOR, SHIFT>(
            shortest,
            &longest[processed_digits_count..processed_digits_count + step_digits_count].to_vec(),
        );
        sum_digits_in_place::<Digit, SEPARATOR, SHIFT>(
            &mut result[processed_digits_count..],
            &product,
        );
        size_longest -= step_digits_count;
        processed_digits_count += step_digits_count;
    }
    normalize_digits(&mut result);
    result
}

fn multiply_digits_plain<Digit, const SEPARATOR: char, const SHIFT: usize>(
    shortest: &[Digit],
    longest: &[Digit],
) -> Vec<Digit>
where
    Digit: BinaryDigit + DoublePrecision + TryFrom<DoublePrecisionOf<Digit>>,
    DoublePrecisionOf<Digit>: BinaryDigit,
{
    let size_shortest = shortest.len();
    let size_longest = longest.len();
    let mut result: Vec<Digit> = vec![Digit::zero(); size_shortest + size_longest];
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
            accumulator = accumulator >> SHIFT;
            digit = digit << 1;
            for next_index in index + 1..shortest.len() {
                accumulator = accumulator
                    + DoublePrecisionOf::<Digit>::from(result[result_position])
                    + DoublePrecisionOf::<Digit>::from(shortest[next_index]) * digit;
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

fn shift_digits_left<Digit, const SHIFT: usize>(
    input_digits: &[Digit],
    shift: usize,
    output_digits: &mut [Digit],
) -> Digit
where
    Digit: DoublePrecision + BinaryDigit + TryFrom<DoublePrecisionOf<Digit>>,
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
    Digit: DoublePrecision + BinaryDigit + TryFrom<DoublePrecisionOf<Digit>>,
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
    Digit: Clone + Zero,
{
    let (low, high) = digits.split_at(digits.len().min(size));
    let (mut low, mut high) = (low.to_vec(), high.to_vec());
    normalize_digits(&mut high);
    normalize_digits(&mut low);
    (high, low)
}

fn subtract_digits<Digit, const SEPARATOR: char, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
    sign: &mut Sign,
) -> Vec<Digit>
where
    Digit: BinaryDigit + TryFrom<usize> + ModularSub<Output = Digit>,
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

fn subtract_digits_in_place<Digit, const SEPARATOR: char, const SHIFT: usize>(
    longest: &mut [Digit],
    shortest: &[Digit],
) -> Digit
where
    Digit: BinaryDigit + ModularSub<Output = Digit>,
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

fn sum_digits<Digit, const SEPARATOR: char, const SHIFT: usize>(
    first: &[Digit],
    second: &[Digit],
) -> Vec<Digit>
where
    Digit: BinaryDigit + TryFrom<usize>,
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

fn sum_digits_in_place<Digit, const SEPARATOR: char, const SHIFT: usize>(
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

fn normalize_digits<Digit>(digits: &mut Vec<Digit>)
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

fn reduce_digits<Digit, Output, const SHIFT: usize>(digits: &[Digit]) -> Output
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
fn to_digit_mask<Digit>(shift: usize) -> Digit
where
    Digit: BinaryDigit,
{
    (Digit::one() << shift) - Digit::one()
}
