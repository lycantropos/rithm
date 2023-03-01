use traiter::numbers::CheckedDiv;

use super::digits::CheckedDivComponents;
use super::types::BigInt;

impl<
        Digit: CheckedDivComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDiv for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        Digit::checked_div_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedDivComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDiv<&Self> for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: &Self) -> Self::Output {
        Digit::checked_div_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedDivComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDiv<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>;

    fn checked_div(
        self,
        divisor: BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_div_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<
            Digit,
            SEPARATOR,
            DIGIT_BITNESS,
        > {
            sign,
            digits,
        })
    }
}

impl<
        Digit: CheckedDivComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDiv for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        Digit::checked_div_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<
            Digit,
            SEPARATOR,
            DIGIT_BITNESS,
        > {
            sign,
            digits,
        })
    }
}
