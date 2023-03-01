use std::cmp::Ordering;

use traiter::numbers::{Sign, Signed, Zeroable};

use super::digits::{compare_digits, DigitsFromNonZeroValue};
use super::types::{BigInt, Sign as BigIntSign};

impl<Digit: Ord, const DIGIT_BITNESS: usize> PartialOrd
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: PartialEq + Signed,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.sign.cmp(&other.sign) {
            Ordering::Equal => match self.sign() {
                Sign::Negative => compare_digits(&other.digits, &self.digits),
                Sign::Positive => compare_digits(&self.digits, &other.digits),
                Sign::Zero => Ordering::Equal,
            },
            value => value,
        })
    }
}

macro_rules! big_int_partial_ord_signed_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const DIGIT_BITNESS: usize,
            > PartialOrd<$integer> for BigInt<Digit, DIGIT_BITNESS>
        where
            Self: PartialEq<$integer> + Signed,
        {
            fn partial_cmp(&self, other: &$integer) -> Option<Ordering> {
                Some(match self.sign.cmp(&(other.signum() as BigIntSign)) {
                    Ordering::Equal => match self.sign() {
                        Sign::Negative => compare_digits(
                            &Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(
                                *other,
                            ),
                            &self.digits,
                        ),
                        Sign::Positive => compare_digits(
                            &self.digits,
                            &Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(
                                *other,
                            ),
                        ),
                        Sign::Zero => Ordering::Equal,
                    },
                    value => value,
                })
            }
        }
    )*)
}

big_int_partial_ord_signed_integer_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! big_int_partial_ord_unsigned_integer_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const DIGIT_BITNESS: usize,
            > PartialOrd<$integer> for BigInt<Digit, DIGIT_BITNESS>
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
                                &Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(
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

big_int_partial_ord_unsigned_integer_impl!(u8 u16 u32 u64 u128 usize);

macro_rules! signed_integer_partial_ord_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const DIGIT_BITNESS: usize,
            > PartialOrd<BigInt<Digit, DIGIT_BITNESS>> for $integer
        where
            BigInt<Digit, DIGIT_BITNESS>: Signed,
            Self: PartialEq<BigInt<Digit, DIGIT_BITNESS>>,
        {
            fn partial_cmp(
                &self,
                other: &BigInt<Digit, DIGIT_BITNESS>,
            ) -> Option<Ordering> {
                Some(match (self.signum() as BigIntSign).cmp(&other.sign) {
                    Ordering::Equal => match other.sign() {
                        Sign::Negative => compare_digits(
                            &other.digits,
                            &Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(*self),
                        ),
                        Sign::Positive => compare_digits(
                            &Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(*self),
                            &other.digits,
                        ),
                        Sign::Zero => Ordering::Equal,
                    },
                    value => value,
                })
            }
        }
    )*)
}

signed_integer_partial_ord_big_int_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_partial_ord_big_int_impl {
    ($($integer:ty)*) => ($(
        impl<
                Digit: DigitsFromNonZeroValue<$integer> + Ord,
                const DIGIT_BITNESS: usize,
            > PartialOrd<BigInt<Digit, DIGIT_BITNESS>> for $integer
        where
            BigInt<Digit, DIGIT_BITNESS>: Signed,
            Self: PartialEq<BigInt<Digit, DIGIT_BITNESS>>,
        {
            fn partial_cmp(
                &self,
                other: &BigInt<Digit, DIGIT_BITNESS>,
            ) -> Option<Ordering> {
                Some(match other.sign() {
                    Sign::Negative => Ordering::Greater,
                    Sign::Positive => {
                        if self.is_zero() {
                            Ordering::Less
                        } else {
                            compare_digits(
                                &Digit::digits_from_non_zero_value::<DIGIT_BITNESS>(
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

unsigned_integer_partial_ord_big_int_impl!(u8 u16 u32 u64 u128 usize);
