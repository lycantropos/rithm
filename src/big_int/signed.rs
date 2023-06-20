use traiter::numbers::{Sign, Signed};

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> Signed
    for &BigInt<Digit, DIGIT_BITNESS>
{
    fn is_negative(self) -> bool {
        self.sign.is_negative()
    }

    fn is_positive(self) -> bool {
        self.sign.is_positive()
    }

    fn sign(self) -> Sign {
        self.sign.sign()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Signed
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn is_negative(self) -> bool {
        self.sign.is_negative()
    }

    fn is_positive(self) -> bool {
        self.sign.is_positive()
    }

    fn sign(self) -> Sign {
        self.sign.sign()
    }
}
