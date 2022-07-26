use std::ops::Neg;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Neg
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>:
        Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Neg
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Neg<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn neg(self) -> Self::Output {
        Self::Output {
            numerator: -(&self.numerator),
            denominator: self.denominator.clone(),
        }
    }
}

macro_rules! signed_integer_fraction_neg_impl {
    ($($integer:ty)*) => ($(
        impl Neg for Fraction<$integer> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::Output {
                    numerator: -self.numerator,
                    denominator: self.denominator,
                }
            }
        }
    )*)
}

signed_integer_fraction_neg_impl!(i8 i16 i32 i64 i128 isize);
