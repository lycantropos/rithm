use std::ops::Rem;

use traiter::numbers::CheckedRem;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Rem
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRem<Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedRem<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: &Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Rem<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRem<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRem<Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRem<BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self:
        CheckedRem<&'a BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedRem<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedRem<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem(self, divisor: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRem<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Rem<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self:
        CheckedRem<&'a Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem(self, divisor: &Fraction<Self>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Rem<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedRem<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Rem<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedRem<
        &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn rem(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! integer_fraction_rem_impl {
    ($($integer:ty)*) => ($(
        impl Rem for Fraction<$integer> {
            type Output = Self;

            fn rem(self, divisor: Self) -> Self::Output {
                self.checked_rem(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl Rem<$integer> for Fraction<$integer> {
            type Output = Self;

            fn rem(self, divisor: $integer) -> Self::Output {
                self.checked_rem(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl Rem<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn rem(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedRem<Fraction<Self>>>::checked_rem(
                    self, divisor,
                )
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

integer_fraction_rem_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
