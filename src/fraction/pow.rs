use crate::traits::{Oppositive, Pow, Unitary, Zeroable};

use super::types::{normalize_components_sign, Fraction};

impl<
        Component: Clone + Eq + Oppositive + Pow<Component, Output = Component> + Unitary + Zeroable,
    > Pow<Component> for Fraction<Component>
{
    type Output = Self;

    fn pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                panic!("Division by zero is undefined.")
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = normalize_components_sign(
                    self.denominator.pow(exponent.clone()),
                    self.numerator.pow(exponent),
                );
                Self {
                    numerator,
                    denominator,
                }
            }
        } else {
            Self {
                numerator: self.numerator.pow(exponent.clone()),
                denominator: self.denominator.pow(exponent),
            }
        }
    }
}
