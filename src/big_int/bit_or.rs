use std::ops::BitOr;

use super::digits::BitwiseOrComponents;
use super::types::BigInt;

impl<Digit: BitwiseOrComponents, const DIGIT_BITNESS: usize> BitOr
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits,
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseOrComponents + Clone, const DIGIT_BITNESS: usize>
    BitOr<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn bitor(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits,
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseOrComponents + Clone, const DIGIT_BITNESS: usize>
    BitOr<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bitor(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseOrComponents + Clone, const DIGIT_BITNESS: usize> BitOr
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
