use std::ops::BitAnd;

use super::digits::BitwiseAndComponents;
use super::types::BigInt;

impl<Digit: BitwiseAndComponents, const DIGIT_BITNESS: usize> BitAnd
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_and_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits,
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseAndComponents + Clone, const DIGIT_BITNESS: usize>
    BitAnd<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn bitand(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_and_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits,
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseAndComponents + Clone, const DIGIT_BITNESS: usize>
    BitAnd<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bitand(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (sign, digits) = Digit::bitwise_and_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseAndComponents + Clone, const DIGIT_BITNESS: usize> BitAnd
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bitand(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_and_components::<DIGIT_BITNESS>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
