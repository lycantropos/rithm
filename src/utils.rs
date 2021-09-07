use std::convert::TryFrom;

use num::PrimInt;

pub(crate) fn bit_length<T>(value: T) -> usize
where
    T: From<u8> + PrimInt,
    usize: TryFrom<T>,
{
    let mut result: usize = 0;
    let mut value = value;
    while value >= <T as From<u8>>::from(32u8) {
        result += 6;
        value = value >> 6;
    }
    const BIT_LENGTHS_TABLE: [usize; 32] = [
        0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5,
    ];
    result + BIT_LENGTHS_TABLE[unsafe { usize::try_from(value).unwrap_unchecked() }]
}

pub(crate) fn gcd<T>(mut first: T, mut second: T) -> T
where
    T: PrimInt,
{
    while !second.is_zero() {
        (first, second) = (second, first % second);
    }
    first
}

pub(crate) const fn floor_log(value: usize, base: usize) -> Result<usize, &'static str> {
    if value == 0usize {
        Err("Logarithm of zero is undefined.")
    } else if value < base {
        Ok(0)
    } else {
        match floor_log(value / base, base) {
            Ok(value) => Ok(value + 1),
            error => error,
        }
    }
}

pub(crate) const fn power(base: usize, exponent: usize) -> usize {
    match exponent {
        0 => 1,
        _ => base * power(base, exponent - 1),
    }
}

pub(crate) fn floor_log2<T>(value: T) -> Result<usize, &'static str>
where
    T: From<u8> + PrimInt,
    usize: TryFrom<T>,
{
    if value.is_zero() {
        Err("Logarithm of zero is undefined.")
    } else {
        Ok(bit_length(value) - 1)
    }
}
