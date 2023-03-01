use traiter::numbers::{Sign, Signed, Zeroable};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Signed
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: Signed,
    Self: Zeroable,
{
    fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.numerator.is_positive()
    }

    fn sign(&self) -> Sign {
        self.numerator.sign()
    }
}

macro_rules! signed_integer_fraction_signed_impl {
    ($($integer:ty)*) => ($(
        impl Signed for Fraction<$integer> {
            fn is_negative(&self) -> bool {
                self.numerator.is_negative()
            }

            fn is_positive(&self) -> bool {
                self.numerator.is_positive()
            }

            fn sign(&self) -> Sign {
                self.numerator.sign()
            }
        }
    )*)
}

signed_integer_fraction_signed_impl!(i8 i16 i32 i64 i128 isize);
