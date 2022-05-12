use crate::big_int::{BigInt, EuclidDivisibleDigit, GcdDigit, MultiplicativeDigit};
use crate::traits::{
    CheckedDivRemEuclid, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Zeroable,
};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + DivisivePartialMagma
            + GcdMagma
            + MultiplicativeMonoid
            + Zeroable,
    > CheckedDivRemEuclid for Fraction<Component>
{
    type Output = Option<(Component, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        let (quotient, remainder_numerator) = (self.numerator * divisor.denominator.clone())
            .checked_div_rem_euclid(divisor.numerator * self.denominator.clone())?;
        let (remainder_numerator, remainder_denominator) = normalize_components_moduli(
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
            + DivisivePartialMagma
            + GcdMagma
            + MultiplicativeMonoid
            + Zeroable,
    > CheckedDivRemEuclid<Component> for Fraction<Component>
{
    type Output = Option<(Component, Self)>;

    fn checked_div_rem_euclid(self, divisor: Component) -> Self::Output {
        let (quotient, remainder_numerator) =
            (self.numerator).checked_div_rem_euclid(divisor * self.denominator.clone())?;
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

impl<
        Digit: EuclidDivisibleDigit + GcdDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivRemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Fraction<Self>)>;

    fn checked_div_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        let (quotient, remainder_numerator) =
            (self * divisor.denominator.clone()).checked_div_rem_euclid(divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            normalize_components_moduli(remainder_numerator, divisor.denominator);
        Some((
            quotient,
            Fraction::<Self> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

macro_rules! primitive_checked_div_rem_euclid_fraction_impl {
    ($($t:ty)*) => ($(
    impl CheckedDivRemEuclid<Fraction<Self>> for $t
    {
        type Output = Option<(Self, Fraction<Self>)>;

        fn checked_div_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
            let (quotient, remainder_numerator) = (self * divisor.denominator.clone())
                .checked_div_rem_euclid(divisor.numerator)?;
            let (remainder_numerator, remainder_denominator) = normalize_components_moduli(
                remainder_numerator, divisor.denominator,
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

primitive_checked_div_rem_euclid_fraction_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
