use crate::traits::Unitary;

use super::types::Fraction;

impl<Component: Clone + Unitary> Unitary for Fraction<Component> {
    fn one() -> Self {
        Self {
            numerator: Component::one(),
            denominator: Component::one(),
        }
    }

    fn is_one(&self) -> bool {
        self.numerator.is_one() && self.denominator.is_one()
    }
}
