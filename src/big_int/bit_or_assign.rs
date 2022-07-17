use std::ops::BitOrAssign;

use super::digits::BitwiseOrComponents;
use super::types::BigInt;

impl<
        Digit: BitwiseOrComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitOrAssign for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = Digit::bitwise_or_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}

impl<
        Digit: BitwiseOrComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitOrAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = Digit::bitwise_or_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}
