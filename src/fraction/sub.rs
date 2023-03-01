use std::ops::{Mul, Sub};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> Sub
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
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

impl<Digit, const DIGIT_BITNESS: usize> Sub<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    Sub<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
    BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn sub(
        self,
        other: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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

impl<Digit, const DIGIT_BITNESS: usize> Sub
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

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

impl<Digit, const DIGIT_BITNESS: usize> Sub<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
    BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Self;

    fn sub(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            - &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Sub<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Self;

    fn sub(self, other: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            - &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Sub<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + Sub<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn sub(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Sub<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Sub<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn sub(self, other: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            - &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Sub<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit, const DIGIT_BITNESS: usize> Sub<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit, const DIGIT_BITNESS: usize>
    Sub<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn sub(
        self,
        other: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    Sub<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Sub<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn sub(
        self,
        other: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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
