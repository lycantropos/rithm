#![feature(associated_type_defaults)]
#![feature(option_result_unwrap_unchecked)]

use std::f64;
use std::iter::Peekable;
use std::str::Chars;

use num::Zero;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::ffi::{Py_hash_t, Py_uhash_t};
use pyo3::prelude::*;

mod big_int;
mod utils;

type Sign = i8;
#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;
type DoubleDigit = big_int::DoublePrecision<Digit>;

const BINARY_SHIFT: usize = (Digit::BITS - 1) as usize;
const BINARY_BASE: DoubleDigit = 1 << BINARY_SHIFT;
const DECIMAL_SHIFT: usize = utils::floor_log10(BINARY_BASE as usize);
const DECIMAL_BASE: usize = utils::power(10, DECIMAL_SHIFT);
const SEPARATOR: char = '_';
const MAX_REPRESENTABLE_BASE: u8 = 36;
const ASCII_CODES_DIGIT_VALUES: [u8; 256] = [
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 37, 37, 37, 37, 37, 37, 37, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 37, 37, 37, 37, 37, 37, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
    37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37, 37,
];

#[pyclass(module = "rithm", subclass)]
struct Int {
    sign: Sign,
    digits: Vec<Digit>,
}

#[pymethods]
impl Int {
    #[new]
    #[args(_string = "\"0\"", base = 10)]
    fn new(_string: &str, mut base: u8) -> PyResult<Self> {
        if (base != 0 && base < 2) || base > MAX_REPRESENTABLE_BASE {
            return Err(PyValueError::new_err(format!(
                "Base should be zero or in range from 2 to {}.",
                MAX_REPRESENTABLE_BASE
            )));
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
        let digits = big_int::digits_to_binary_base::<u8, Digit>(
            &parse_digits(characters, base)?,
            base as usize,
            BINARY_SHIFT,
        );
        Ok(Int {
            sign: sign * ((digits.len() > 1 || digits[0] != 0) as Sign),
            digits,
        })
    }
}

fn parse_digits(mut characters: Peekable<Chars>, base: u8) -> PyResult<Vec<u8>> {
    if characters.peek() == Some(&SEPARATOR) {
        return Err(PyValueError::new_err("Should not start with separator."));
    }
    let mut result: Vec<u8> = Vec::new();
    let mut prev: char = SEPARATOR;
    while let Some(character) = characters.next() {
        if character != SEPARATOR {
            let digit = ASCII_CODES_DIGIT_VALUES[character as usize];
            if digit >= base {
                return Err(PyValueError::new_err(format!(
                    "Invalid digit in base {}: {}.",
                    base, character
                )));
            }
            result.push(digit);
        } else if prev == SEPARATOR {
            return Err(PyValueError::new_err("Consecutive separators found."));
        }
        prev = character;
    }
    if prev == SEPARATOR {
        return Err(PyValueError::new_err("Should not end with separator."));
    }
    result.reverse();
    Ok(result)
}

#[cfg(target_arch = "x86")]
const HASH_BITS: usize = 31;
#[cfg(not(target_arch = "x86"))]
const HASH_BITS: usize = 61;
const HASH_MODULUS: usize = (1 << HASH_BITS) - 1;

#[pyproto]
impl PyObjectProtocol for Int {
    fn __hash__(&self) -> PyResult<Py_hash_t> {
        let sign = self.sign;
        let digits = &self.digits;
        if digits.len() == 1 {
            return if sign < 0 {
                Ok(-((digits[0] + ((digits[0] == 1) as Digit)) as Py_hash_t))
            } else {
                Ok(digits[0] as Py_hash_t)
            };
        }
        let mut result: Py_uhash_t = 0;
        for position in digits.iter().rev() {
            result =
                ((result << BINARY_SHIFT) & HASH_MODULUS) | (result >> (HASH_BITS - BINARY_SHIFT));
            result += *position as Py_uhash_t;
            if result >= HASH_MODULUS {
                result -= HASH_MODULUS;
            }
        }
        if sign < 0 {
            result = Py_uhash_t::MAX - result + 1
        };
        result -= (result == Py_uhash_t::MAX) as Py_uhash_t;
        Ok(result as Py_hash_t)
    }

    fn __repr__(&self) -> PyResult<String> {
        let base_digits: Vec<Digit> = big_int::binary_digits_to_base::<Digit, Digit>(
            &self.digits,
            BINARY_SHIFT,
            DECIMAL_BASE,
        );
        let characters_count: usize = ((self.sign < 0) as usize)
            + (base_digits.len() - 1) * DECIMAL_SHIFT
            + utils::floor_log10(*base_digits.last().unwrap() as usize)
            + 1;
        let mut characters: String = String::with_capacity(characters_count);
        for index in 0..base_digits.len() - 1 {
            let mut remainder = base_digits[index];
            for _ in 0..DECIMAL_SHIFT {
                characters.push((('0' as u8) + ((remainder % 10) as u8)) as char);
                remainder /= 10;
            }
        }
        let mut remainder = *base_digits.last().unwrap();
        while !remainder.is_zero() {
            characters.push((('0' as u8) + ((remainder % 10) as u8)) as char);
            remainder /= 10;
        }
        if self.sign == 0 {
            characters.push('0');
        } else if self.sign < 0 {
            characters.push('-');
        }
        Ok(characters.chars().rev().collect())
    }
}

#[pymodule]
fn _rithm(_py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    module.setattr("__doc__", env!("CARGO_PKG_DESCRIPTION"))?;
    module.add_class::<Int>()?;
    Ok(())
}
