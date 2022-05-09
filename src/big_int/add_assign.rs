use std::ops::AddAssign;

use super::digits::{sum_components, AdditiveGroupDigit};
use super::types::BigInt;

impl<Digit: AdditiveGroupDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: &Self) {
        (self.sign, self.digits) =
            sum_components::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
    }
}

impl<Digit: AdditiveGroupDigit, const SEPARATOR: char, const SHIFT: usize> AddAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            sum_components::<Digit, SHIFT>(self.sign, &self.digits, other.sign, &other.digits);
    }
}
