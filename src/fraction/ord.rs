use std::cmp::Ordering;

use crate::traits::MultiplicativeMonoid;

use super::types::Fraction;

impl<Component: Clone + Eq + MultiplicativeMonoid + PartialOrd> Ord for Fraction<Component> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
