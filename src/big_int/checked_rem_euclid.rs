use traiter::numbers::CheckedRemEuclid;

use super::digits::CheckedRemEuclidComponents;
use super::types::BigInt;

impl<Digit: CheckedRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedRemEuclid for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: CheckedRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedRemEuclid<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: &Self) -> Self::Output {
        Digit::checked_rem_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: CheckedRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedRemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_rem_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, DIGIT_BITNESS> { sign, digits })
    }
}

impl<Digit: CheckedRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedRemEuclid for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_euclid_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, DIGIT_BITNESS> { sign, digits })
    }
}
