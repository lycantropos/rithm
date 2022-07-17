use std::ops::{Div, Mul, Sub, SubAssign};

use traiter::numbers::{Gcd, Signed};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed
            + Sub<Output = Component>,
    > SubAssign for Fraction<Component>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() * subtrahend.denominator.clone()
                - subtrahend.numerator * self.denominator.clone(),
            self.denominator.clone() * subtrahend.denominator,
        );
    }
}

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>
            + Sub<Output = Component>,
    > SubAssign<Component> for Fraction<Component>
{
    fn sub_assign(&mut self, subtrahend: Component) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() - subtrahend * self.denominator.clone(),
            self.denominator.clone(),
        );
    }
}
