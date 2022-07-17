use traiter::numbers::{Unitary, Zeroable};

use super::digits::{non_zero_value_to_sign, DigitsFromNonZeroValue};
use super::types::BigInt;

impl<Digit: Unitary + Zeroable, const SEPARATOR: char, const SHIFT: usize>
    From<bool> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn from(value: bool) -> Self {
        if value {
            Self::one()
        } else {
            Self::zero()
        }
    }
}

macro_rules! primitive_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<Digit: DigitsFromNonZeroValue<$t> + Zeroable, const SEPARATOR: char, const SHIFT: usize> From<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn from(value: $t) -> Self {
                if value.is_zero() {
                    Self::zero()
                } else {
                    Self {
                        sign: non_zero_value_to_sign(value),
                        digits: Digit::digits_from_non_zero_value::<SHIFT>(value),
                    }
                }
            }
        }
    )*)
}

primitive_partial_eq_to_big_int_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
