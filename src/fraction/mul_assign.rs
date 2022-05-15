use std::ops::MulAssign;

use crate::traits::{DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed};

use super::types::{normalize_components_moduli, Fraction};

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid> MulAssign
    for Fraction<Component>
{
    fn mul_assign(&mut self, other: Self) {
        let (numerator, other_denominator) =
            normalize_components_moduli(self.numerator.clone(), other.denominator);
        let (other_numerator, denominator) =
            normalize_components_moduli(other.numerator, self.denominator.clone());
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
    }
}

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid>
    MulAssign<Component> for Fraction<Component>
{
    fn mul_assign(&mut self, other: Component) {
        let (other, denominator) = normalize_components_moduli(other, self.denominator.clone());
        self.numerator = self.numerator.clone() * other;
        self.denominator = denominator;
    }
}
