use traiter::numbers::Zero;

use super::types::{BigInt, Sign};

impl<Digit: Zero, const DIGIT_BITNESS: usize> Zero
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn zero() -> Self {
        Self {
            sign: Sign::zero(),
            digits: vec![Digit::zero()],
        }
    }
}
