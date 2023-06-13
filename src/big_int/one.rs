use traiter::numbers::One;

use super::types::{BigInt, Sign};

impl<Digit: One, const DIGIT_BITNESS: usize> One
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn one() -> Self {
        Self {
            sign: Sign::one(),
            digits: vec![Digit::one()],
        }
    }
}
