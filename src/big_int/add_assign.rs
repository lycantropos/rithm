use std::ops::AddAssign;

use super::digits::SumComponents;
use super::types::BigInt;

impl<Digit: SumComponents, const SEPARATOR: char, const SHIFT: usize>
    AddAssign<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: &Self) {
        (self.sign, self.digits) = Digit::sum_components::<SHIFT>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
    }
}

impl<Digit: SumComponents, const SEPARATOR: char, const SHIFT: usize> AddAssign
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn add_assign(&mut self, other: Self) {
        (self.sign, self.digits) = Digit::sum_components::<SHIFT>(
            self.sign,
            &self.digits,
            other.sign,
            &other.digits,
        );
    }
}
