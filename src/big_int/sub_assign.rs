use std::ops::SubAssign;

use super::digits::SubtractComponents;
use super::types::BigInt;

impl<Digit: SubtractComponents, const DIGIT_BITNESS: usize> SubAssign
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.sign, self.digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}

impl<Digit: SubtractComponents, const DIGIT_BITNESS: usize> SubAssign<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn sub_assign(&mut self, subtrahend: &Self) {
        (self.sign, self.digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}
