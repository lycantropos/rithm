use traiter::numbers::Abs;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Abs
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
{
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self::Output {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Abs
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Abs<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

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
