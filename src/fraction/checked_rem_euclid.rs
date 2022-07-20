use std::ops::Mul;

use traiter::numbers::{CheckedRemEuclid, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > CheckedRemEuclid for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        let (numerator, denominator) = Component::normalize_moduli(
            (self.numerator * divisor.denominator.clone())
                .checked_rem_euclid(
                    divisor.numerator * self.denominator.clone(),
                )?,
            self.denominator * divisor.denominator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > CheckedRemEuclid<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Component) -> Self::Output {
        let (numerator, denominator) = Component::normalize_moduli(
            self.numerator
                .checked_rem_euclid(divisor * self.denominator.clone())?,
            self.denominator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRemEuclid<Output = Option<Self>>
        + Clone
        + Mul<Output = Self>
        + NormalizeModuli<Output = (Self, Self)>,
    Fraction<Self>: Zeroable,
{
    type Output = Option<Fraction<Self>>;

    fn checked_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = Self::normalize_moduli(
                (self * divisor.denominator.clone())
                    .checked_rem_euclid(divisor.numerator)?,
                divisor.denominator,
            );
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

macro_rules! integer_checked_rem_euclid_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedRemEuclid<Fraction<Self>> for $integer {
            type Output = Option<Fraction<Self>>;

            fn checked_rem_euclid(
                self,
                divisor: Fraction<Self>,
            ) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = <$integer>::normalize_moduli(
                        (self * divisor.denominator)
                            .checked_rem_euclid(divisor.numerator)?,
                        divisor.denominator,
                    );
                    Some(Fraction::<Self> {
                        numerator,
                        denominator,
                    })
                }
            }
        }
    )*)
}

integer_checked_rem_euclid_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
