use std::ops::Neg;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Neg
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Neg<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Neg
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Clone,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Neg<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

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
