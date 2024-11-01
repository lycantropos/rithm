use std::fmt::{Display, Formatter};

use traiter::numbers::Unitary;

use super::types::Fraction;

impl<Component: Display> Display for Fraction<Component>
where
    for<'a> &'a Component: Unitary,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}
