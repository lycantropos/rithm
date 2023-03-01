use traiter::numbers::CheckedDivEuclid;

use super::digits::CheckedDivEuclidComponents;
use super::types::BigInt;

impl<Digit: CheckedDivEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivEuclid for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit: CheckedDivEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<&Self> for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit: CheckedDivEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_div_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, DIGIT_BITNESS> { sign, digits })
    }
}

impl<Digit: CheckedDivEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivEuclid for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, DIGIT_BITNESS> { sign, digits })
    }
}
