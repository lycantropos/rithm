use std::fmt::{Debug, Display, Formatter};

use super::constants::{MAX_REPRESENTABLE_BASE, MIN_REPRESENTABLE_BASE};

pub(super) type Sign = i8;
pub(super) type WindowDigit = u8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigInt<Digit, const SEPARATOR: char, const SHIFT: usize> {
    pub(super) sign: Sign,
    pub(super) digits: Vec<Digit>,
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT> {
    pub(crate) fn digits(&self) -> &[Digit] {
        &self.digits
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CheckedDivAsFloatError {
    TooLarge,
    ZeroDivision,
}

impl CheckedDivAsFloatError {
    fn description(&self) -> &str {
        match self {
            CheckedDivAsFloatError::TooLarge => {
                "Division result too large to be expressed as floating point."
            }
            CheckedDivAsFloatError::ZeroDivision => "Division by zero is undefined.",
        }
    }
}

impl Debug for CheckedDivAsFloatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for CheckedDivAsFloatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CheckedPowRemEuclidError {
    ZeroDivisor,
    NonInvertibleBase,
}

impl CheckedPowRemEuclidError {
    fn description(&self) -> &str {
        match self {
            CheckedPowRemEuclidError::ZeroDivisor => "Divisor should not be zero.",
            CheckedPowRemEuclidError::NonInvertibleBase => {
                "Base is not invertible for the given divisor."
            }
        }
    }
}

impl Debug for CheckedPowRemEuclidError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for CheckedPowRemEuclidError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ShlError {
    NegativeShift,
    OutOfMemory,
    TooLarge,
}

impl ShlError {
    fn description(&self) -> String {
        match self {
            ShlError::NegativeShift => String::from("Shift by negative step is undefined."),
            ShlError::OutOfMemory => String::from("Not enough memory for shift result."),
            ShlError::TooLarge => String::from("Too large shift step."),
        }
    }
}

impl Debug for ShlError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for ShlError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ShrError {
    NegativeShift,
}

impl ShrError {
    fn description(&self) -> String {
        match self {
            ShrError::NegativeShift => String::from("Shift by negative step is undefined."),
        }
    }
}

impl Debug for ShrError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for ShrError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TryFromFloatError {
    Infinity,
    NaN,
}

impl TryFromFloatError {
    fn description(&self) -> &str {
        match self {
            TryFromFloatError::Infinity => "Conversion of infinity is undefined.",
            TryFromFloatError::NaN => "Conversion of NaN is undefined.",
        }
    }
}

impl Debug for TryFromFloatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for TryFromFloatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TryFromStringError {
    BaseOutOfBounds(u32),
    ConsecutiveSeparators,
    EndsWithSeparator,
    InvalidDigit(char, u8),
    StartsWithSeparator,
}

impl TryFromStringError {
    fn description(&self) -> String {
        match self {
            TryFromStringError::BaseOutOfBounds(base) => {
                format!(
                    "Base should be zero or in range from {MIN_REPRESENTABLE_BASE} \
                     to {MAX_REPRESENTABLE_BASE}, but found: {}.",
                    base
                )
            }
            TryFromStringError::ConsecutiveSeparators => {
                String::from("Consecutive separators found.")
            }
            TryFromStringError::EndsWithSeparator => String::from("Should not end with separator."),
            TryFromStringError::InvalidDigit(character, base) => {
                format!("Invalid digit in base {}: {:?}.", base, character)
            }
            TryFromStringError::StartsWithSeparator => {
                String::from("Should not start with separator.")
            }
        }
    }
}

impl Debug for TryFromStringError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.description())
    }
}

impl Display for TryFromStringError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TryIntoFloatError {
    TooLarge,
}

impl TryIntoFloatError {
    fn description(&self) -> &str {
        match self {
            TryIntoFloatError::TooLarge => "Too large to convert to floating point.",
        }
    }
}

impl Debug for TryIntoFloatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for TryIntoFloatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TryIntoSignedIntegerError {
    TooLarge,
}

impl TryIntoSignedIntegerError {
    fn description(&self) -> &str {
        match self {
            TryIntoSignedIntegerError::TooLarge => {
                "Value too large to be expressed as given signed integer type."
            }
        }
    }
}

impl Debug for TryIntoSignedIntegerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for TryIntoSignedIntegerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TryIntoUnsignedIntegerError {
    TooLarge,
    Negative,
}

impl TryIntoUnsignedIntegerError {
    fn description(&self) -> &str {
        match self {
            TryIntoUnsignedIntegerError::Negative => {
                "Negative value cannot be expressed by unsigned integer type."
            }
            TryIntoUnsignedIntegerError::TooLarge => {
                "Value too large to be expressed by given unsigned integer type."
            }
        }
    }
}

impl Debug for TryIntoUnsignedIntegerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for TryIntoUnsignedIntegerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}
