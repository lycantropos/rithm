use crate::traits::Unitary;

use super::types::Fraction;

impl<Component: Clone + Unitary> From<Component> for Fraction<Component> {
    fn from(value: Component) -> Self {
        Self {
            numerator: value,
            denominator: Component::one(),
        }
    }
}
