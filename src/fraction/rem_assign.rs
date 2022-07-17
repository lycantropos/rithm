use std::ops::{Div, Mul, RemAssign};

use traiter::numbers::{CheckedRem, Gcd, Signed};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::{normalize_components_moduli, Fraction};

impl<
        Component: Clone
            + CheckedRem<Output = Option<Component>>
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > RemAssign for Fraction<Component>
{
    fn rem_assign(&mut self, divisor: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli(
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
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Mul<Output = Component>
            + Signed,
    > RemAssign<Component> for Fraction<Component>
{
    fn rem_assign(&mut self, divisor: Component) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator
                .clone()
                .checked_rem(divisor * self.denominator.clone())
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE),
            self.denominator.clone(),
        );
    }
}
