use std::ops::Div;

use traiter::numbers::CheckedDiv;

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Div
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDiv<Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedDiv<&'a Self, Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: &Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Div<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDiv<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn div(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDiv<Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>>,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDiv<BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self:
        CheckedDiv<&'a BigInt<Digit, DIGIT_BITNESS>, Output = Option<Self>>,
{
    type Output = Self;

    fn div(self, divisor: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDiv<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn div(self, divisor: BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedDiv<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn div(self, divisor: &BigInt<Digit, DIGIT_BITNESS>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDiv<Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn div(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Div<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self:
        CheckedDiv<&'a Fraction<Self>, Output = Option<Fraction<Self>>>,
{
    type Output = Fraction<Self>;

    fn div(self, divisor: &Fraction<Self>) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Div<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDiv<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn div(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    Div<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDiv<
        &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>,
    >,
{
    type Output = Fraction<BigInt<Digit, DIGIT_BITNESS>>;

    fn div(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_div(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! integer_fraction_div_impl {
    ($($integer:ty)*) => ($(
        impl Div for Fraction<$integer> {
            type Output = Self;

            fn div(self, divisor: Self) -> Self::Output {
                self.checked_div(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl Div<$integer> for Fraction<$integer> {
            type Output = Self;

            fn div(self, divisor: $integer) -> Self::Output {
                self.checked_div(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl Div<Fraction<Self>> for $integer {
            type Output = Fraction<Self>;

            fn div(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedDiv<Fraction<Self>>>::checked_div(
                    self, divisor,
                )
                .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

integer_fraction_div_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
