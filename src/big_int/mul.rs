use std::ops::Mul;

use super::digits::MultiplyDigits;
use super::types::BigInt;

impl<Digit: MultiplyDigits, const DIGIT_BITNESS: usize> Mul
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: Digit::multiply_digits::<DIGIT_BITNESS>(
                &self.digits,
                &other.digits,
            ),
        }
    }
}

impl<Digit: MultiplyDigits, const DIGIT_BITNESS: usize> Mul<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: Digit::multiply_digits::<DIGIT_BITNESS>(
                &self.digits,
                &other.digits,
            ),
        }
    }
}

impl<Digit: MultiplyDigits, const DIGIT_BITNESS: usize>
    Mul<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn mul(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: Digit::multiply_digits::<DIGIT_BITNESS>(
                &self.digits,
                &other.digits,
            ),
        }
    }
}

impl<Digit: MultiplyDigits, const DIGIT_BITNESS: usize> Mul
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            sign: self.sign * other.sign,
            digits: Digit::multiply_digits::<DIGIT_BITNESS>(
                &self.digits,
                &other.digits,
            ),
        }
    }
}
