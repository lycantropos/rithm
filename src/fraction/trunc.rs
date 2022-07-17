use std::ops::{Mul, Neg};

use traiter::numbers::{
    Ceil, CheckedDivEuclid, Floor, Signed, Trunc, Unitary,
};

use super::types::Fraction;

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + Mul<Output = Component>
            + Neg<Output = Component>
            + Signed
            + Unitary,
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
