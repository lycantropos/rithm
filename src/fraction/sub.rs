use std::ops::{Mul, Sub};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (numerator, denominator) = (self.numerator * &other.denominator
            - &self.denominator * other.numerator)
            .normalize_moduli(self.denominator * other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        let (numerator, denominator) = (self.numerator * &other.denominator
            - &self.denominator * &other.numerator)
            .normalize_moduli(self.denominator * &other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn sub(
        self,
        other: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        let (numerator, denominator) = (&self.numerator * &other.denominator
            - &self.denominator * other.numerator)
            .normalize_moduli(&self.denominator * other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn sub(self, other: Self) -> Self::Output {
        let (numerator, denominator) = (&self.numerator * &other.denominator
            - &self.denominator * &other.numerator)
            .normalize_moduli(&self.denominator * &other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Self;

    fn sub(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            - &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Self;

    fn sub(self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            - &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Sub<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn sub(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Sub<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn sub(self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: Mul<&'a Self, Output = Self>
        + NormalizeModuli<Output = (Self, Self)>
        + Sub<Output = Self>,
{
    type Output = Fraction<Self>;

    fn sub(self, other: Fraction<Self>) -> Self::Output {
        let (numerator, denominator) = (self * &other.denominator
            - other.numerator)
            .normalize_moduli(other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Sub<&Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: Mul<&'a Self, Output = Self>
        + NormalizeModuli<&'a Self, Output = (Self, Self)>
        + Sub<&'a Self, Output = Self>,
{
    type Output = Fraction<Self>;

    fn sub(self, other: &Fraction<Self>) -> Self::Output {
        let (numerator, denominator) = (self * &other.denominator
            - &other.numerator)
            .normalize_moduli(&other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn sub(
        self,
        other: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        let (numerator, denominator) = (self * &other.denominator
            - other.numerator)
            .normalize_moduli(other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Sub<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Sub<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn sub(
        self,
        other: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        let (numerator, denominator) = (self * &other.denominator
            - &other.numerator)
            .normalize_moduli(&other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

macro_rules! integer_fraction_sub_impl {
    ($($integer:ty)*) => ($(
        impl Sub for Fraction<$integer> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let (numerator, denominator) = (self.numerator
                    * other.denominator
                    - self.denominator * other.numerator)
                    .normalize_moduli(self.denominator * other.denominator);
                Self::Output {
                    numerator,
                    denominator,
                }
            }
        }

        impl Sub<$integer> for Fraction<$integer> {
            type Output = Self;

            fn sub(self, other: $integer) -> Self::Output {
                let (numerator, denominator) = (self.numerator
                    - self.denominator * other)
                    .normalize_moduli(self.denominator);
                Self::Output {
                    numerator,
                    denominator,
                }
            }
        }

        impl Sub<Fraction<$integer>> for $integer {
            type Output = Fraction<Self>;

            fn sub(self, other: Fraction<Self>) -> Self::Output {
                let (numerator, denominator) = (self * other.denominator
                    - other.numerator)
                    .normalize_moduli(other.denominator);
                Self::Output {
                    numerator,
                    denominator,
                }
            }
        }
    )*)
}

integer_fraction_sub_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
