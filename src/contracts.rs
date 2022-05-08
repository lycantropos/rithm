use crate::traits::{Oppose, OppositionOf};

pub(crate) const fn are_same<T, U>() -> bool {
    trait SameTo<U> {
        const VALUE: bool;
    }

    impl<T, U> SameTo<U> for T {
        default const VALUE: bool = false;
    }

    impl<T> SameTo<T> for T {
        const VALUE: bool = true;
    }

    <T as SameTo<U>>::VALUE
}

pub(crate) const fn is_signed<T: Oppose>() -> bool {
    are_same::<T, OppositionOf<T>>()
}

pub(crate) const fn is_unsigned<T: Oppose>() -> bool {
    !are_same::<T, OppositionOf<T>>()
}
