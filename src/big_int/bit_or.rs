use std::ops::BitOr;

use super::digits::BitwiseOrComponents;
use super::types::BigInt;

impl<
        Digit: BitwiseOrComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitOr for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<SHIFT>(
            self.sign,
            self.digits,
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: BitwiseOrComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitOr<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitor(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<SHIFT>(
            self.sign,
            self.digits,
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: BitwiseOrComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitOr<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitor(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: BitwiseOrComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitOr for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_or_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
