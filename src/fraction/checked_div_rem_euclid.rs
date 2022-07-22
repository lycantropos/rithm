use std::ops::Mul;

use traiter::numbers::CheckedDivRemEuclid;

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
{
    type Output = Option<(BigInt<Digit, SEPARATOR, SHIFT>, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        let (quotient, remainder_numerator) = (self.numerator
            * &divisor.denominator)
            .checked_div_rem_euclid(&self.denominator * divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) = remainder_numerator
            .normalize_moduli(self.denominator * divisor.denominator);
        Some((
            quotient,
            Self {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&Self> for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + Mul<
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
    type Output = Option<(BigInt<Digit, SEPARATOR, SHIFT>, Self)>;

    fn checked_div_rem_euclid(self, divisor: &Self) -> Self::Output {
        let (quotient, remainder_numerator) = (self.numerator
            * &divisor.denominator)
            .checked_div_rem_euclid(&self.denominator * &divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) = remainder_numerator
            .normalize_moduli(self.denominator * &divisor.denominator);
        Some((
            quotient,
            Self {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = (&self.numerator
            * &divisor.denominator)
            .checked_div_rem_euclid(&self.denominator * divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) = remainder_numerator
            .normalize_moduli(&self.denominator * divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDivRemEuclid
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    )>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        let (quotient, remainder_numerator) = (&self.numerator
            * &divisor.denominator)
            .checked_div_rem_euclid(&self.denominator * &divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) = remainder_numerator
            .normalize_moduli(&self.denominator * &divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Option<(BigInt<Digit, SEPARATOR, SHIFT>, Self)>;

    fn checked_div_rem_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = self
            .numerator
            .checked_div_rem_euclid(divisor * &self.denominator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(self.denominator);
        Some((
            quotient,
            Self {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Option<(BigInt<Digit, SEPARATOR, SHIFT>, Self)>;

    fn checked_div_rem_euclid(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = self
            .numerator
            .checked_div_rem_euclid(divisor * &self.denominator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(self.denominator);
        Some((
            quotient,
            Self {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<(
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        )>,
    >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = self
            .numerator
            .checked_div_rem_euclid(divisor * &self.denominator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&self.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = self
            .numerator
            .checked_div_rem_euclid(divisor * &self.denominator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&self.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedDivRemEuclid<Self, Output = Option<(Self, Self)>>
        + Mul<&'a Self, Output = Self>
        + NormalizeModuli<Output = (Self, Self)>,
{
    type Output = Option<(Self, Fraction<Self>)>;

    fn checked_div_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        let (quotient, remainder_numerator) = (self * &divisor.denominator)
            .checked_div_rem_euclid(divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(divisor.denominator);
        Some((
            quotient,
            Fraction::<Self> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedDivRemEuclid<&'a Self, Output = Option<(Self, Self)>>
        + Mul<&'a Self, Output = Self>
        + NormalizeModuli<&'a Self, Output = (Self, Self)>,
{
    type Output = Option<(Self, Fraction<Self>)>;

    fn checked_div_rem_euclid(self, divisor: &Fraction<Self>) -> Self::Output {
        let (quotient, remainder_numerator) = (self * &divisor.denominator)
            .checked_div_rem_euclid(&divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&divisor.denominator);
        Some((
            quotient,
            Fraction::<Self> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = (self * &divisor.denominator)
            .checked_div_rem_euclid(divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivRemEuclid<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
{
    type Output = Option<(
        BigInt<Digit, SEPARATOR, SHIFT>,
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = (self * &divisor.denominator)
            .checked_div_rem_euclid(&divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

macro_rules! integer_checked_div_rem_euclid_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDivRemEuclid for Fraction<$integer> {
            type Output = Option<($integer, Self)>;

            fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
                let (quotient, remainder_numerator) = (self.numerator
                    * divisor.denominator)
                    .checked_div_rem_euclid(
                        divisor.numerator * self.denominator,
                    )?;
                let (remainder_numerator, remainder_denominator) =
                    remainder_numerator.normalize_moduli(
                        self.denominator * divisor.denominator,
                    );
                Some((
                    quotient,
                    Self {
                        numerator: remainder_numerator,
                        denominator: remainder_denominator,
                    },
                ))
            }
        }

        impl CheckedDivRemEuclid<$integer> for Fraction<$integer> {
            type Output = Option<($integer, Self)>;

            fn checked_div_rem_euclid(
                self,
                divisor: $integer,
            ) -> Self::Output {
                let (quotient, remainder_numerator) = (self.numerator)
                    .checked_div_rem_euclid(divisor * self.denominator)?;
                let (remainder_numerator, remainder_denominator) =
                    remainder_numerator.normalize_moduli(self.denominator);
                Some((
                    quotient,
                    Self {
                        numerator: remainder_numerator,
                        denominator: remainder_denominator,
                    },
                ))
            }
        }

        impl CheckedDivRemEuclid<Fraction<Self>> for $integer {
            type Output = Option<(Self, Fraction<Self>)>;

            fn checked_div_rem_euclid(
                self,
                divisor: Fraction<Self>,
            ) -> Self::Output {
                let (quotient, remainder_numerator) = (self
                    * divisor.denominator)
                .checked_div_rem_euclid(divisor.numerator)?;
                let (remainder_numerator, remainder_denominator) =
                    <$integer>::normalize_moduli(
                        remainder_numerator,
                        divisor.denominator,
                    );
                Some((
                    quotient,
                    Fraction::<Self> {
                        numerator: remainder_numerator,
                        denominator: remainder_denominator,
                    },
                ))
            }
        }
    )*)
}

integer_checked_div_rem_euclid_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
