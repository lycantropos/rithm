use std::cmp::Ordering;
use std::ops::Mul;

use traiter::numbers::Unitary;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Component: Clone + Mul<Output = Component> + PartialOrd> PartialOrd
    for Fraction<Component>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.numerator.clone() * other.denominator.clone())
            .partial_cmp(&(other.numerator.clone() * self.denominator.clone()))
    }
}

impl<Component: Clone + Mul<Output = Component> + PartialOrd + Unitary>
    PartialOrd<Component> for Fraction<Component>
{
    fn partial_cmp(&self, other: &Component) -> Option<Ordering> {
        self.numerator
            .partial_cmp(&(other.clone() * self.denominator.clone()))
    }
}

impl<Component: Clone, Digit, const SEPARATOR: char, const SHIFT: usize>
    PartialOrd<Fraction<Component>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Clone
        + Mul<Component, Output = Self>
        + PartialEq<Fraction<Component>>
        + PartialOrd<Component>,
{
    fn partial_cmp(&self, other: &Fraction<Component>) -> Option<Ordering> {
        (self.clone() * other.denominator.clone())
            .partial_cmp(&other.numerator)
    }
}

macro_rules! signed_integer_partial_ord_fraction_impl {
    ($($integer:ty)*) => ($(
    impl PartialOrd<Fraction<Self>> for $integer
    {
        fn ge(&self, other: &Fraction<Self>) -> bool {
            self * other.denominator >= other.numerator
        }

        fn gt(&self, other: &Fraction<Self>) -> bool {
            self * other.denominator > other.numerator
        }

        fn le(&self, other: &Fraction<Self>) -> bool {
            self * other.denominator <= other.numerator
        }

        fn lt(&self, other: &Fraction<Self>) -> bool {
            self * other.denominator < other.numerator
        }

        fn partial_cmp(&self, other: &Fraction<Self>) -> Option<Ordering> {
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

signed_integer_partial_ord_fraction_impl!(i8 i16 i32 i64 i128 isize);
