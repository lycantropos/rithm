use std::cmp::Ordering;

use traiter::numbers::{Sign, Signed};

use super::digits::compare_digits;
use super::types::BigInt;

impl<Digit: Ord, const SEPARATOR: char, const SHIFT: usize> Ord
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Signed,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.sign.cmp(&other.sign) {
            Ordering::Equal => match self.sign() {
                Sign::Negative => compare_digits(&other.digits, &self.digits),
                Sign::Positive => compare_digits(&self.digits, &other.digits),
                Sign::Zero => Ordering::Equal,
            },
            value => value,
        }
    }
}
