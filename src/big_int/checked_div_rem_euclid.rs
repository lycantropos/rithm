use crate::traits::CheckedDivRemEuclid;

use super::digits::{checked_div_rem_euclid, EuclidDivisibleDigit};
use super::types::BigInt;

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: &Self) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    Self {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    Self {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem_euclid(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: EuclidDivisibleDigit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        checked_div_rem_euclid::<Digit, SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(quotient_sign, quotient_digits, remainder_sign, remainder_digits)| {
                (
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, SEPARATOR, SHIFT> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}
