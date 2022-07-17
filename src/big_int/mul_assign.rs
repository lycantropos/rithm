use std::ops::MulAssign;

use super::digits::MultiplyDigits;
use super::types::BigInt;

impl<Digit: MultiplyDigits, const SEPARATOR: char, const SHIFT: usize>
    MulAssign for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn mul_assign(&mut self, other: Self) {
        self.sign *= other.sign;
        self.digits =
            Digit::multiply_digits::<SHIFT>(&self.digits, &other.digits);
    }
}

impl<Digit: MultiplyDigits, const SEPARATOR: char, const SHIFT: usize>
    MulAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn mul_assign(&mut self, other: &Self) {
        self.sign *= other.sign;
        self.digits =
            Digit::multiply_digits::<SHIFT>(&self.digits, &other.digits);
    }
}
