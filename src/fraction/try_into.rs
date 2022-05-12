use std::convert::TryFrom;

use crate::traits::{CheckedDivAsF32, CheckedDivAsF64, Maybe};

use super::types::Fraction;

impl<Component: Clone + CheckedDivAsF32> TryFrom<Fraction<Component>> for f32 {
    type Error = <<Component as CheckedDivAsF32>::Output as Maybe>::Error;

    fn try_from(value: Fraction<Component>) -> Result<f32, Self::Error> {
        let maybe = value.numerator.checked_div_as_f32(value.denominator);
        if maybe.is_result() {
            Ok(maybe.result())
        } else {
            Err(maybe.error())
        }
    }
}

impl<Component: Clone + CheckedDivAsF64> TryFrom<Fraction<Component>> for f64 {
    type Error = <<Component as CheckedDivAsF64>::Output as Maybe>::Error;

    fn try_from(value: Fraction<Component>) -> Result<f64, Self::Error> {
        let maybe = value.numerator.checked_div_as_f64(value.denominator);
        if maybe.is_result() {
            Ok(maybe.result())
        } else {
            Err(maybe.error())
        }
    }
}
