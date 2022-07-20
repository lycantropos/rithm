use std::ops::{Mul, Sub, SubAssign};

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + Sub<Output = Component>,
    > SubAssign for Fraction<Component>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.numerator, self.denominator) = Component::normalize_moduli(
            self.numerator.clone() * subtrahend.denominator.clone()
                - subtrahend.numerator * self.denominator.clone(),
            self.denominator.clone() * subtrahend.denominator,
        );
    }
}

impl<
        Component: Clone
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>
            + Sub<Output = Component>,
    > SubAssign<Component> for Fraction<Component>
{
    fn sub_assign(&mut self, subtrahend: Component) {
        (self.numerator, self.denominator) = Component::normalize_moduli(
            self.numerator.clone() - subtrahend * self.denominator.clone(),
            self.denominator.clone(),
        );
    }
}
