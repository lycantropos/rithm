use std::cmp::Ordering;

use super::digits::compare_digits;
use super::types::BigInt;

impl<Digit: Ord, const SEPARATOR: char, const SHIFT: usize> Ord
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.sign.cmp(&other.sign) {
            Ordering::Equal => compare_digits(&self.digits, &other.digits),
            value => value,
        }
    }
}
