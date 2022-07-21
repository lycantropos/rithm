use std::ops::BitAndAssign;

use super::digits::BitwiseAndComponents;
use super::types::BigInt;

impl<
        Digit: BitwiseAndComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitAndAssign for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: Self) {
        (self.sign, self.digits) = Digit::bitwise_and_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}

impl<
        Digit: BitwiseAndComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitAndAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitand_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = Digit::bitwise_and_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}
