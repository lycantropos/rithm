use num::{PrimInt, Unsigned};

pub(crate) fn to_bit_length(mut value: u32) -> usize {
    static BIT_LENGTHS_TABLE: [usize; 32] = [
        0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5,
    ];
    let mut result: usize = 0;
    while value >= 32 {
        result += 6;
        value >>= 6;
    }
    result += BIT_LENGTHS_TABLE[value as usize];
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

pub(crate) fn floor_log2<T>(mut value: T) -> T
where
    T: PrimInt + Unsigned,
{
    let mut result: T = T::zero();
    while !value.is_zero() {
        result = result + T::one();
        value = value >> 1;
    }
    result - T::one()
}
