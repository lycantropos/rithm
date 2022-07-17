use std::ops::{Div, Mul};

use traiter::numbers::{CheckedDivRemEuclid, Gcd, Zeroable};

use crate::big_int::BigInt;

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Zeroable,
    > CheckedDivRemEuclid for Fraction<Component>
{
    type Output = Option<(Component, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        let (quotient, remainder_numerator) = (self.numerator
            * divisor.denominator.clone())
        .checked_div_rem_euclid(
            divisor.numerator * self.denominator.clone(),
        )?;
        let (remainder_numerator, remainder_denominator) =
            normalize_components_moduli(
                remainder_numerator,
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

impl<
        Component: Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Zeroable,
    > CheckedDivRemEuclid<Component> for Fraction<Component>
{
    type Output = Option<(Component, Self)>;

    fn checked_div_rem_euclid(self, divisor: Component) -> Self::Output {
        let (quotient, remainder_numerator) = (self.numerator)
            .checked_div_rem_euclid(divisor * self.denominator.clone())?;
        let (remainder_numerator, remainder_denominator) =
            normalize_components_moduli(remainder_numerator, self.denominator);
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
    CheckedDivRemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivRemEuclid<Self, Output = Option<(Self, Self)>>
        + Clone
        + Div<Output = Self>
        + Gcd<Output = Self>
        + Mul<Output = Self>,
{
    type Output = Option<(Self, Fraction<Self>)>;

    fn checked_div_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        let (quotient, remainder_numerator) = (self
            * divisor.denominator.clone())
        .checked_div_rem_euclid(divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            normalize_components_moduli(
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

macro_rules! integer_checked_div_rem_euclid_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDivRemEuclid<Fraction<Self>> for $integer {
            type Output = Option<(Self, Fraction<Self>)>;

            fn checked_div_rem_euclid(
                self,
                divisor: Fraction<Self>,
            ) -> Self::Output {
                let (quotient, remainder_numerator) = (self
                    * divisor.denominator.clone())
                .checked_div_rem_euclid(divisor.numerator)?;
                let (remainder_numerator, remainder_denominator) =
                    normalize_components_moduli(
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
