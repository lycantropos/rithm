use std::ops::BitAndAssign;

use super::digits::BitwiseAndComponents;
use super::types::BigInt;

impl<Digit: BitwiseAndComponents + Clone, const DIGIT_BITNESS: usize>
    BitAndAssign for BigInt<Digit, DIGIT_BITNESS>
{
    fn bitand_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            Digit::bitwise_and_components::<DIGIT_BITNESS>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits,
            );
    }
}

impl<Digit: BitwiseAndComponents + Clone, const DIGIT_BITNESS: usize>
    BitAndAssign<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    fn bitand_assign(&mut self, other: &Self) {
        (self.sign, self.digits) =
            Digit::bitwise_and_components::<DIGIT_BITNESS>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            );
    }
}
