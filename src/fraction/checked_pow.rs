use std::ops::Neg;

use traiter::numbers::{CheckedPow, Signed, Zeroable};

use super::types::{Fraction, NormalizeSign};

impl<
        Component: Clone
            + CheckedPow<Component, Output = Option<Component>>
            + Signed
            + Neg<Output = Component>
            + NormalizeSign<Output = (Component, Component)>,
    > CheckedPow<Component> for Fraction<Component>
where
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = Component::normalize_sign(
                    self.denominator.checked_pow(exponent.clone())?,
                    self.numerator.checked_pow(exponent)?,
                );
                Some(Self {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Self {
                numerator: unsafe {
                    self.numerator
                        .checked_pow(exponent.clone())
                        .unwrap_unchecked()
                },
                denominator: unsafe {
                    self.denominator.checked_pow(exponent).unwrap_unchecked()
                },
            })
        }
    }
}

macro_rules! signed_integer_fraction_checked_pow_impl {
    ($($integer:ty)*) => ($(
        impl CheckedPow<u32> for Fraction<$integer> {
            type Output = Option<Self>;

            fn checked_pow(self, exponent: u32) -> Self::Output {
                Some(Self {
                    numerator: self.numerator.checked_pow(exponent)?,
                    denominator: self.denominator.checked_pow(exponent)?,
                })
            }
        }
    )*)
}

signed_integer_fraction_checked_pow_impl!(i8 i16 i32 i64 i128 isize);
