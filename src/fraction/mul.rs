use std::ops::Mul;

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn mul(
        self,
        other: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Self;

    fn mul(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Self;

    fn mul(self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn mul(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: &self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn mul(self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (denominator, other) = self.denominator.normalize_moduli(other);
        Self::Output {
            numerator: &self.numerator * other,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Fraction<Self>: Mul<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    fn mul(self, other: Fraction<Self>) -> Self::Output {
        other * self
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Mul<&Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a Fraction<Self>: Mul<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    fn mul(self, other: &Fraction<Self>) -> Self::Output {
        other * self
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Mul<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn mul(
        self,
        other: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        other * self
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Mul<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Mul<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn mul(
        self,
        other: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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
