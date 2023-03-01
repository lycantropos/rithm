use traiter::numbers::{CheckedPow, Pow};

use super::types::BigInt;

const NEGATIVE_EXPONENT_MESSAGE: &str = "Exponent should be non-negative.";

impl<Digit, const DIGIT_BITNESS: usize> Pow<Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPow<Self, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: Self) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Pow<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedPow<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: &Self) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Pow<BigInt<Digit, DIGIT_BITNESS>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPow<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn pow(self, exponent: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Pow<Self>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedPow<Self, Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn pow(self, exponent: Self) -> Self::Output {
        self.checked_pow(exponent).expect(NEGATIVE_EXPONENT_MESSAGE)
    }
}
