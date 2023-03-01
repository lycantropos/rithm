use std::ops::BitXor;

use super::digits::BitwiseXorComponents;
use super::types::BigInt;

impl<Digit: BitwiseXorComponents, const DIGIT_BITNESS: usize> BitXor
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits,
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseXorComponents + Clone, const DIGIT_BITNESS: usize>
    BitXor<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn bitxor(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits,
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseXorComponents + Clone, const DIGIT_BITNESS: usize>
    BitXor<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bitxor(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseXorComponents + Clone, const DIGIT_BITNESS: usize> BitXor
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
