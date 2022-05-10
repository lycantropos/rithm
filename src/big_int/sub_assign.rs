use std::ops::SubAssign;

use super::digits::{subtract_components, AdditiveDigit};
use super::types::BigInt;

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> SubAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.sign, self.digits) = subtract_components::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}

impl<Digit: AdditiveDigit, const SEPARATOR: char, const SHIFT: usize> SubAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: &Self) {
        (self.sign, self.digits) = subtract_components::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}
