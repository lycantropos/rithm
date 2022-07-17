use traiter::numbers::CheckedDivRem;

use super::digits::CheckedDivRemComponents;
use super::types::BigInt;

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivRem for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_div_rem_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
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

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivRem<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem(self, divisor: &Self) -> Self::Output {
        Digit::checked_div_rem_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
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

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivRem<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        Digit::checked_div_rem_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
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

impl<
        Digit: CheckedDivRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivRem for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        BigInt<Digit, SEPARATOR, SHIFT>,
    )>;

    fn checked_div_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_div_rem_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(
            |(
                quotient_sign,
                quotient_digits,
                remainder_sign,
                remainder_digits,
            )| {
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
