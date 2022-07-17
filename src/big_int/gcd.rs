use traiter::numbers::Gcd;

use super::digits::GcdDigits;
use super::types::BigInt;

impl<Digit: GcdDigits, const SEPARATOR: char, const SHIFT: usize> Gcd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) =
            Digit::gcd_digits::<SHIFT>(self.digits, other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: Clone + GcdDigits, const SEPARATOR: char, const SHIFT: usize>
    Gcd<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn gcd(self, other: &Self) -> Self::Output {
        let (sign, digits) =
            Digit::gcd_digits::<SHIFT>(self.digits, other.digits.clone());
        Self::Output { sign, digits }
    }
}

impl<Digit: Clone + GcdDigits, const SEPARATOR: char, const SHIFT: usize>
    Gcd<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn gcd(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (sign, digits) =
            Digit::gcd_digits::<SHIFT>(self.digits.clone(), other.digits);
        Self::Output { sign, digits }
    }
}

impl<Digit: Clone + GcdDigits, const SEPARATOR: char, const SHIFT: usize> Gcd
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn gcd(self, other: Self) -> Self::Output {
        let (sign, digits) = Digit::gcd_digits::<SHIFT>(
            self.digits.clone(),
            other.digits.clone(),
        );
        Self::Output { sign, digits }
    }
}
