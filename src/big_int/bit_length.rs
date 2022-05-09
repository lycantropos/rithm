use crate::traits::BitLength;

use super::digits::{ConstructibleFrom, MultiplicativeDigit};
use super::types::BigInt;

impl<
        Digit: BitLength<Output = usize> + ConstructibleFrom<usize> + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitLength for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Self;

    fn bit_length(self) -> Self::Output {
        if self.digits.len() <= usize::MAX / SHIFT {
            Self::from(
                (self.digits.len() - 1) * SHIFT + self.digits[self.digits.len() - 1].bit_length(),
            )
        } else {
            Self::from(self.digits.len() - 1) * Self::from(SHIFT)
                + Self::from(self.digits[self.digits.len() - 1].bit_length())
        }
    }
}
