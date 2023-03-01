use std::ops::Neg;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const DIGIT_BITNESS: usize> Neg
    for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            sign: -self.sign,
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const SEPARATOR: char, const DIGIT_BITNESS: usize> Neg
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn neg(self) -> Self::Output {
        Self::Output {
            sign: -self.sign,
            digits: self.digits.clone(),
        }
    }
}
