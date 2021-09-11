use std::fmt::{Display, Formatter};

use crate::traits::{
    AdditiveMonoid, DivisivePartialMagma, GcdMagma, Modular, ModularUnaryAlgebra,
    MultiplicativeMonoid, NegatableUnaryAlgebra, Oppositive, SubtractiveMagma, Unitary,
};
use std::cmp::Ordering;
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Eq, PartialEq)]
pub struct Fraction<Component: Clone + Eq> {
    numerator: Component,
    denominator: Component,
}

impl<Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive> Fraction<Component> {
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

impl<
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid,
    > Add for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        unsafe {
            Self::new(
                self.numerator * other.denominator.clone()
                    + other.numerator * self.denominator.clone(),
                self.denominator * other.denominator,
            )
            .unwrap_unchecked()
        }
    }
}

impl<Component: Clone + Display + Eq + Unitary> Display for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl<Component: Clone + Eq + ModularUnaryAlgebra> Modular for Fraction<Component> {
    type Output = Self;

    fn abs(self) -> <Self as Modular>::Output {
        Self {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}

impl<Component: Clone + Eq + NegatableUnaryAlgebra> Neg for Fraction<Component> {
    type Output = Self;

    fn neg(self) -> <Self as Neg>::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
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

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > Sub for Fraction<Component>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli::<Component>(
            self.numerator * other.denominator.clone() - other.numerator * self.denominator.clone(),
            self.denominator * other.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

fn normalize_components_moduli<Component: Clone + DivisivePartialMagma + GcdMagma>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    let gcd = numerator.clone().gcd(denominator.clone());
    (numerator / gcd.clone(), denominator / gcd)
}
