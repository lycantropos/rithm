use super::types::Fraction;

impl<Component: Eq> Eq for Fraction<Component> where Self: PartialEq {}
