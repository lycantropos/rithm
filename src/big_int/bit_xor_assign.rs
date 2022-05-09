use std::ops::BitXorAssign;

use super::digits::{bitwise_xor_components, BitwiseExclusiveDisjunctiveDigit};
use super::types::BigInt;

impl<Digit: BitwiseExclusiveDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize>
    BitXorAssign for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: Self) {
        (self.sign, self.digits) = bitwise_xor_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
    }
}

impl<Digit: BitwiseExclusiveDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize>
    BitXorAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn bitxor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = bitwise_xor_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
    }
}
