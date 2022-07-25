use traiter::numbers::{CheckedPow, Pow};

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Pow<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self: CheckedPow<BigInt<Digit, SEPARATOR, SHIFT>, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Pow<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> Self:
        CheckedPow<&'a BigInt<Digit, SEPARATOR, SHIFT>, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Pow<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self: CheckedPow<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn pow(self, exponent: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Pow<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> Self: CheckedPow<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn pow(self, exponent: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! signed_integer_fraction_pow_impl {
    ($($integer:ty)*) => ($(
        impl Pow<u32> for Fraction<$integer> {
            type Output = Self;

            fn pow(self, exponent: u32) -> Self::Output {
                Self::Output {
                    numerator: self.numerator.pow(exponent),
                    denominator: self.denominator.pow(exponent),
                }
            }
        }
    )*)
}

signed_integer_fraction_pow_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
