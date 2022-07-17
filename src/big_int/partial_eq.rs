use traiter::numbers::Zeroable;

use super::digits::{value_to_sign, DigitsFromNonZeroValue};
use super::types::BigInt;

macro_rules! primitive_partial_eq_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$t> + PartialEq + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialEq<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            fn eq(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) == other.sign
                    && (self.is_zero()
                        || Digit::digits_from_non_zero_value::<SHIFT>(*self) == other.digits)
            }
        }

        impl<
                Digit: DigitsFromNonZeroValue<$t> + PartialEq + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialEq<$t> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn eq(&self, other: &$t) -> bool {
                self.sign == value_to_sign(*other)
                    && (self.is_zero()
                        || self.digits == Digit::digits_from_non_zero_value::<SHIFT>(*other))
            }
        }
    )*)
}

primitive_partial_eq_to_big_int_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
