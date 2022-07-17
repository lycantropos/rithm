use traiter::numbers::Unitary;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Component: PartialEq> PartialEq for Fraction<Component> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.eq(&other.numerator)
            && self.denominator.eq(&other.denominator)
    }
}

impl<Component: PartialEq + Unitary> PartialEq<Component>
    for Fraction<Component>
{
    fn eq(&self, other: &Component) -> bool {
        self.denominator.is_one() && self.numerator.eq(other)
    }
}

impl<
        Component: PartialEq<Self> + Unitary,
        Digit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > PartialEq<Fraction<Component>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn eq(&self, other: &Fraction<Component>) -> bool {
        other.denominator.is_one() && other.numerator.eq(self)
    }
}

macro_rules! signed_integer_partial_eq_fraction_impl {
    ($($integer:ty)*) => ($(
        impl<Component: PartialEq<$integer> + Unitary>
            PartialEq<Fraction<Component>> for $integer
        {
            fn eq(&self, other: &Fraction<Component>) -> bool {
                other.denominator.is_one() && other.numerator.eq(self)
            }
        }
    )*)
}

signed_integer_partial_eq_fraction_impl!(i8 i16 i32 i64 i128 isize);
