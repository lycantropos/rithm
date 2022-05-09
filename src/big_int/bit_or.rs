use std::ops::BitOr;

use super::digits::{bitwise_or_components, BitwiseDisjunctiveDigit};
use super::types::BigInt;

impl<Digit: BitwiseDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitOr
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) =
            bitwise_or_components::<Digit, SHIFT>(self.sign, self.digits, other.sign, other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitOr<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitor(self, other: &Self) -> Self::Output {
        let (sign, digits) = bitwise_or_components::<Digit, SHIFT>(
            self.sign,
            self.digits,
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize>
    BitOr<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitor(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = bitwise_or_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: BitwiseDisjunctiveDigit, const SEPARATOR: char, const SHIFT: usize> BitOr
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitor(self, other: Self) -> Self::Output {
        let (sign, digits) = bitwise_or_components::<Digit, SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
