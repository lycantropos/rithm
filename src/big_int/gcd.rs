use traiter::numbers::Gcd;

use super::digits::GcdDigits;
use super::types::BigInt;

impl<Digit: GcdDigits, const SEPARATOR: char, const DIGIT_BITNESS: usize> Gcd
    for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = Self;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) =
            Digit::gcd_digits::<DIGIT_BITNESS>(self.digits, other.digits);
        Self::Output { sign, digits }
    }
}

impl<
        Digit: Clone + GcdDigits,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Gcd<&Self> for BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
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

impl<
        Digit: Clone + GcdDigits,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Gcd<BigInt<Digit, SEPARATOR, DIGIT_BITNESS>>
    for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn gcd(
        self,
        other: BigInt<Digit, SEPARATOR, DIGIT_BITNESS>,
    ) -> Self::Output {
        let (sign, digits) = Digit::gcd_digits::<DIGIT_BITNESS>(
            self.digits.clone(),
            other.digits,
        );
        Self::Output { sign, digits }
    }
}

impl<
        Digit: Clone + GcdDigits,
        const SEPARATOR: char,
        const DIGIT_BITNESS: usize,
    > Gcd for &BigInt<Digit, SEPARATOR, DIGIT_BITNESS>
{
    type Output = BigInt<Digit, SEPARATOR, DIGIT_BITNESS>;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::gcd_digits::<DIGIT_BITNESS>(
            self.digits.clone(),
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
