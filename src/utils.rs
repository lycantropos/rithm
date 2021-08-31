use std::convert::TryFrom;

use num::{PrimInt, Unsigned};

pub(crate) fn to_bit_length<T>(value: T) -> usize
where
    T: PrimInt,
    usize: TryFrom<T>,
{
    static BIT_LENGTHS_TABLE: [usize; 32] = [
        0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5,
    ];
    let mut result: usize = 0;
    let mut value = value;
    unsafe {
        while value >= T::from(32).unwrap_unchecked() {
            result += 6;
            value = value >> 6;
        }
        result += BIT_LENGTHS_TABLE[usize::try_from(value).unwrap_unchecked()];
    }
    result
}

pub(crate) const fn floor_log10(value: usize) -> usize {
    match value {
        0..=9 => 0,
        _ => floor_log10(value / 10) + 1,
    }
}

pub(crate) const fn power(base: usize, exponent: usize) -> usize {
    match exponent {
        0 => 1,
        _ => base * power(base, exponent - 1),
    }
}

pub(crate) fn floor_log2<T>(mut value: T) -> usize
where
    T: PrimInt,
    usize: TryFrom<T>,
{
    to_bit_length(value) - 1
}
