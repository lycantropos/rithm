use traiter::numbers::{Unitary, Zeroable};

use super::digits::{non_zero_value_to_sign, DigitsFromNonZeroValue};
use super::types::BigInt;

impl<Digit: Unitary + Zeroable, const DIGIT_BITNESS: usize> From<bool>
    for BigInt<Digit, DIGIT_BITNESS>
{
    fn from(value: bool) -> Self {
        if value {
            Self::one()
        } else {
            Self::zero()
        }
    }
}

macro_rules! from_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Zeroable,
                const DIGIT_BITNESS: usize,
            > From<$integer> for BigInt<Digit, DIGIT_BITNESS>
        {
            fn from(value: $integer) -> Self {
                if value.is_zero() {
                    Self::zero()
                } else {
                    Self {
                        sign: non_zero_value_to_sign(value),
                        digits: Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(
                            value,
                        ),
                    }
                }
            }
        }
    )*)
}

from_integer_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
