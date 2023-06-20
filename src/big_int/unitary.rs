use traiter::numbers::Unitary;

use super::types::BigInt;

impl<'a, Digit, const DIGIT_BITNESS: usize> Unitary
    for &'a BigInt<Digit, DIGIT_BITNESS>
where
    &'a Digit: Unitary,
{
    fn is_one(self) -> bool {
        self.sign.is_positive()
            && self.digits.len() == 1
            && self.digits[0].is_one()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Unitary
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a Digit: Unitary,
{
    fn is_one(self) -> bool {
        self.sign.is_positive()
            && self.digits.len() == 1
            && self.digits[0].is_one()
    }
}
