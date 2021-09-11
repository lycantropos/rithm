use std::fmt::{Display, Formatter};

use crate::traits::{DivisivePartialMagma, Gcd, MultiplicativeMonoid, One, Oppositive};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
pub struct Fraction<Component: Clone + Eq> {
    numerator: Component,
    denominator: Component,
}

impl<
        Component: Clone
            + DivisivePartialMagma
            + Gcd<Output = Component>
            + Oppositive
            + Eq,
    > Fraction<Component>
{
    pub fn new(mut numerator: Component, mut denominator: Component) -> Result<Self, &'static str> {
        if denominator.is_zero() {
            Err("Denominator should not be zero.")
        } else {
            if denominator.is_negative() {
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

impl<Component: Clone + Display + Eq + One> Display for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl<Component: Clone + Eq + MultiplicativeMonoid + PartialOrd> PartialOrd for Fraction<Component> {
    fn ge(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            >= other.numerator.clone() * self.denominator.clone()
    }

    fn gt(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            > other.numerator.clone() * self.denominator.clone()
    }

    fn le(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            <= other.numerator.clone() * self.denominator.clone()
    }

    fn lt(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            < other.numerator.clone() * self.denominator.clone()
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if other.lt(self) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}
