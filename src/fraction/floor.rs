use crate::traits::{CheckedDivEuclid, Floor, MultiplicativeMonoid, Zeroable};

use super::types::Fraction;

impl<
        Component: Clone + CheckedDivEuclid<Output = Option<Component>> + MultiplicativeMonoid + Zeroable,
    > Floor for Fraction<Component>
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
