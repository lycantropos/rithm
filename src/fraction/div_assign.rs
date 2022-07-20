use std::ops::{DivAssign, Mul};

use super::types::{Fraction, NormalizeModuli, NormalizeSign};

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + NormalizeSign<Output = (Component, Component)>,
    > DivAssign for Fraction<Component>
{
    fn div_assign(&mut self, divisor: Self) {
        let (numerator, divisor_numerator) = Component::normalize_moduli(
            self.numerator.clone(),
            divisor.numerator,
        );
        let (denominator, divisor_denominator) = Component::normalize_moduli(
            self.denominator.clone(),
            divisor.denominator,
        );
        (self.numerator, self.denominator) = Component::normalize_sign(
            numerator * divisor_denominator,
            denominator * divisor_numerator,
        );
    }
}

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + NormalizeSign<Output = (Component, Component)>,
    > DivAssign<Component> for Fraction<Component>
{
    fn div_assign(&mut self, divisor: Component) {
        let (numerator, divisor) =
            Component::normalize_moduli(self.numerator.clone(), divisor);
        (self.numerator, self.denominator) = Component::normalize_sign(
            numerator,
            self.denominator.clone() * divisor,
        );
    }
}
