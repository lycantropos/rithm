use traiter::numbers::{IsPowerOfTwo, Zeroable};

use super::types::BigInt;

impl<'a, Digit, const DIGIT_BITNESS: usize> IsPowerOfTwo
    for &'a BigInt<Digit, DIGIT_BITNESS>
where
    &'a Digit: IsPowerOfTwo + Zeroable,
{
    fn is_power_of_two(self) -> bool {
        self.sign.is_positive()
            && self.digits[..self.digits.len() - 1]
                .iter()
                .all(Zeroable::is_zero)
            && self.digits[self.digits.len() - 1].is_power_of_two()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> IsPowerOfTwo
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a Digit: IsPowerOfTwo + Zeroable,
{
    fn is_power_of_two(self) -> bool {
        self.sign.is_positive()
            && self.digits[..self.digits.len() - 1]
                .iter()
                .all(Zeroable::is_zero)
            && self.digits[self.digits.len() - 1].is_power_of_two()
    }
}
