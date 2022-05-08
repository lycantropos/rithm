use std::ops::Neg;

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Neg for BigInt<Digit, SEPARATOR, SHIFT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            sign: -self.sign,
            digits: self.digits,
        }
    }
}

impl<Digit: Clone, const SEPARATOR: char, const SHIFT: usize> Neg
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn neg(self) -> Self::Output {
        Self::Output {
            sign: -self.sign,
            digits: self.digits.clone(),
        }
    }
}
