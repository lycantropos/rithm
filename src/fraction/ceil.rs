use std::ops::Neg;

use traiter::numbers::{Ceil, CheckedDivEuclid, Zeroable};

use super::types::Fraction;

impl<
        Component: CheckedDivEuclid<Output = Option<Component>>
            + Neg<Output = Component>
            + Zeroable,
    > Ceil for Fraction<Component>
{
    type Output = Component;

    fn ceil(self) -> Self::Output {
        -unsafe {
            (-self.numerator)
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}
