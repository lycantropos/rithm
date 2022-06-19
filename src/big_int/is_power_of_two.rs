use crate::traits::{IsPowerOfTwo, Zeroable};

use super::types::BigInt;

impl<Digit: IsPowerOfTwo + Zeroable, const SEPARATOR: char, const SHIFT: usize> IsPowerOfTwo
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_power_of_two(&self) -> bool {
        self.sign.is_positive()
            && self.digits[..self.digits.len() - 1]
                .iter()
                .all(|digit| digit.is_zero())
            && self.digits[self.digits.len() - 1].is_power_of_two()
    }
}
