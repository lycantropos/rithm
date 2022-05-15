use std::fmt;

use crate::traits::{DivisivePartialMagma, GcdMagma, Signed};

#[derive(Clone)]
pub struct Fraction<Component: Clone> {
    pub(super) numerator: Component,
    pub(super) denominator: Component,
}

impl<Component: Clone + DivisivePartialMagma + GcdMagma + Signed> Fraction<Component> {
    pub fn new(mut numerator: Component, mut denominator: Component) -> Option<Self> {
        if denominator.is_zero() {
            None
        } else {
            (numerator, denominator) = normalize_components_sign(numerator, denominator);
            (numerator, denominator) = normalize_components_moduli(numerator, denominator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Component: Clone> Fraction<Component> {
    pub fn denominator(&self) -> &Component {
        &self.denominator
    }

    pub fn numerator(&self) -> &Component {
        &self.numerator
    }
}

#[inline]
pub(super) fn normalize_components_moduli<Component: Clone + DivisivePartialMagma + GcdMagma>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    let gcd = numerator.clone().gcd(denominator.clone());
    (numerator / gcd.clone(), denominator / gcd)
}

#[inline]
pub(super) fn normalize_components_sign<Component: Signed>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    if denominator.is_negative() {
        (-numerator, -denominator)
    } else {
        (numerator, denominator)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FromFloatConversionError {
    Infinity,
    NaN,
    OutOfBounds,
}

impl FromFloatConversionError {
    fn description(&self) -> &str {
        match self {
            FromFloatConversionError::Infinity => "Conversion of infinity is undefined.",
            FromFloatConversionError::NaN => "Conversion of NaN is undefined.",
            FromFloatConversionError::OutOfBounds => "Value is out of bounds.",
        }
    }
}

impl fmt::Debug for FromFloatConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl fmt::Display for FromFloatConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.description(), formatter)
    }
}
