use crate::traits::{Abs, ModularUnaryAlgebra};

use super::types::Fraction;

impl<Component: Clone + ModularUnaryAlgebra> Abs for Fraction<Component> {
    type Output = Self;

    fn abs(self) -> <Self as Abs>::Output {
        Self::Output {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}
