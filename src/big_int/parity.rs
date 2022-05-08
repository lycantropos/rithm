use crate::traits::Parity;

use super::digits::ParitiableDigit;
use super::types::BigInt;

impl<Digit: ParitiableDigit, const SEPARATOR: char, const SHIFT: usize> Parity
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_even(&self) -> bool {
        (self.digits[0] & Digit::one()).is_zero()
    }

    fn is_odd(&self) -> bool {
        !(self.digits[0] & Digit::one()).is_zero()
    }
}
