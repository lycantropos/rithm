use crate::traits::Oppositive;

use super::digits::OppositiveDigit;
use super::types::BigInt;

impl<Digit: OppositiveDigit, const SEPARATOR: char, const SHIFT: usize> Oppositive
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_negative(&self) -> bool {
        self.sign.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.sign.is_positive()
    }
}
