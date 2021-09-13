use std::fmt::{Display, Formatter};

use crate::traits::{
    Abs, AdditiveMonoid, CheckedDiv, CheckedPow, DivisivePartialMagma, GcdMagma,
    ModularUnaryAlgebra, MultiplicativeMonoid, NegatableUnaryAlgebra, Oppositive, Pow,
    SubtractiveMagma, Unitary, Zeroable,
};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Eq, PartialEq)]
pub struct Fraction<Component: Clone + Eq> {
    numerator: Component,
    denominator: Component,
}

impl<Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive> Fraction<Component> {
    pub fn new(mut numerator: Component, mut denominator: Component) -> Option<Self> {
        if denominator.is_zero() {
            None
        } else {
            (numerator, denominator) =
                normalize_components_sign::<Component>(numerator, denominator);
            (numerator, denominator) =
                normalize_components_moduli::<Component>(numerator, denominator);
            Some(Self {
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
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid,
    > AddAssign for Fraction<Component>
{
    fn add_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli::<Component>(
            self.numerator.clone() * other.denominator.clone()
                + other.numerator * self.denominator.clone(),
            self.denominator.clone() * other.denominator,
        );
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > CheckedDiv for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_div(self, other: Self) -> Self::Output {
        if other.is_zero() {
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

impl<
        Component: Clone
            + Eq
            + Oppositive
            + CheckedPow<Component, Output = Option<Component>>
            + Unitary
            + Zeroable,
    > CheckedPow<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = normalize_components_sign(
                    self.denominator.checked_pow(exponent.clone())?,
                    self.numerator.checked_pow(exponent)?,
                );
                Some(Self {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Self {
                numerator: self.numerator.checked_pow(exponent.clone())?,
                denominator: self.denominator.checked_pow(exponent)?,
            })
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

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > Div for Fraction<Component>
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.checked_div(other).unwrap()
    }
}

impl<Component: Clone + Eq + ModularUnaryAlgebra> Abs for Fraction<Component> {
    type Output = Self;

    fn abs(self) -> <Self as Abs>::Output {
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

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > MulAssign for Fraction<Component>
{
    fn mul_assign(&mut self, other: Self) {
        let (numerator, other_denominator) =
            normalize_components_moduli::<Component>(self.numerator.clone(), other.denominator);
        let (other_numerator, denominator) =
            normalize_components_moduli::<Component>(other.numerator, self.denominator.clone());
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
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
        Component: Clone + Eq + Oppositive + Pow<Component, Output = Component> + Unitary + Zeroable,
    > Pow<Component> for Fraction<Component>
{
    type Output = Self;

    fn pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                panic!("Division by zero is undefined.")
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = normalize_components_sign(
                    self.denominator.pow(exponent.clone()),
                    self.numerator.pow(exponent),
                );
                Self {
                    numerator,
                    denominator,
                }
            }
        } else {
            Self {
                numerator: self.numerator.pow(exponent.clone()),
                denominator: self.denominator.pow(exponent),
            }
        }
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

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > SubAssign for Fraction<Component>
{
    fn sub_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli::<Component>(
            self.numerator.clone() * other.denominator.clone()
                - other.numerator * self.denominator.clone(),
            self.denominator.clone() * other.denominator,
        );
    }
}

impl<Component: Clone + Eq + Unitary> Unitary for Fraction<Component> {
    fn one() -> Self {
        Self {
            numerator: Component::one(),
            denominator: Component::one(),
        }
    }

    fn is_one(&self) -> bool {
        self.numerator.is_one() && self.denominator.is_one()
    }
}

impl<Component: Clone + Eq + Unitary + Zeroable> Zeroable for Fraction<Component> {
    fn zero() -> Self {
        Self {
            numerator: Component::zero(),
            denominator: Component::one(),
        }
    }

    fn is_zero(&self) -> bool {
        self.numerator.is_zero()
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
