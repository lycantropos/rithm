use traiter::numbers::Unitary;

use super::types::Fraction;

impl<Component: Unitary> From<Component> for Fraction<Component> {
    fn from(value: Component) -> Self {
        Self {
            numerator: value,
            denominator: Component::one(),
        }
    }
}
