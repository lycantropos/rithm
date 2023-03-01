use traiter::numbers::{CheckedDivEuclid, DivEuclid};

use crate::big_int::BigInt;
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedDivEuclid<
        &'a Self,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(self, divisor: &Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    DivEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDivEuclid<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(self, divisor: Self) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDivEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    DivEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedDivEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    Self: CheckedDivEuclid<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    DivEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> Self: CheckedDivEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivEuclid<Fraction<Self>, Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> DivEuclid<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedDivEuclid<&'a Fraction<Self>, Output = Option<Self>>,
{
    type Output = Self;

    fn div_euclid(self, divisor: &Fraction<Self>) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    DivEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivEuclid<
        Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    DivEuclid<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<
        &'a Fraction<BigInt<Digit, DIGIT_BITNESS>>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

    fn div_euclid(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        self.checked_div_euclid(divisor)
            .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
    }
}

macro_rules! integer_fraction_div_euclid_impl {
    ($($integer:ty)*) => ($(
        impl DivEuclid for Fraction<$integer>
        {
            type Output = $integer;

            fn div_euclid(self, divisor: Self) -> Self::Output {
                self.checked_div_euclid(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl DivEuclid<$integer> for Fraction<$integer>
        {
            type Output = $integer;

            fn div_euclid(self, divisor: $integer) -> Self::Output {
                self.checked_div_euclid(divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }

        impl DivEuclid<Fraction<Self>> for $integer {
            type Output = Self;

            fn div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                <$integer as CheckedDivEuclid<Fraction<Self>>>::checked_div_euclid(self, divisor)
                    .expect(UNDEFINED_DIVISION_ERROR_MESSAGE)
            }
        }
    )*)
}

integer_fraction_div_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
