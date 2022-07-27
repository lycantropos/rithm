use std::cmp::Ordering;

use traiter::numbers::{Sign, Signed, Zeroable};

use super::digits::{compare_digits, DigitsFromNonZeroValue};
use super::types::{BigInt, Sign as BigIntSign};

impl<Digit: Ord, const SEPARATOR: char, const SHIFT: usize> PartialOrd
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.sign.cmp(&other.sign) {
            Ordering::Equal => {
                if self.sign.is_zero() {
                    Ordering::Equal
                } else {
                    compare_digits(&self.digits, &other.digits)
                }
            }
            value => value,
        })
    }
}

macro_rules! big_int_partial_ord_to_signed_primitive_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Self: PartialEq<$integer>,
        {
            fn partial_cmp(&self, other: &$integer) -> Option<Ordering> {
                Some(match self.sign.cmp(&(other.signum() as BigIntSign)) {
                    Ordering::Equal => {
                        if self.sign.is_zero() {
                            Ordering::Equal
                        } else {
                            compare_digits(
                                &self.digits,
                                &Digit::digits_from_non_zero_value::<SHIFT>(
                                    *other,
                                ),
                            )
                        }
                    }
                    value => value,
                })
            }
        }
    )*)
}

big_int_partial_ord_to_signed_primitive_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! big_int_partial_ord_to_unsigned_primitive_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<$integer> for BigInt<Digit, SEPARATOR, SHIFT>
        where
            Self: PartialEq<$integer> + Signed,
        {
            fn partial_cmp(&self, other: &$integer) -> Option<Ordering> {
                Some(match self.sign() {
                    Sign::Negative => Ordering::Less,
                    Sign::Positive => {
                        if other.is_zero() {
                            Ordering::Greater
                        } else {
                            compare_digits(
                                &self.digits,
                                &Digit::digits_from_non_zero_value::<SHIFT>(
                                    *other,
                                ),
                            )
                        }
                    }
                    Sign::Zero => {
                        if other.is_zero() {
                            Ordering::Equal
                        } else {
                            Ordering::Less
                        }
                    }
                })
            }
        }
    )*)
}

big_int_partial_ord_to_unsigned_primitive_impl!(u8 u16 u32 u64 u128 usize);

macro_rules! signed_primitive_partial_ord_to_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        where
            Self: PartialEq<BigInt<Digit, SEPARATOR, SHIFT>>,
        {
            fn partial_cmp(
                &self,
                other: &BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Option<Ordering> {
                Some(match (self.signum() as BigIntSign).cmp(&other.sign) {
                    Ordering::Equal => {
                        if self.is_zero() {
                            Ordering::Equal
                        } else {
                            compare_digits(
                                &Digit::digits_from_non_zero_value::<SHIFT>(
                                    *self,
                                ),
                                &other.digits,
                            )
                        }
                    }
                    value => value,
                })
            }
        }
    )*)
}

signed_primitive_partial_ord_to_big_int_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_primitive_partial_ord_to_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>> for $integer
        where
            Self: PartialEq<BigInt<Digit, SEPARATOR, SHIFT>>,
            BigInt<Digit, SEPARATOR, SHIFT>: Signed,
        {
            fn partial_cmp(
                &self,
                other: &BigInt<Digit, SEPARATOR, SHIFT>,
            ) -> Option<Ordering> {
                Some(match other.sign() {
                    Sign::Negative => Ordering::Greater,
                    Sign::Positive => {
                        if self.is_zero() {
                            Ordering::Less
                        } else {
                            compare_digits(
                                &Digit::digits_from_non_zero_value::<SHIFT>(
                                    *self,
                                ),
                                &other.digits,
                            )
                        }
                    }
                    Sign::Zero => {
                        if self.is_zero() {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    }
                })
            }
        }
    )*)
}

unsigned_primitive_partial_ord_to_big_int_impl!(u8 u16 u32 u64 u128 usize);
