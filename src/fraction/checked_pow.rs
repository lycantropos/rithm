use crate::traits::{CheckedPow, Signed, Unitary, Zeroable};

use super::types::{normalize_components_sign, Fraction};

impl<
        Component: Clone + Signed + CheckedPow<Component, Output = Option<Component>> + Unitary + Zeroable,
    > CheckedPow<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = normalize_components_sign(
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
                denominator: unsafe { self.denominator.checked_pow(exponent).unwrap_unchecked() },
            })
        }
    }
}

macro_rules! primitive_fraction_checked_pow_impl {
    ($($t:ty)*) => ($(
        impl CheckedPow<u32> for Fraction<$t> {
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

primitive_fraction_checked_pow_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
