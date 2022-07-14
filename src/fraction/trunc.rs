use traiter::numbers::{Ceil, CheckedDivEuclid, Floor, Signed, Trunc};

use crate::traits::{MultiplicativeMonoid, NegatableUnaryAlgebra};

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + MultiplicativeMonoid
            + NegatableUnaryAlgebra
            + Signed,
    > Trunc for Fraction<Component>
{
    type Output = Component;

    fn trunc(self) -> Self::Output {
        if self.is_negative() {
            self.ceil()
        } else {
            self.floor()
        }
    }
}
