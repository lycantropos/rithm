use traiter::numbers::{Unitary, Zeroable};

use super::types::Fraction;

impl<Component: Unitary + Zeroable> Zeroable for Fraction<Component> {
    fn zero() -> Self {
        Self {
            numerator: Component::zero(),
            denominator: Component::one(),
        }
    }

    fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
}
