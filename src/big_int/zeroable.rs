use traiter::numbers::Zeroable;

use super::types::BigInt;

impl<Digit, const DIGIT_BITNESS: usize> Zeroable
    for &BigInt<Digit, DIGIT_BITNESS>
{
    fn is_zero(self) -> bool {
        self.sign.is_zero()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Zeroable
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn is_zero(self) -> bool {
        self.sign.is_zero()
    }
}
