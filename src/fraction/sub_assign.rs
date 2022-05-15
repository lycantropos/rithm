use std::ops::SubAssign;

use crate::traits::{
    DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Signed, SubtractiveMagma,
};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid + SubtractiveMagma,
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
        Component: Clone + DivisivePartialMagma + GcdMagma + Signed + MultiplicativeMonoid + SubtractiveMagma,
    > SubAssign<Component> for Fraction<Component>
{
    fn sub_assign(&mut self, subtrahend: Component) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() - subtrahend * self.denominator.clone(),
            self.denominator.clone(),
        );
    }
}
