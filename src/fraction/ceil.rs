use traiter::numbers::{Ceil, CheckedDivEuclid, Zeroable};

use crate::traits::{MultiplicativeMonoid, NegatableUnaryAlgebra};

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + MultiplicativeMonoid
            + NegatableUnaryAlgebra
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
