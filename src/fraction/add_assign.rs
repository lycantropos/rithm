use std::ops::AddAssign;

use traiter::numbers::Signed;

use crate::traits::{
    AdditiveMonoid, DivisivePartialMagma, GcdMagma, MultiplicativeMonoid,
};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + GcdMagma
            + MultiplicativeMonoid
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
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + GcdMagma
            + MultiplicativeMonoid
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
