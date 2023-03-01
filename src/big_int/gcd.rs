use traiter::numbers::Gcd;

use super::digits::GcdDigits;
use super::types::BigInt;

impl<Digit: GcdDigits, const DIGIT_BITNESS: usize> Gcd
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) =
            Digit::gcd_digits::<DIGIT_BITNESS>(self.digits, other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: Clone + GcdDigits, const DIGIT_BITNESS: usize> Gcd<&Self>
    for BigInt<Digit, DIGIT_BITNESS>
{
    type Output = Self;

    fn gcd(self, other: &Self) -> Self::Output {
        let (sign, digits) = Digit::gcd_digits::<DIGIT_BITNESS>(
            self.digits,
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: Clone + GcdDigits, const DIGIT_BITNESS: usize>
    Gcd<BigInt<Digit, DIGIT_BITNESS>> for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn gcd(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (sign, digits) = Digit::gcd_digits::<DIGIT_BITNESS>(
            self.digits.clone(),
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<Digit: Clone + GcdDigits, const DIGIT_BITNESS: usize> Gcd
    for &BigInt<Digit, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::gcd_digits::<DIGIT_BITNESS>(
            self.digits.clone(),
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
