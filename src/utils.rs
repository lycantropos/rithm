use std::convert::TryFrom;

use crate::traits::{AssigningShiftingRightMonoid, Float, ModularPartialMagma, Zeroable};

pub(crate) const fn are_same<T, U>() -> bool {
    trait SameTo<U> {
        const VALUE: bool;
    }

    impl<T, U> SameTo<U> for T {
        default const VALUE: bool = false;
    }

    impl<T> SameTo<T> for T {
        const VALUE: bool = true;
    }

    <T as SameTo<U>>::VALUE
}

pub(crate) fn bit_length<T>(value: T) -> usize
where
    T: From<u8> + AssigningShiftingRightMonoid<usize> + PartialOrd,
    usize: TryFrom<T>,
{
    let mut result: usize = 0;
    let mut value = value;
    while value >= T::from(32u8) {
        result += 6;
        value >>= 6;
    }
    const BIT_LENGTHS_TABLE: [usize; 32] = [
        0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5,
    ];
    result + BIT_LENGTHS_TABLE[unsafe { usize::try_from(value).unwrap_unchecked() }]
}

pub(crate) fn gcd<T>(mut first: T, mut second: T) -> T
where
    T: Copy + ModularPartialMagma + Zeroable,
{
    while !second.is_zero() {
        (first, second) = (second, first.rem_euclid(second));
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

pub(crate) fn floor_log2<T>(value: T) -> usize
where
    T: From<u8> + AssigningShiftingRightMonoid<usize> + PartialOrd,
    usize: TryFrom<T>,
{
    debug_assert!(!value.is_zero());
    bit_length(value) - 1
}

pub(crate) fn load_exponent<Fraction: Float>(fraction: Fraction, exponent: i32) -> Fraction {
    let is_max_exponent = (exponent == Fraction::MAX_EXP) as i32;
    fraction
        * Fraction::from((1 + is_max_exponent) as f32)
        * Fraction::from(2.0f32).pow(exponent - is_max_exponent)
}

pub(crate) const fn power(base: usize, exponent: usize) -> usize {
    match exponent {
        0 => 1,
        _ => base * power(base, exponent - 1),
    }
}
