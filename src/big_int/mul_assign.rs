use std::ops::MulAssign;

use super::digits::{multiply_digits, MultiplicativeDigit};
use super::types::BigInt;

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> MulAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn mul_assign(&mut self, other: Self) {
        self.sign *= other.sign;
        self.digits = multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits);
    }
}

impl<Digit: MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize> MulAssign<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn mul_assign(&mut self, other: &Self) {
        self.sign *= other.sign;
        self.digits = multiply_digits::<Digit, SHIFT>(&self.digits, &other.digits);
    }
}
