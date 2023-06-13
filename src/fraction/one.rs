use traiter::numbers::One;

use super::types::Fraction;

impl<Component: One> One for Fraction<Component> {
    fn one() -> Self {
        Self {
            numerator: Component::one(),
            denominator: Component::one(),
        }
    }
}
