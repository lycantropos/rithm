use traiter::numbers::Unitary;

use super::types::Fraction;

impl<Component: Unitary> Unitary for Fraction<Component> {
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
