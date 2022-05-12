use std::fmt::{Display, Formatter};

use crate::traits::Unitary;

use super::types::Fraction;

impl<Component: Clone + Display + Unitary> Display for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}
