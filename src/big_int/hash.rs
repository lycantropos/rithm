use std::hash::{Hash, Hasher};

use super::types::BigInt;

impl<Digit: Hash, const SEPARATOR: char, const SHIFT: usize> Hash
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign.hash(state);
        self.digits.hash(state);
    }
}
