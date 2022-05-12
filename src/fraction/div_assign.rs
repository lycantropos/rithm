use std::ops::DivAssign;

use crate::traits::{DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Oppositive};

use super::types::{normalize_components_moduli, Fraction};

impl<Component: Clone + DivisivePartialMagma + GcdMagma + MultiplicativeMonoid + Oppositive>
    DivAssign for Fraction<Component>
{
    fn div_assign(&mut self, other: Self) {
        let (numerator, other_numerator) =
            normalize_components_moduli(self.numerator.clone(), other.numerator);
        let (denominator, other_denominator) =
            normalize_components_moduli(self.denominator.clone(), other.denominator);
        self.numerator = numerator * other_denominator;
        self.denominator = denominator * other_numerator;
    }
}

impl<Component: Clone + DivisivePartialMagma + GcdMagma + MultiplicativeMonoid + Oppositive>
    DivAssign<Component> for Fraction<Component>
{
    fn div_assign(&mut self, other: Component) {
        let (numerator, other) = normalize_components_moduli(self.numerator.clone(), other);
        self.numerator = numerator;
        self.denominator = self.denominator.clone() * other;
    }
}
