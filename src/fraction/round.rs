use std::cmp::Ordering;

use crate::traits::{
    AdditiveMonoid, CheckedDivRemEuclid, MultiplicativeMonoid, Parity, Round, Signed,
    SubtractiveMagma, TieBreaking,
};

use super::types::Fraction;

impl<
        Component: AdditiveMonoid
            + Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + From<u8>
            + MultiplicativeMonoid
            + Signed
            + Ord
            + Parity
            + SubtractiveMagma,
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
