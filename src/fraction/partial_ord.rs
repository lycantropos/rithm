use std::cmp::Ordering;

use crate::big_int::{BigInt, GcdDigit, MultiplicativeDigit};
use crate::traits::MultiplicativeMonoid;

use super::types::Fraction;

impl<Component: Clone + MultiplicativeMonoid + PartialOrd> PartialOrd for Fraction<Component> {
    fn ge(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            >= other.numerator.clone() * self.denominator.clone()
    }

    fn gt(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            > other.numerator.clone() * self.denominator.clone()
    }

    fn le(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            <= other.numerator.clone() * self.denominator.clone()
    }

    fn lt(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            < other.numerator.clone() * self.denominator.clone()
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

impl<Component: Clone + MultiplicativeMonoid + PartialOrd> PartialOrd<Component>
    for Fraction<Component>
{
    fn ge(&self, other: &Component) -> bool {
        self.numerator >= other.clone() * self.denominator.clone()
    }

    fn gt(&self, other: &Component) -> bool {
        self.numerator > other.clone() * self.denominator.clone()
    }

    fn le(&self, other: &Component) -> bool {
        self.numerator <= other.clone() * self.denominator.clone()
    }

    fn lt(&self, other: &Component) -> bool {
        self.numerator < other.clone() * self.denominator.clone()
    }

    fn partial_cmp(&self, other: &Component) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl<
        Digit: Clone + GcdDigit + MultiplicativeDigit + PartialOrd,
        const SEPARATOR: char,
        const SHIFT: usize,
    > PartialOrd<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn ge(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() >= other.numerator
    }

    fn gt(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() > other.numerator
    }

    fn le(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() <= other.numerator
    }

    fn lt(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() < other.numerator
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

macro_rules! primitive_partial_ord_fraction_impl {
    ($($t:ty)*) => ($(
    impl PartialOrd<Fraction<Self>> for $t
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

primitive_partial_ord_fraction_impl!(i8 i16 i32 i64 i128 isize);
