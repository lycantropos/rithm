use traiter::numbers::{CheckedRemEuclid, RemEuclid};

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRemEuclid<Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedRemEuclid<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    RemEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRemEuclid<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRemEuclid<
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem_euclid(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self:
        CheckedRemEuclid<BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    RemEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedRemEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Self>,
    >,
{
    type Output = Self;

    fn rem_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRemEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    RemEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedRemEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRemEuclid<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> RemEuclid<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self:
        CheckedRemEuclid<&'a Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem_euclid(self, divisor: &Fraction<Self>) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    RemEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRemEuclid<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    RemEuclid<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedRemEuclid<
        &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem_euclid(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_rem_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! integer_fraction_rem_euclid_impl {
    ($($integer:ty)*) => ($(
        impl RemEuclid for Fraction<$integer> {
            type Output = Self;

            fn rem_euclid(self, divisor: Self) -> Self::Output {
                self.checked_rem_euclid(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl RemEuclid<$integer> for Fraction<$integer> {
            type Output = Self;

            fn rem_euclid(self, divisor: $integer) -> Self::Output {
                self.checked_rem_euclid(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl RemEuclid<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedRemEuclid<Fraction<Self>>>::checked_rem_euclid(
                    self, divisor,
                )
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

integer_fraction_rem_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
