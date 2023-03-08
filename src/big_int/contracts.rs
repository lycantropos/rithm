use core::mem::size_of;

use crate::contracts::is_signed;
use crate::traits::HasSignBit;

#[must_use]
pub const fn is_valid_digit_bitness<
    Digit: HasSignBit,
    const DIGIT_BITNESS: usize,
>() -> bool {
    const BITS_IN_BYTE: usize = 8;
    0 < DIGIT_BITNESS
        && DIGIT_BITNESS
            < BITS_IN_BYTE * size_of::<Digit>()
                - (is_signed::<Digit>() as usize)
}
