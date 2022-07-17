use std::ops::{Div, Mul, MulAssign};

use traiter::numbers::{Gcd, Signed};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > MulAssign for Fraction<Component>
{
    fn mul_assign(&mut self, other: Self) {
        let (numerator, other_denominator) = normalize_components_moduli(
            self.numerator.clone(),
            other.denominator,
        );
        let (other_numerator, denominator) = normalize_components_moduli(
            other.numerator,
            self.denominator.clone(),
        );
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
    }
}

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Signed
            + Mul<Output = Component>,
    > MulAssign<Component> for Fraction<Component>
{
    fn mul_assign(&mut self, other: Component) {
        let (other, denominator) =
            normalize_components_moduli(other, self.denominator.clone());
        self.numerator = self.numerator.clone() * other;
        self.denominator = denominator;
    }
}
