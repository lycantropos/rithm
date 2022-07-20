use std::ops::{Add, AddAssign, Mul};

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Add<Output = Component>
            + Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > AddAssign for Fraction<Component>
{
    fn add_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = Component::normalize_moduli(
            self.numerator.clone() * other.denominator.clone()
                + other.numerator * self.denominator.clone(),
            self.denominator.clone() * other.denominator,
        );
    }
}

impl<
        Component: Add<Output = Component>
            + Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > AddAssign<Component> for Fraction<Component>
{
    fn add_assign(&mut self, other: Component) {
        (self.numerator, self.denominator) = Component::normalize_moduli(
            self.numerator.clone() + other * self.denominator.clone(),
            self.denominator.clone(),
        );
    }
}
