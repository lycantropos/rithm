use traiter::numbers::Zeroable;

use super::digits::{value_to_sign, DigitsFromNonZeroValue};
use super::types::BigInt;

macro_rules! integer_partial_eq_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + PartialEq + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialEq<BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        {
            fn eq(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) == other.sign
                    && (self.is_zero()
                        || Digit::digits_from_non_zero_value::<SHIFT>(*self)
                            == other.digits)
            }
        }

        impl<
                Digit: DigitsFromNonZeroValue<$integer> + PartialEq + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialEq<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn eq(&self, other: &$integer) -> bool {
                self.sign == value_to_sign(*other)
                    && (self.is_zero()
                        || self.digits
                            == Digit::digits_from_non_zero_value::<SHIFT>(
                                *other,
                            ))
            }
        }
    )*)
}

integer_partial_eq_big_int_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
