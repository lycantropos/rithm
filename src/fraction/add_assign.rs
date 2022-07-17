use std::ops::{Add, AddAssign, Div, Mul};

use traiter::numbers::{Gcd, Signed};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Add<Output = Component>
            + Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > AddAssign for Fraction<Component>
{
    fn add_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() * other.denominator.clone()
                + other.numerator * self.denominator.clone(),
            self.denominator.clone() * other.denominator,
        );
    }
}

impl<
        Component: Add<Output = Component>
            + Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > AddAssign<Component> for Fraction<Component>
{
    fn add_assign(&mut self, other: Component) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() + other * self.denominator.clone(),
            self.denominator.clone(),
        );
    }
}
