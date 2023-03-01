use std::ops::Mul;

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> Mul
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (numerator, other_denominator) =
            self.numerator.normalize_moduli(other.denominator);
        let (denominator, other_numerator) =
            self.denominator.normalize_moduli(other.numerator);
        Self::Output {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        let (numerator, other_denominator) =
            self.numerator.normalize_moduli(&other.denominator);
        let (denominator, other_numerator) =
            self.denominator.normalize_moduli(&other.numerator);
        Self::Output {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Mul<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn mul(
        self,
        other: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        let (numerator, other_denominator) =
            self.numerator.normalize_moduli(other.denominator);
        let (denominator, other_numerator) =
            self.denominator.normalize_moduli(other.numerator);
        Self::Output {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn mul(self, other: Self) -> Self::Output {
        let (numerator, other_denominator) =
            self.numerator.normalize_moduli(&other.denominator);
        let (denominator, other_numerator) =
            self.denominator.normalize_moduli(&other.numerator);
        Self::Output {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Self;

    fn mul(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Self;

    fn mul(self, other: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn mul(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: &self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn mul(self, other: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: &self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Fraction<Self>: Mul<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    fn mul(self, other: Fraction<Self>) -> Self::Output {
        other * self
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Mul<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a Fraction<Self>: Mul<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    fn mul(self, other: &Fraction<Self>) -> Self::Output {
        other * self
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Mul<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Fraction<BigInt<Digit, DIGIT_BITNESS>>: Mul<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn mul(
        self,
        other: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        other * self
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Mul<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>: Mul<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn mul(
        self,
        other: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        other * self
    }
}

macro_rules! integer_mul_fraction_impl {
    ($($integer:ty)*) => ($(
        impl Mul for Fraction<$integer> {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                let (numerator, other_denominator) =
                    self.numerator.normalize_moduli(other.denominator);
                let (other_numerator, denominator) =
                    other.numerator.normalize_moduli(self.denominator);
                Self::Output {
                    numerator: numerator * other_numerator,
                    denominator: denominator * other_denominator,
                }
            }
        }

        impl Mul<$integer> for Fraction<$integer> {
            type Output = Self;

            fn mul(self, other: $integer) -> Self::Output {
                let (other, denominator) =
                    other.normalize_moduli(self.denominator);
                Self::Output {
                    numerator: self.numerator * other,
                    denominator,
                }
            }
        }

        impl Mul<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn mul(self, other: Fraction<Self>) -> Self::Output {
                other * self
            }
        }
    )*)
}

integer_mul_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
