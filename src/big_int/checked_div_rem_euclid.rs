use traiter::numbers::CheckedDivRemEuclid;

use super::digits::CheckedDivRemEuclidComponents;
use super::types::BigInt;

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_rem_euclid_components::<DIGIT_BITNESS>(
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

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<&Self> for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Option<(Self, Self)>;

    fn checked_div_rem_euclid(self, divisor: &Self) -> Self::Output {
        Digit::checked_div_rem_euclid_components::<DIGIT_BITNESS>(
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

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output =
        Option<(BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>)>;

    fn checked_div_rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        Digit::checked_div_rem_euclid_components::<DIGIT_BITNESS>(
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
                    BigInt::<Digit, DIGIT_BITNESS> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, DIGIT_BITNESS> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}

impl<Digit: CheckedDivRemEuclidComponents, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output =
        Option<(BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_rem_euclid_components::<DIGIT_BITNESS>(
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
                    BigInt::<Digit, DIGIT_BITNESS> {
                        sign: quotient_sign,
                        digits: quotient_digits,
                    },
                    BigInt::<Digit, DIGIT_BITNESS> {
                        sign: remainder_sign,
                        digits: remainder_digits,
                    },
                )
            },
        )
    }
}
