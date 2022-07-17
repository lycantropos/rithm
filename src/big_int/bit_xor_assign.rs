use std::ops::BitXorAssign;

use super::digits::BitwiseXorComponents;
use super::types::BigInt;

impl<
        Digit: BitwiseXorComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitXorAssign for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = Digit::bitwise_xor_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}

impl<
        Digit: BitwiseXorComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitXorAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = Digit::bitwise_xor_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}
