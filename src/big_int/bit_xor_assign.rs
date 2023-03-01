use std::ops::BitXorAssign;

use super::digits::BitwiseXorComponents;
use super::types::BigInt;

impl<
        Digit: BitwiseXorComponents + Clone,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > BitXorAssign for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    fn bitxor_assign(&mut self, other: Self) {
        (self.sign, self.digits) =
            Digit::bitwise_xor_components::<DIGIT_BITNESS>(
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
        const DIGIT_BITNESS: usize,
    > BitXorAssign<&Self> for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    fn bitxor_assign(&mut self, other: &Self) {
        (self.sign, self.digits) =
            Digit::bitwise_xor_components::<DIGIT_BITNESS>(
                self.sign,
                self.digits.clone(),
                other.sign,
                other.digits.clone(),
            );
    }
}
