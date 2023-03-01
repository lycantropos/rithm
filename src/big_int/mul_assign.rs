use std::ops::MulAssign;

use super::digits::MultiplyDigits;
use super::types::BigInt;

impl<Digit: MultiplyDigits, const DIGIT_BITNESS: usize> MulAssign
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn mul_assign(&mut self, other: Self) {
        self.sign *= other.sign;
        self.digits = Digit::multiply_digits::<DIGIT_BITNESS>(
            &self.digits,
            &other.digits,
        );
    }
}

impl<Digit: MultiplyDigits, const DIGIT_BITNESS: usize> MulAssign<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn mul_assign(&mut self, other: &Self) {
        self.sign *= other.sign;
        self.digits = Digit::multiply_digits::<DIGIT_BITNESS>(
            &self.digits,
            &other.digits,
        );
    }
}
