use std::cmp::Ordering;
use std::ops::Mul;

use super::types::Fraction;

impl<Component: Clone + Eq + Mul<Output = Component> + Ord> Ord
    for Fraction<Component>
{
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}
