use traiter::numbers::{Sign, Signed, Unitary};

use super::types::Fraction;

impl<Component: Clone + Signed + Unitary> Signed for Fraction<Component> {
    fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.numerator.is_positive()
    }

    fn sign(&self) -> Sign {
        self.numerator.sign()
    }
}
