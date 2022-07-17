use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

use traiter::numbers::{
    CheckedDivRemEuclid, Parity, Round, Signed, TieBreaking, Unitary,
};

use super::types::Fraction;

impl<
        Component: Add<Output = Component>
            + Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + From<u8>
            + Mul<Output = Component>
            + Signed
            + Ord
            + Parity
            + Sub<Output = Component>
            + Unitary,
    > Round for Fraction<Component>
{
    type Output = Component;

    fn round(self, tie_breaking: TieBreaking) -> Self::Output {
        let (quotient, remainder) = unsafe {
            self.numerator
                .checked_div_rem_euclid(self.denominator.clone())
                .unwrap_unchecked()
        };
        match (remainder * Component::from(2u8)).cmp(&self.denominator) {
            Ordering::Equal => {
                if match tie_breaking {
                    TieBreaking::AwayFromZero => !quotient.is_negative(),
                    TieBreaking::ToEven => quotient.is_odd(),
                    TieBreaking::ToOdd => quotient.is_even(),
                    TieBreaking::TowardZero => quotient.is_negative(),
                } {
                    quotient + Component::one()
                } else {
                    quotient
                }
            }
            Ordering::Greater => quotient + Component::one(),
            Ordering::Less => quotient,
        }
    }
}
