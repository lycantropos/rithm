use super::digits::{non_zero_value_to_digits, value_to_sign, ConstructibleFrom};
use super::types::BigInt;

macro_rules! primitive_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<
                Digit: ConstructibleFrom<$t> + PartialEq,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialEq<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            fn eq(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) == other.sign
                    && (self.is_zero()
                        || non_zero_value_to_digits::<$t, Digit, SHIFT>(*self) == other.digits)
            }
        }

        impl<
                Digit: ConstructibleFrom<$t> + PartialEq,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialEq<$t> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn eq(&self, other: &$t) -> bool {
                self.sign == value_to_sign(*other)
                    && (self.is_zero()
                        || self.digits == non_zero_value_to_digits::<$t, Digit, SHIFT>(*other))
            }
        }
    )*)
}

primitive_partial_eq_to_big_int_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
