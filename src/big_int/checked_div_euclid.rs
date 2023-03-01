use traiter::numbers::CheckedDivEuclid;

use super::digits::CheckedDivEuclidComponents;
use super::types::BigInt;

impl<
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivEuclid for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivEuclid<&Self> for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: &Self) -> Self::Output {
        Digit::checked_div_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivEuclid<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_div_euclid_components::<DIGIT_BITNESS>(
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
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > CheckedDivEuclid for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_euclid_components::<DIGIT_BITNESS>(
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
