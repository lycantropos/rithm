use crate::traits::{Oppositive, Unitary};

use super::types::Fraction;

impl<Component: Clone + Eq + Oppositive + Unitary> Oppositive for Fraction<Component> {
    fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.numerator.is_positive()
    }
}
