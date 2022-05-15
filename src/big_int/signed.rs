use crate::traits::{Signed, Zeroable};

use super::types::BigInt;

impl<Digit: Zeroable, const SEPARATOR: char, const SHIFT: usize> Signed
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn is_negative(&self) -> bool {
        self.sign.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.sign.is_positive()
    }
}
