use std::ops::Neg;

use super::types::Fraction;

impl<Component: Neg<Output = Component>> Neg for Fraction<Component> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}
