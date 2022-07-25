use std::ops::Rem;

use traiter::numbers::CheckedRem;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self: CheckedRem<Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> Self: CheckedRem<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: &Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self: CheckedRem<
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn rem(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self:
        CheckedRem<Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn rem(self, divisor: Self) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self: CheckedRem<BigInt<Digit, SEPARATOR, SHIFT>, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> Self:
        CheckedRem<&'a BigInt<Digit, SEPARATOR, SHIFT>, Output = Option<Self>>,
{
    type Output = Self;

    fn rem(self, divisor: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    Self: CheckedRem<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn rem(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> Self: CheckedRem<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn rem(self, divisor: &BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem<Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRem<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn rem(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Rem<&Fraction<Self>>
    for BigInt<Digit, SEPARATOR, SHIFT>
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedRem<
        Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn rem(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        self.checked_rem(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Rem<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<
        &'a Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
        Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn rem(
        self,
        divisor: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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
