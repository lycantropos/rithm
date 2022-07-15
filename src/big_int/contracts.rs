use core::mem::size_of;

use crate::contracts::is_signed;
use crate::traits::HasSignBit;

pub const fn is_valid_shift<Digit: HasSignBit, const SHIFT: usize>() -> bool {
    const BITS_IN_BYTE: usize = 8;
    SHIFT < BITS_IN_BYTE * size_of::<Digit>() - (is_signed::<Digit>() as usize)
}
