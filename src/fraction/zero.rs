use traiter::numbers::{One, Zero};

use super::types::Fraction;

impl<Component: One + Zero> Zero for Fraction<Component> {
    fn zero() -> Self {
        Self {
            numerator: Component::zero(),
            denominator: Component::one(),
        }
    }
}
