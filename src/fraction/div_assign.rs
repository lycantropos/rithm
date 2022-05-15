use std::ops::DivAssign;

use crate::traits::{DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed};

use super::types::{normalize_components_moduli, normalize_components_sign, Fraction};

impl<Component: Clone + DivisivePartialMagma + GcdMagma + MultiplicativeMonoid + Signed> DivAssign
    for Fraction<Component>
{
    fn div_assign(&mut self, divisor: Self) {
        let (numerator, divisor_numerator) =
            normalize_components_moduli(self.numerator.clone(), divisor.numerator);
        let (denominator, divisor_denominator) =
            normalize_components_moduli(self.denominator.clone(), divisor.denominator);
        (self.numerator, self.denominator) = normalize_components_sign(
            numerator * divisor_denominator,
            denominator * divisor_numerator,
        );
    }
}

impl<Component: Clone + DivisivePartialMagma + GcdMagma + MultiplicativeMonoid + Signed>
    DivAssign<Component> for Fraction<Component>
{
    fn div_assign(&mut self, divisor: Component) {
        let (numerator, divisor) = normalize_components_moduli(self.numerator.clone(), divisor);
        (self.numerator, self.denominator) =
            normalize_components_sign(numerator, self.denominator.clone() * divisor);
    }
}
