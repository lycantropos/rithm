use traiter::numbers::Unitary;

use super::types::{BigInt, Sign};

impl<Digit: Unitary, const DIGIT_BITNESS: usize> Unitary
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn one() -> Self {
        Self {
            sign: Sign::one(),
            digits: vec![Digit::one()],
        }
    }

    fn is_one(&self) -> bool {
        self.sign.is_positive()
            && self.digits.len() == 1
            && self.digits[0].is_one()
    }
}
