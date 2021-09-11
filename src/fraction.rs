use std::fmt::{Display, Formatter};

use crate::traits::{
    AdditiveMonoid, CheckedDiv, DivisivePartialMagma, GcdMagma, Modular, ModularUnaryAlgebra,
    MultiplicativeMonoid, NegatableUnaryAlgebra, Oppositive, SubtractiveMagma, Unitary,
};
use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};

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
            (numerator, denominator) =
                normalize_components_sign::<Component>(numerator, denominator);
            (numerator, denominator) =
                normalize_components_moduli::<Component>(numerator, denominator);
            Ok(Self {
                numerator,
                denominator,
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
        let (numerator, denominator) = normalize_components_moduli::<Component>(
            self.numerator * other.denominator.clone() + other.numerator * self.denominator.clone(),
            self.denominator * other.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > CheckedDiv for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_div(self, other: Self) -> Self::Output {
        if other.numerator.is_zero() {
            return None;
        }
        let (numerator, other_numerator) =
            normalize_components_moduli::<Component>(self.numerator, other.numerator);
        let (denominator, other_denominator) =
            normalize_components_moduli::<Component>(self.denominator, other.denominator);
        let (result_numerator, result_denominator) = normalize_components_sign::<Component>(
            numerator * other_denominator,
            denominator * other_numerator,
        );
        Some(Self {
            numerator: result_numerator,
            denominator: result_denominator,
        })
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

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > Mul for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (numerator, other_denominator) =
            normalize_components_moduli::<Component>(self.numerator, other.denominator);
        let (other_numerator, denominator) =
            normalize_components_moduli::<Component>(other.numerator, self.denominator);
        Self {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
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

#[inline]
fn normalize_components_moduli<Component: Clone + DivisivePartialMagma + GcdMagma>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    let gcd = numerator.clone().gcd(denominator.clone());
    (numerator / gcd.clone(), denominator / gcd)
}

#[inline]
fn normalize_components_sign<Component: Oppositive>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    if denominator.is_negative() {
        (-numerator, -denominator)
    } else {
        (numerator, denominator)
    }
}
