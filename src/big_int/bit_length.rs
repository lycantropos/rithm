use traiter::numbers::BitLength;

use super::digits::{ConstructibleFrom, MultiplicativeDigit};
use super::types::BigInt;

impl<
        Digit: BitLength<Output = usize>
            + ConstructibleFrom<usize>
            + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > BitLength for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn bit_length(self) -> Self::Output {
        if self.digits.len() <= usize::MAX / SHIFT {
            Self::Output::from(
                (self.digits.len() - 1) * SHIFT
                    + self.digits[self.digits.len() - 1].bit_length(),
            )
        } else {
            Self::Output::from(self.digits.len() - 1)
                * Self::Output::from(SHIFT)
                + Self::Output::from(
                    self.digits[self.digits.len() - 1].bit_length(),
                )
        }
    }
}
