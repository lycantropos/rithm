use std::ops::{Add, Mul};

use traiter::numbers::BitLength;

use super::types::BigInt;

impl<Digit: BitLength<Output = usize> + Copy, const DIGIT_BITNESS: usize>
    BitLength for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + From<usize>
        + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn bit_length(self) -> Self::Output {
        if self.digits.len() <= usize::MAX / DIGIT_BITNESS {
            Self::Output::from(
                (self.digits.len() - 1) * DIGIT_BITNESS
                    + self.digits[self.digits.len() - 1].bit_length(),
            )
        } else {
            Self::Output::from(self.digits.len() - 1)
                * Self::Output::from(DIGIT_BITNESS)
                + Self::Output::from(
                    self.digits[self.digits.len() - 1].bit_length(),
                )
        }
    }
}
