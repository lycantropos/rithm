use traiter::numbers::{CheckedPow, Pow};

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Pow<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedPow<BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Pow<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self:
        CheckedPow<&'a BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn pow(self, exponent: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Pow<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedPow<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn pow(self, exponent: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_pow(exponent)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Pow<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedPow<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn pow(self, exponent: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
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
