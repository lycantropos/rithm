use crate::traits::Zeroable;

use super::contracts::is_valid_shift;
use super::digits::ZeroableDigit;
use super::types::{BigInt, Sign};

impl<Digit: ZeroableDigit, const SEPARATOR: char, const SHIFT: usize> Zeroable
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn zero() -> Self {
        debug_assert!(is_valid_shift::<Digit, SHIFT>());
        Self {
            sign: Sign::zero(),
            digits: vec![Digit::zero()],
        }
    }

    fn is_zero(&self) -> bool {
        self.sign.is_zero()
    }
}
