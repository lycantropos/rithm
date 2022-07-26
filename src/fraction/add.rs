use std::ops::{Add, Mul};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
    BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn add(
        self,
        other: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Self;

    fn add(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            + &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Self;

    fn add(self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (self.numerator
            + &self.denominator * other)
            .normalize_moduli(self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Add<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn add(self, other: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            + &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Add<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn add(self, other: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        let (numerator, denominator) = (&self.numerator
            + &self.denominator * other)
            .normalize_moduli(&self.denominator);
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Fraction<Self>: Add<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    #[inline]
    fn add(self, other: Fraction<Self>) -> Self::Output {
        other + self
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Add<&Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a Fraction<Self>: Add<Self, Output = Fraction<Self>>,
{
    type Output = Fraction<Self>;

    #[inline]
    fn add(self, other: &Fraction<Self>) -> Self::Output {
        other + self
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>:
        Add<Self, Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    #[inline]
    fn add(
        self,
        other: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        other + self
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Add<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Add<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    #[inline]
    fn add(
        self,
        other: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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
