use std::convert::TryFrom;

use super::digits::{BinaryDigit, MultiplicativeDigit};
use super::types::BigInt;
use crate::traits::{BitLength, Oppose};

impl<
        Digit: BitLength<Output = usize> + BinaryDigit + MultiplicativeDigit + Oppose + TryFrom<usize>,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitLength for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bit_length(self) -> Self::Output {
        if self.digits.len() <= usize::MAX / SHIFT {
            Self::from(
                (self.digits.len() - 1) * SHIFT + self.digits[self.digits.len() - 1].bit_length(),
            )
        } else {
            Self::from(self.digits.len() - 1) * Self::from(SHIFT)
                + Self::from(self.digits[self.digits.len() - 1].bit_length())
        }
    }
}
