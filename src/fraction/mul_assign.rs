use std::ops::{Mul, MulAssign};

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > MulAssign for Fraction<Component>
{
    fn mul_assign(&mut self, other: Self) {
        let (numerator, other_denominator) = Component::normalize_moduli(
            self.numerator.clone(),
            other.denominator,
        );
        let (other_numerator, denominator) = Component::normalize_moduli(
            other.numerator,
            self.denominator.clone(),
        );
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
    }
}

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > MulAssign<Component> for Fraction<Component>
{
    fn mul_assign(&mut self, other: Component) {
        let (other, denominator) =
            Component::normalize_moduli(other, self.denominator.clone());
        self.numerator = self.numerator.clone() * other;
        self.denominator = denominator;
    }
}
