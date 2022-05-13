use std::ops::Neg;

use crate::traits::NegatableUnaryAlgebra;

use super::types::Fraction;

impl<Component: Clone + NegatableUnaryAlgebra> Neg for Fraction<Component> {
    type Output = Self;

    fn neg(self) -> <Self as Neg>::Output {
        Self::Output {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}
