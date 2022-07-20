use std::ops::{Mul, RemAssign};

use traiter::numbers::CheckedRem;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::{Fraction, NormalizeModuli};

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > RemAssign for Fraction<Component>
{
    fn rem_assign(&mut self, divisor: Self) {
        (self.numerator, self.denominator) = Component::normalize_moduli(
            (self.numerator.clone() * divisor.denominator.clone())
                .checked_rem(divisor.numerator * self.denominator.clone())
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE),
            self.denominator.clone() * divisor.denominator,
        );
    }
}

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + Mul<Output = Component>
            + NormalizeModuli<Output = (Component, Component)>,
    > RemAssign<Component> for Fraction<Component>
{
    fn rem_assign(&mut self, divisor: Component) {
        (self.numerator, self.denominator) = Component::normalize_moduli(
            self.numerator
                .clone()
                .checked_rem(divisor * self.denominator.clone())
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE),
            self.denominator.clone(),
        );
    }
}
