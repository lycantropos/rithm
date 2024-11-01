use std::fmt::{Debug, Formatter};

use traiter::numbers::Unitary;

use super::types::Fraction;

impl<Component: Debug> Debug for Fraction<Component>
where
    for<'a> &'a Component: Unitary,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(
                formatter,
                "{}::from({:?})",
                std::any::type_name::<Self>(),
                self.numerator
            )
        } else {
            write!(
                formatter,
                "{}::new({:?}, {:?}).unwrap()",
                std::any::type_name::<Self>(),
                self.numerator,
                self.denominator
            )
        }
    }
}
