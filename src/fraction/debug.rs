use std::fmt::{Debug, Formatter};

use crate::traits::Unitary;

use super::types::Fraction;

impl<Component: Clone + Debug + Unitary> Debug for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
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
