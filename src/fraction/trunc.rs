use crate::traits::{Ceil, CheckedDivEuclid, Floor, MultiplicativeMonoid, Signed, Trunc};

use super::types::Fraction;

impl<
        Component: Clone + CheckedDivEuclid<Output = Option<Component>> + MultiplicativeMonoid + Signed,
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
