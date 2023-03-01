use std::ops::Mul;

use traiter::numbers::{CheckedDiv, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli, NormalizeSign};

impl<Digit, const DIGIT_BITNESS: usize> CheckedDiv
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor.numerator);
            let (dividend_denominator, divisor_denominator) =
                self.denominator.normalize_moduli(divisor.denominator);
            let (numerator, denominator) = (dividend_numerator
                * divisor_denominator)
                .normalize_sign(dividend_denominator * divisor_numerator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDiv<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: &Self) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(&divisor.numerator);
            let (dividend_denominator, divisor_denominator) =
                self.denominator.normalize_moduli(&divisor.denominator);
            let (numerator, denominator) = (dividend_numerator
                * divisor_denominator)
                .normalize_sign(dividend_denominator * divisor_numerator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_div(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor.numerator);
            let (dividend_denominator, divisor_denominator) =
                self.denominator.normalize_moduli(divisor.denominator);
            let (numerator, denominator) = (dividend_numerator
                * divisor_denominator)
                .normalize_sign(dividend_denominator * divisor_numerator);
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDiv
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(&divisor.numerator);
            let (dividend_denominator, divisor_denominator) =
                self.denominator.normalize_moduli(&divisor.denominator);
            let (numerator, denominator) = (dividend_numerator
                * divisor_denominator)
                .normalize_sign(dividend_denominator * divisor_numerator);
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor);
            let (numerator, denominator) = dividend_numerator
                .normalize_sign(self.denominator * divisor_numerator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor);
            let (numerator, denominator) = dividend_numerator
                .normalize_sign(self.denominator * divisor_numerator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
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
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_div(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor);
            let (numerator, denominator) = dividend_numerator
                .normalize_sign(&self.denominator * divisor_numerator);
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
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
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_div(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor);
            let (numerator, denominator) = dividend_numerator
                .normalize_sign(&self.denominator * divisor_numerator);
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDiv<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Fraction<Self>: Zeroable,
    Self: Mul<Output = Self>
        + NormalizeModuli<Output = (Self, Self)>
        + NormalizeSign<Output = (Self, Self)>,
{
    type Output = Option<Fraction<Self>>;

    fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend, divisor_numerator) =
                self.normalize_moduli(divisor.numerator);
            let (numerator, denominator) = (dividend * divisor.denominator)
                .normalize_sign(divisor_numerator);
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDiv<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Fraction<Self>: Zeroable,
    for<'a> Self: Mul<&'a Self, Output = Self>
        + NormalizeModuli<&'a Self, Output = (Self, Self)>
        + NormalizeSign<Output = (Self, Self)>,
{
    type Output = Option<Fraction<Self>>;

    fn checked_div(self, divisor: &Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend, divisor_numerator) =
                self.normalize_moduli(&divisor.numerator);
            let (numerator, denominator) = (dividend * &divisor.denominator)
                .normalize_sign(divisor_numerator);
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
    Self: NormalizeModuli<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_div(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend, divisor_numerator) =
                self.normalize_moduli(divisor.numerator);
            let (numerator, denominator) = (dividend * divisor.denominator)
                .normalize_sign(divisor_numerator);
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDiv<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
        Output = (BigInt<Digit, DIGIT_BITNESS>, BigInt<Digit, DIGIT_BITNESS>),
    >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeSign<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_div(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend, divisor_numerator) =
                self.normalize_moduli(&divisor.numerator);
            let (numerator, denominator) = (dividend * &divisor.denominator)
                .normalize_sign(divisor_numerator);
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

macro_rules! integer_fraction_checked_div_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDiv for Fraction<$integer>
        where
            Self: Zeroable,
        {
            type Output = Option<Self>;

            fn checked_div(self, divisor: Self) -> Self::Output {
                if divisor == 0 {
                    None
                } else {
                    let (dividend_numerator, divisor_numerator) =
                        self.numerator.normalize_moduli(divisor.numerator);
                    let (dividend_denominator, divisor_denominator) =
                        self.denominator.normalize_moduli(divisor.denominator);
                    let (numerator, denominator) = (dividend_numerator
                        * divisor_denominator)
                        .normalize_sign(
                            dividend_denominator * divisor_numerator,
                        );
                    Some(Self {
                        numerator,
                        denominator,
                    })
                }
            }
        }

        impl CheckedDiv<$integer> for Fraction<$integer>
        where
            Self: Zeroable,
        {
            type Output = Option<Self>;

            fn checked_div(self, divisor: $integer) -> Self::Output {
                if divisor == 0 {
                    None
                } else {
                    let (dividend_numerator, divisor_numerator) =
                        self.numerator.normalize_moduli(divisor);
                    let (numerator, denominator) = dividend_numerator
                        .normalize_sign(self.denominator * divisor_numerator);
                    Some(Self {
                        numerator,
                        denominator,
                    })
                }
            }
        }

        impl CheckedDiv<Fraction<Self>> for $integer {
            type Output = Option<Fraction<Self>>;

            fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
                if divisor == 0 {
                    None
                } else {
                    let (dividend, divisor_numerator) =
                        self.normalize_moduli(divisor.numerator);
                    let (numerator, denominator) = (dividend
                        * divisor.denominator)
                        .normalize_sign(divisor_numerator);
                    Some(Fraction::<Self> {
                        numerator,
                        denominator,
                    })
                }
            }
        }
    )*)
}

integer_fraction_checked_div_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
