use traiter::numbers::Abs;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> Abs for BigInt<Digit, DIGIT_BITNESS> {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const DIGIT_BITNESS: usize> Abs
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits.clone(),
        }
    }
}
