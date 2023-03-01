use traiter::numbers::Zeroable;

use super::types::{BigInt, Sign};

impl<Digit: Zeroable, const DIGIT_BITNESS: usize> Zeroable
    for BigInt<Digit, DIGIT_BITNESS>
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
