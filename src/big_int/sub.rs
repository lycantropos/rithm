use std::ops::Sub;

use super::digits::SubtractComponents;
use super::types::BigInt;

impl<
        Digit: SubtractComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Sub for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
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

impl<
        Digit: SubtractComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Sub<&Self> for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
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

impl<
        Digit: SubtractComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Sub<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn sub(
        self,
        subtrahend: BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    ) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: SubtractComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Sub for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

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
