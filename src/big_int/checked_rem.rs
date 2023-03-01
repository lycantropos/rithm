use traiter::numbers::CheckedRem;

use super::digits::CheckedRemComponents;
use super::types::BigInt;

impl<Digit: CheckedRemComponents, const DIGIT_BITNESS: usize> CheckedRem
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: CheckedRemComponents, const DIGIT_BITNESS: usize> CheckedRem<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: &Self) -> Self::Output {
        Digit::checked_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<Digit: CheckedRemComponents, const DIGIT_BITNESS: usize>
    CheckedRem<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_rem(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, DIGIT_BITNESS> { sign, digits })
    }
}

impl<Digit: CheckedRemComponents, const DIGIT_BITNESS: usize> CheckedRem
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_components::<DIGIT_BITNESS>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, DIGIT_BITNESS> { sign, digits })
    }
}
