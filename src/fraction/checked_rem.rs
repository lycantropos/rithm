use crate::big_int::{BigInt, DivisibleDigit, GcdDigit, MultiplicativeDigit};
use crate::traits::{CheckedRem, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Zeroable};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + DivisivePartialMagma
            + GcdMagma
            + MultiplicativeMonoid
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
            + DivisivePartialMagma
            + GcdMagma
            + MultiplicativeMonoid
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

impl<
        Digit: DivisibleDigit + GcdDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRem<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Fraction<Self>>;

    fn checked_rem(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = normalize_components_moduli(
                (self * divisor.denominator.clone()).checked_rem(divisor.numerator)?,
                divisor.denominator,
            );
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

macro_rules! primitive_checked_rem_fraction_impl {
    ($($t:ty)*) => ($(
        impl CheckedRem<Fraction<Self>> for $t
        {
            type Output = Option<Fraction<Self>>;

            fn checked_rem(self, divisor: Fraction<Self>) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = normalize_components_moduli(
                        (self * divisor.denominator).checked_rem(divisor.numerator)?,
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

primitive_checked_rem_fraction_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
