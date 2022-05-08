use crate::traits::{Oppositive, Unitary};

use super::contracts::is_valid_shift;
use super::digits::UnitaryDigit;
use super::types::{BigInt, Sign};

impl<Digit: UnitaryDigit, const SEPARATOR: char, const SHIFT: usize> Unitary
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn one() -> Self {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        Self {
            sign: Sign::one(),
            digits: vec![Digit::one()],
        }
    }

    fn is_one(&self) -> bool {
        self.is_positive() && self.digits.len() == 1 && self.digits[0].is_one()
    }
}
