use std::cmp::Ordering;

use crate::traits::{Signed, Zeroable};

use super::digits::{
    digits_lesser_than, non_zero_value_to_digits, value_to_sign, ConstructibleFrom,
};
use super::types::{BigInt, Sign};

impl<Digit: Clone + PartialOrd + Zeroable, const SEPARATOR: char, const SHIFT: usize> PartialOrd
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn ge(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && !{
                    if self.is_positive() {
                        digits_lesser_than(&self.digits, &other.digits)
                    } else {
                        digits_lesser_than(&other.digits, &self.digits)
                    }
                }
    }

    fn gt(&self, other: &Self) -> bool {
        self.sign > other.sign
            || self.sign == other.sign
                && if self.is_positive() {
                    digits_lesser_than(&other.digits, &self.digits)
                } else {
                    digits_lesser_than(&self.digits, &other.digits)
                }
    }

    fn le(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && !{
                    if self.is_positive() {
                        digits_lesser_than(&other.digits, &self.digits)
                    } else {
                        digits_lesser_than(&self.digits, &other.digits)
                    }
                }
    }

    fn lt(&self, other: &Self) -> bool {
        self.sign < other.sign
            || self.sign == other.sign
                && if self.is_positive() {
                    digits_lesser_than(&self.digits, &other.digits)
                } else {
                    digits_lesser_than(&other.digits, &self.digits)
                }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

macro_rules! big_int_partial_ord_to_signed_primitive_impl {
    ($($t:ty)*) => ($(
        impl<
                Digit: ConstructibleFrom<$t> + PartialOrd + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<$t> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn ge(&self, other: &$t) -> bool {
                self.sign > ((*other).signum() as Sign)
                    || self.sign == ((*other).signum() as Sign)
                        && (other.is_zero()
                            || !{
                                if self.is_positive() {
                                    digits_lesser_than(
                                        &self.digits,
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                    )
                                } else {
                                    digits_lesser_than(
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                        &self.digits,
                                    )
                                }
                            })
            }

            fn gt(&self, other: &$t) -> bool {
                self.sign > ((*other).signum() as Sign)
                    || !other.is_zero()
                        && self.sign == ((*other).signum() as Sign)
                        && if self.is_positive() {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                &self.digits,
                            )
                        } else {
                            digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            )
                        }
            }

            fn le(&self, other: &$t) -> bool {
                self.sign < ((*other).signum() as Sign)
                    || self.sign == ((*other).signum() as Sign)
                        && (other.is_zero()
                            || !{
                                if self.is_positive() {
                                    digits_lesser_than(
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                        &self.digits,
                                    )
                                } else {
                                    digits_lesser_than(
                                        &self.digits,
                                        &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                    )
                                }
                            })
            }

            fn lt(&self, other: &$t) -> bool {
                self.sign < ((*other).signum() as Sign)
                    || !other.is_zero()
                        && self.sign == ((*other).signum() as Sign)
                        && if self.is_positive() {
                            digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            )
                        } else {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                                &self.digits,
                            )
                        }
            }

            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                Some(if self.lt(other) {
                    Ordering::Less
                } else if self.gt(other) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

big_int_partial_ord_to_signed_primitive_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! big_int_partial_ord_to_unsigned_primitive_impl {
    ($($t:ty)*) => ($(
        impl<
                Digit: ConstructibleFrom<$t> + PartialOrd + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<$t> for BigInt<Digit, SEPARATOR, SHIFT>
        {
            fn ge(&self, other: &$t) -> bool {
                self.is_zero() && other.is_zero()
                    || self.is_positive()
                        && (other.is_zero()
                            || !digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            ))
            }

            fn gt(&self, other: &$t) -> bool {
                self.is_positive()
                    && (other.is_zero()
                        || digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            &self.digits,
                        ))
            }

            fn le(&self, other: &$t) -> bool {
                !self.is_positive()
                    || !other.is_zero()
                        && digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            &self.digits,
                        )
            }

            fn lt(&self, other: &$t) -> bool {
                self.is_negative()
                    || !other.is_zero()
                        && (self.is_zero()
                            || digits_lesser_than(
                                &self.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*other),
                            ))
            }

            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                Some(if self.lt(other) {
                    Ordering::Less
                } else if self.gt(other) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

big_int_partial_ord_to_unsigned_primitive_impl!(u8 u16 u32 u64 u128 usize);

macro_rules! signed_primitive_partial_ord_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<
                Digit: ConstructibleFrom<$t> + PartialOrd + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            fn le(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) < other.sign
                    || value_to_sign(*self) == other.sign
                        && !{
                            if self.is_positive() {
                                digits_lesser_than(
                                    &other.digits,
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                )
                            } else {
                                digits_lesser_than(
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                    &other.digits,
                                )
                            }
                        }
            }

            fn lt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) < other.sign
                    || value_to_sign(*self) == other.sign
                        && if self.is_positive() {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                &other.digits,
                            )
                        } else {
                            digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            )
                        }
            }

            fn ge(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) > other.sign
                    || value_to_sign(*self) == other.sign
                        && !{
                            if self.is_positive() {
                                digits_lesser_than(
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                    &other.digits,
                                )
                            } else {
                                digits_lesser_than(
                                    &other.digits,
                                    &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                )
                            }
                        }
            }

            fn gt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                value_to_sign(*self) > other.sign
                    || value_to_sign(*self) == other.sign
                        && if self.is_positive() {
                            digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            )
                        } else {
                            digits_lesser_than(
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                                &other.digits,
                            )
                        }
            }

            fn partial_cmp(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Option<Ordering> {
                Some(if self.lt(other) {
                    Ordering::Less
                } else if self.gt(other) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

signed_primitive_partial_ord_to_big_int_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_primitive_partial_ord_to_big_int_impl {
    ($($t:ty)*) => ($(
        impl<
                Digit: ConstructibleFrom<$t> + PartialOrd + Zeroable,
                const SEPARATOR: char,
                const SHIFT: usize,
            > PartialOrd<BigInt<Digit, SEPARATOR, SHIFT>> for $t
        {
            fn ge(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                !other.is_positive()
                    || !self.is_zero()
                        && digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            &other.digits,
                        )
            }

            fn gt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                other.is_negative()
                    || !self.is_zero()
                        && (other.is_zero()
                            || digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            ))
            }

            fn le(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                self.is_zero() && other.is_zero()
                    || other.is_positive()
                        && (self.is_zero()
                            || !digits_lesser_than(
                                &other.digits,
                                &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            ))
            }

            fn lt(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
                other.is_positive()
                    && (self.is_zero()
                        || digits_lesser_than(
                            &non_zero_value_to_digits::<$t, Digit, SHIFT>(*self),
                            &other.digits,
                        ))
            }

            fn partial_cmp(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Option<Ordering> {
                Some(if other.lt(self) {
                    Ordering::Less
                } else if other.gt(self) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                })
            }
        }
    )*)
}

unsigned_primitive_partial_ord_to_big_int_impl!(u8 u16 u32 u64 u128 usize);
