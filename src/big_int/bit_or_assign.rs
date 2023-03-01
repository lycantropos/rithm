use std::ops::BitOrAssign;

use super::digits::BitwiseOrComponents;
use super::types::BigInt;

impl<Digit: BitwiseOrComponents + Clone, const DIGIT_BITNESS: usize>
    BitOrAssign for BigInt<Digit, DIGIT_BITNESS>
{
    fn bitor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = Digit::bitwise_or_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}

impl<Digit: BitwiseOrComponents + Clone, const DIGIT_BITNESS: usize>
    BitOrAssign<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    fn bitor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = Digit::bitwise_or_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}
