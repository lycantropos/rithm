use std::hash::{Hash, Hasher};

use super::types::Fraction;

impl<Component: Clone + Hash> Hash for Fraction<Component> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator.hash(state);
        self.denominator.hash(state);
    }
}
