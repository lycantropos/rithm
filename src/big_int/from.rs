use super::digits::{non_zero_value_to_digits, non_zero_value_to_sign, ConstructibleFrom};
use super::types::BigInt;

macro_rules! primitive_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<Digit: ConstructibleFrom<$t>, const SEPARATOR: char, const SHIFT: usize> From<$t>
            for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn from(value: $t) -> Self {
                if value.is_zero() {
                    Self::zero()
                } else {
                    Self {
                        sign: non_zero_value_to_sign(value),
                        digits: non_zero_value_to_digits::<$t, Digit, SHIFT>(value),
                    }
                }
            }
        }
    )*)
}

primitive_partial_eq_to_big_int_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
