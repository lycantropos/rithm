use traiter::numbers::Abs;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const DIGIT_BITNESS: usize> Abs
    for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const SEPARATOR: char, const DIGIT_BITNESS: usize> Abs
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits.clone(),
        }
    }
}
