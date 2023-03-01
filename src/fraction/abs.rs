use traiter::numbers::Abs;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Abs
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
{
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self::Output {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Abs
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Clone,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Abs<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn abs(self) -> Self::Output {
        Self::Output {
            numerator: (&self.numerator).abs(),
            denominator: self.denominator.clone(),
        }
    }
}

macro_rules! signed_integer_fraction_abs_impl {
    ($($integer:ty)*) => ($(
        impl Abs for Fraction<$integer> {
            type Output = Self;

            fn abs(self) -> Self::Output {
                Self::Output {
                    numerator: self.numerator.abs(),
                    denominator: self.denominator,
                }
            }
        }
    )*)
}

signed_integer_fraction_abs_impl!(i8 i16 i32 i64 i128 isize);
