use core::convert::TryFrom;

use crate::traits::{Oppose, OppositionOf};

use super::contracts::is_valid_shift;
use super::digits::{non_zero_value_to_digits, non_zero_value_to_sign, BinaryDigit};
use super::types::BigInt;

impl<Source, Digit, const SEPARATOR: char, const SHIFT: usize> From<Source>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Source: BinaryDigit + Oppose + TryFrom<OppositionOf<Source>>,
    Digit: BinaryDigit + Oppose + TryFrom<Source>,
    OppositionOf<Source>: TryFrom<Source>,
{
    fn from(value: Source) -> Self {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        if value.is_zero() {
            Self::zero()
        } else {
            Self {
                sign: non_zero_value_to_sign(value),
                digits: non_zero_value_to_digits::<Source, Digit, SHIFT>(value),
            }
        }
    }
}
