use std::ops::{Div, Mul};

use traiter::numbers::{CheckedRem, Gcd, Unitary, Zeroable};

use crate::big_int::BigInt;

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Zeroable,
    > CheckedRem for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            (self.numerator * divisor.denominator.clone())
                .checked_rem(divisor.numerator * self.denominator.clone())?,
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
            + CheckedRem<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Zeroable,
    > CheckedRem<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator
                .checked_rem(divisor * self.denominator.clone())?,
            self.denominator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: Clone
        + CheckedRem<Output = Option<Self>>
        + Div<Output = Self>
        + Gcd<Output = Self>
        + Mul<Output = Self>
        + Unitary
        + Zeroable,
{
    type Output = Option<Fraction<Self>>;

    fn checked_rem(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = normalize_components_moduli(
                (self * divisor.denominator.clone())
                    .checked_rem(divisor.numerator)?,
                divisor.denominator,
            );
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

macro_rules! integer_checked_rem_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedRem<Fraction<Self>> for $integer {
            type Output = Option<Fraction<Self>>;

            fn checked_rem(self, divisor: Fraction<Self>) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = normalize_components_moduli(
                        (self * divisor.denominator)
                            .checked_rem(divisor.numerator)?,
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

integer_checked_rem_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
