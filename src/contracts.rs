use crate::traits::HasSignBit;

pub(crate) const fn is_signed<T: HasSignBit>() -> bool {
    T::RESULT
}

pub(crate) const fn is_unsigned<T: HasSignBit>() -> bool {
    !T::RESULT
}
