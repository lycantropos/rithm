use std::ops::Add;

use super::digits::SumComponents;
use super::types::BigInt;

impl<Digit: SumComponents, const DIGIT_BITNESS: usize> Add
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::sum_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SumComponents, const DIGIT_BITNESS: usize>
    Add<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn add(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (sign, digits) = Digit::sum_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SumComponents, const DIGIT_BITNESS: usize> Add
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn add(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::sum_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SumComponents, const DIGIT_BITNESS: usize> Add<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::sum_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
        Self::Output { sign, digits }
    }
}
