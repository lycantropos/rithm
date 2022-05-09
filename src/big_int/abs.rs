use crate::traits::Abs;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Abs for BigInt<Digit, SEPARATOR, SHIFT> {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const SEPARATOR: char, const SHIFT: usize> Abs
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn abs(self) -> Self::Output {
        Self::Output {
            sign: self.sign.abs(),
            digits: self.digits.clone(),
        }
    }
}
