use traiter::numbers::{CheckedDivEuclid, Floor};

use super::types::Fraction;

impl<Component: CheckedDivEuclid<Output = Option<Component>>> Floor
    for Fraction<Component>
{
    type Output = Component;

    fn floor(self) -> Self::Output {
        unsafe {
            self.numerator
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}
