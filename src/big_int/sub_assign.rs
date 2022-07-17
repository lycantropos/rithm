use std::ops::SubAssign;

use super::digits::SubtractComponents;
use super::types::BigInt;

impl<Digit: SubtractComponents, const SEPARATOR: char, const SHIFT: usize>
    SubAssign for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.sign, self.digits) = Digit::subtract_components::<SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}

impl<Digit: SubtractComponents, const SEPARATOR: char, const SHIFT: usize>
    SubAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn sub_assign(&mut self, subtrahend: &Self) {
        (self.sign, self.digits) = Digit::subtract_components::<SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
    }
}
