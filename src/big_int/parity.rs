use traiter::numbers::Parity;

use super::types::BigInt;

impl<Digit: Parity, const SEPARATOR: char, const SHIFT: usize> Parity
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_even(&self) -> bool {
        self.digits[0].is_even()
    }

    fn is_odd(&self) -> bool {
        self.digits[0].is_odd()
    }
}
