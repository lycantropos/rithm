use std::ops::{Add, Mul};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> Add
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (numerator, denominator) = (self.numerator * &other.denominator
            + &self.denominator * other.numerator)
            .normalize_moduli(self.denominator * other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        let (numerator, denominator) = (self.numerator * &other.denominator
            + &self.denominator * &other.numerator)
            .normalize_moduli(self.denominator * &other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Add<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
    BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn add(
        self,
        other: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        let (numerator, denominator) = (&self.numerator * &other.denominator
            + &self.denominator * other.numerator)
            .normalize_moduli(&self.denominator * other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn add(self, other: Self) -> Self::Output {
        let (numerator, denominator) = (&self.numerator * &other.denominator
            + &self.denominator * &other.numerator)
            .normalize_moduli(&self.denominator * &other.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
    BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Self;

    fn add(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            + &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Self;

    fn add(self, other: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            + &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Add<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn add(self, other: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            + &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Add<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn add(self, other: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            + &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Fraction<Self>: Add<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    #[inline]
    fn add(self, other: Fraction<Self>) -> Self::Output {
        other + self
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Add<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a Fraction<Self>: Add<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    #[inline]
    fn add(self, other: &Fraction<Self>) -> Self::Output {
        other + self
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Add<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Fraction<BigInt<Digit, DIGIT_BITNESS>>:
        Add<Self, Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    #[inline]
    fn add(
        self,
        other: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        other + self
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Add<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>: Add<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    #[inline]
    fn add(
        self,
        other: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        other + self
    }
}

macro_rules! integer_fraction_add_impl {
    ($($integer:ty)*) => ($(
        impl Add for Fraction<$integer> {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                let (numerator, denominator) = (self.numerator
                    * other.denominator
                    + self.denominator * other.numerator)
                    .normalize_moduli(self.denominator * other.denominator);
                Self::Output {
                    numerator,
                    denominator,
                }
            }
        }

        impl Add<$integer> for Fraction<$integer> {
            type Output = Self;

            fn add(self, other: $integer) -> Self::Output {
                let (numerator, denominator) = (self.numerator
                    + self.denominator * other)
                    .normalize_moduli(self.denominator);
                Self::Output {
                    numerator,
                    denominator,
                }
            }
        }

        impl Add<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            #[inline]
            fn add(self, other: Fraction<Self>) -> Self::Output {
                other + self
            }
        }
    )*)
}

integer_fraction_add_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
