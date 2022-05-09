use core::mem::size_of;

use crate::contracts::is_signed;
use crate::traits::Oppose;

pub const fn is_valid_shift<Digit: Oppose, const SHIFT: usize>() -> bool {
    const BITS_IN_BYTE: usize = 8;
    SHIFT < BITS_IN_BYTE * size_of::<Digit>() - (is_signed::<Digit>() as usize)
}
