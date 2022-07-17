use std::ops::BitXor;

use super::digits::BitwiseXorComponents;
use super::types::BigInt;

impl<
        Digit: BitwiseXorComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitXor for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<SHIFT>(
            self.sign,
            self.digits,
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: BitwiseXorComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitXor<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bitxor(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<SHIFT>(
            self.sign,
            self.digits,
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: BitwiseXorComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitXor<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitxor(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: BitwiseXorComponents + Clone,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitXor for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bitxor(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::bitwise_xor_components::<SHIFT>(
            self.sign,
            self.digits.clone(),
            other.sign,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
