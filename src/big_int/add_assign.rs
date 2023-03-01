use std::ops::AddAssign;

use super::digits::SumComponents;
use super::types::BigInt;

impl<Digit: SumComponents, const DIGIT_BITNESS: usize> AddAssign<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn add_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = Digit::sum_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
    }
}

impl<Digit: SumComponents, const DIGIT_BITNESS: usize> AddAssign
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) = Digit::sum_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
    }
}
