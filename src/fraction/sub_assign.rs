use std::ops::SubAssign;

use crate::traits::{
    DivisivePartialMagma, GcdMagma, MultiplicativeMonoid, Oppositive, SubtractiveMagma,
};

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
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
