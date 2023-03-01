use std::ops::RemAssign;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::digits::CheckedRemComponents;
use super::types::BigInt;

impl<Digit: CheckedRemComponents, const DIGIT_BITNESS: usize> RemAssign
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn rem_assign(&mut self, divisor: Self) {
        (self.sign, self.digits) =
            Digit::checked_rem_components::<DIGIT_BITNESS>(
                self.sign,
                &self.digits,
                divisor.sign,
                &divisor.digits,
            )
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
    }
}

impl<Digit: CheckedRemComponents, const DIGIT_BITNESS: usize> RemAssign<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn rem_assign(&mut self, divisor: &Self) {
        (self.sign, self.digits) =
            Digit::checked_rem_components::<DIGIT_BITNESS>(
                self.sign,
                &self.digits,
                divisor.sign,
                &divisor.digits,
            )
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
    }
}
