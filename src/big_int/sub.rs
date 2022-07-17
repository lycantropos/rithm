use std::ops::Sub;

use super::digits::SubtractComponents;
use super::types::BigInt;

impl<Digit: SubtractComponents, const SEPARATOR: char, const SHIFT: usize> Sub
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SubtractComponents, const SEPARATOR: char, const SHIFT: usize>
    Sub<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn sub(self, subtrahend: &Self) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SubtractComponents, const SEPARATOR: char, const SHIFT: usize>
    Sub<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn sub(self, subtrahend: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: SubtractComponents, const SEPARATOR: char, const SHIFT: usize> Sub
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (sign, digits) = Digit::subtract_components::<SHIFT>(
            self.sign,
            &self.digits,
            subtrahend.sign,
            &subtrahend.digits,
        );
        Self::Output { sign, digits }
    }
}
