use std::ops::Sub;

use super::digits::SubtractComponents;
use super::types::BigInt;

impl<Digit: SubtractComponents, const DIGIT_BITNESS: usize> Sub
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SubtractComponents, const DIGIT_BITNESS: usize> Sub<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn sub(self, subtrahend: &Self) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SubtractComponents, const DIGIT_BITNESS: usize>
    Sub<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn sub(self, subtrahend: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SubtractComponents, const DIGIT_BITNESS: usize> Sub
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}
