use traiter::numbers::Unitary;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> PartialEq
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.numerator.eq(&other.numerator)
            && self.denominator.eq(&other.denominator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    PartialEq<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: PartialEq + Unitary,
{
    fn eq(&self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> bool {
        self.denominator.is_one() && self.numerator.eq(other)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    PartialEq<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: PartialEq + Unitary,
{
    fn eq(&self, other: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>) -> bool {
        other.denominator.is_one() && other.numerator.eq(self)
    }
}

macro_rules! integer_fraction_partial_eq_impl {
    ($($integer:ty)*) => ($(
        impl PartialEq for Fraction<$integer> {
            fn eq(&self, other: &Self) -> bool {
                self.numerator.eq(&other.numerator)
                    && self.denominator.eq(&other.denominator)
            }
        }

        impl PartialEq<$integer> for Fraction<$integer> {
            fn eq(&self, other: &$integer) -> bool {
                self.denominator.is_one() && self.numerator.eq(other)
            }
        }

        impl PartialEq<Fraction<$integer>> for $integer {
            fn eq(&self, other: &Fraction<$integer>) -> bool {
                other.denominator.is_one() && other.numerator.eq(self)
            }
        }
    )*)
}

integer_fraction_partial_eq_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
