use std::ops::Mul;

use traiter::numbers::CheckedDivRemEuclid;

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivRemEuclid
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
{
    type Output = Option<(BigInt<Digit, DIGIT_BITNESS>, Self)>;

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

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivRemEuclid<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + Mul<
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
    type Output = Option<(BigInt<Digit, DIGIT_BITNESS>, Self)>;

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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Option<(
        BigInt<Digit, DIGIT_BITNESS>,
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = (&self.numerator
            * &divisor.denominator)
            .checked_div_rem_euclid(&self.denominator * divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) = remainder_numerator
            .normalize_moduli(&self.denominator * divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivRemEuclid
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Option<(
        BigInt<Digit, DIGIT_BITNESS>,
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    )>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        let (quotient, remainder_numerator) = (&self.numerator
            * &divisor.denominator)
            .checked_div_rem_euclid(&self.denominator * &divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) = remainder_numerator
            .normalize_moduli(&self.denominator * &divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Option<(BigInt<Digit, DIGIT_BITNESS>, Self)>;

    fn checked_div_rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Option<(BigInt<Digit, DIGIT_BITNESS>, Self)>;

    fn checked_div_rem_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<(
            BigInt<Digit, DIGIT_BITNESS>,
            BigInt<Digit, DIGIT_BITNESS>,
        )>,
    >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Option<(
        BigInt<Digit, DIGIT_BITNESS>,
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = self
            .numerator
            .checked_div_rem_euclid(divisor * &self.denominator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&self.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
{
    type Output = Option<(
        BigInt<Digit, DIGIT_BITNESS>,
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = self
            .numerator
            .checked_div_rem_euclid(divisor * &self.denominator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&self.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivRemEuclid<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivRemEuclid<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Option<(
        BigInt<Digit, DIGIT_BITNESS>,
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = (self * &divisor.denominator)
            .checked_div_rem_euclid(divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivRemEuclid<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
{
    type Output = Option<(
        BigInt<Digit, DIGIT_BITNESS>,
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    )>;

    fn checked_div_rem_euclid(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        let (quotient, remainder_numerator) = (self * &divisor.denominator)
            .checked_div_rem_euclid(&divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            remainder_numerator.normalize_moduli(&divisor.denominator);
        Some((
            quotient,
            Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
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
