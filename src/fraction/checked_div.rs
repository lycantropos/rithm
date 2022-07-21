use std::ops::Mul;

use traiter::numbers::{CheckedDiv, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli, NormalizeSign};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_div(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDiv
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

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
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_div(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
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
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_div(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor);
            let (numerator, denominator) = dividend_numerator
                .normalize_sign(&self.denominator * divisor_numerator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
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
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_div(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend_numerator, divisor_numerator) =
                self.numerator.normalize_moduli(divisor);
            let (numerator, denominator) = dividend_numerator
                .normalize_sign(&self.denominator * divisor_numerator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<&Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
    Self: NormalizeModuli<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_div(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend, divisor_numerator) =
                self.normalize_moduli(divisor.numerator);
            let (numerator, denominator) = (dividend * divisor.denominator)
                .normalize_sign(divisor_numerator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
        Output = (
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        ),
    >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeSign<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_div(
        self,
        divisor: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (dividend, divisor_numerator) =
                self.normalize_moduli(&divisor.numerator);
            let (numerator, denominator) = (dividend * &divisor.denominator)
                .normalize_sign(divisor_numerator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
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
