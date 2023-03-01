use std::ops::DivAssign;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::digits::CheckedDivComponents;
use super::types::BigInt;

impl<Digit: CheckedDivComponents, const DIGIT_BITNESS: usize> DivAssign
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn div_assign(&mut self, divisor: Self) {
        (self.sign, self.digits) =
            Digit::checked_div_components::<DIGIT_BITNESS>(
                self.sign,
                &self.digits,
                divisor.sign,
                &divisor.digits,
            )
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
    }
}

impl<Digit: CheckedDivComponents, const DIGIT_BITNESS: usize> DivAssign<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn div_assign(&mut self, divisor: &Self) {
        (self.sign, self.digits) =
            Digit::checked_div_components::<DIGIT_BITNESS>(
                self.sign,
                &self.digits,
                divisor.sign,
                &divisor.digits,
            )
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
    }
}
