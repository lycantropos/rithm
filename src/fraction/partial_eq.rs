use crate::big_int::BigInt;
use crate::traits::Unitary;

use super::types::Fraction;

impl<Component: Clone + PartialEq> PartialEq for Fraction<Component> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.eq(&other.numerator) && self.denominator.eq(&other.denominator)
    }
}

impl<Component: Clone + PartialEq + Unitary> PartialEq<Component> for Fraction<Component> {
    fn eq(&self, other: &Component) -> bool {
        self.denominator.is_one() && self.numerator.eq(other)
    }
}

impl<
        Component: Clone + PartialEq<Self> + Unitary,
        Digit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > PartialEq<Fraction<Component>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn eq(&self, other: &Fraction<Component>) -> bool {
        other.denominator.is_one() && other.numerator.eq(self)
    }
}

macro_rules! primitive_partial_eq_fraction_impl {
    ($($t:ty)*) => ($(
    impl<Component: Clone + PartialEq<$t> + Unitary> PartialEq<Fraction<Component>> for $t {
        fn eq(&self, other: &Fraction<Component>) -> bool {
            other.denominator.is_one() && other.numerator.eq(self)
        }
    }
    )*)
}

primitive_partial_eq_fraction_impl!(i8 i16 i32 i64 i128 isize);
