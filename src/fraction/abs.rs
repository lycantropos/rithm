use traiter::numbers::Abs;

use super::types::Fraction;

impl<Component: Abs<Output = Component>> Abs for Fraction<Component> {
    type Output = Self;

    fn abs(self) -> <Self as Abs>::Output {
        Self::Output {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}
