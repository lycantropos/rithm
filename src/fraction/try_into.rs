use std::convert::TryFrom;

use crate::traits::TryDivAsFloat;

use super::types::Fraction;

macro_rules! try_float_from_fraction {
    ($($float:ty)*) => ($(
        impl<Component: TryDivAsFloat<Component, $float>>
            TryFrom<Fraction<Component>> for $float
        {
            type Error = <Component as TryDivAsFloat<Component, $float>>::Error;

            fn try_from(
                value: Fraction<Component>,
            ) -> Result<$float, Self::Error> {
                value.numerator.try_div_as_float(value.denominator)
            }
        }
    )*)
}

try_float_from_fraction!(f32 f64);
