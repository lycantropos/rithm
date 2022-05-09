use crate::traits::CheckedRem;

use super::digits::{checked_rem, DivisibleDigit};
use super::types::BigInt;

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem<&Self>
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: &Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}

impl<Digit: DivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        checked_rem::<Digit, SHIFT>(self.sign, &self.digits, divisor.sign, &divisor.digits)
            .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> { sign, digits })
    }
}
