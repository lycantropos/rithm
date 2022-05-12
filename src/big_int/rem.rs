use std::ops::Rem;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::digits::{checked_rem, DivisibleDigit};
use super::types::BigInt;

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Rem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Rem
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem(self, divisor: Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> Rem<&Self>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem(self, divisor: &Self) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    Rem<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) =
            checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE);
        Self::Output { sign, digits }
    }
}
