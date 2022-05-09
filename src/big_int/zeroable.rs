use crate::traits::Zeroable;

use super::types::{BigInt, Sign};

impl<Digit: Zeroable, const SEPARATOR: char, const SHIFT: usize> Zeroable
    for BigInt<Digit, SEPARATOR, SHIFT>
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
