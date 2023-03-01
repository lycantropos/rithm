use traiter::numbers::Parity;

use super::types::BigInt;

impl<Digit: Parity, const SEPARATOR: char, const DIGIT_BITNESS: usize> Parity
    for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    fn is_even(&self) -> bool {
        self.digits[0].is_even()
    }

    fn is_odd(&self) -> bool {
        self.digits[0].is_odd()
    }
}
