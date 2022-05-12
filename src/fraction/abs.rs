use crate::traits::{Abs, ModularUnaryAlgebra};

use super::types::Fraction;

impl<Component: Clone + ModularUnaryAlgebra> Abs for Fraction<Component> {
    type Output = Self;

    fn abs(self) -> <Self as Abs>::Output {
        Self {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}
