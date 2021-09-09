use std::fmt::{Display, Formatter};
use std::ops::{Div, Neg};

use num::{One, Zero};

use crate::Gcd;

#[derive(Clone, PartialEq, Eq)]
pub struct Fraction<Component: Clone + PartialEq + Eq> {
    numerator: Component,
    denominator: Component,
}

impl<
        Component: Clone
            + Div<Output = Component>
            + Gcd<Output = Component>
            + Neg<Output = Component>
            + PartialEq
            + PartialOrd
            + Eq
            + Zero,
    > Fraction<Component>
{
    pub fn new(mut numerator: Component, mut denominator: Component) -> Result<Self, &'static str> {
        if denominator.is_zero() {
            Err("Denominator should not be zero.")
        } else {
            if denominator.lt(&Component::zero()) {
                (numerator, denominator) = (-numerator, -denominator);
            };
            let gcd = numerator.clone().gcd(denominator.clone());
            Ok(Self {
                numerator: numerator / gcd.clone(),
                denominator: denominator / gcd,
            })
        }
    }

    pub fn denominator(&self) -> &Component {
        &self.denominator
    }

    pub fn numerator(&self) -> &Component {
        &self.numerator
    }
}

impl<Component: Clone + Display + PartialEq + Eq + One> Display for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}
