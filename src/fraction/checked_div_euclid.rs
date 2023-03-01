use std::ops::Mul;

use traiter::numbers::CheckedDivEuclid;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivEuclid
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (self.numerator * divisor.denominator)
            .checked_div_euclid(self.denominator * divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivEuclid<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(self, divisor: &Self) -> Self::Output {
        (self.numerator * &divisor.denominator)
            .checked_div_euclid(self.denominator * &divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        (&self.numerator * divisor.denominator)
            .checked_div_euclid(&self.denominator * divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivEuclid
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (&self.numerator * &divisor.denominator)
            .checked_div_euclid(&self.denominator * &divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(self.denominator * divisor)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(self.denominator * divisor)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(&self.denominator * divisor)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(&self.denominator * divisor)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivEuclid<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    Self: CheckedDivEuclid<Output = Option<Self>> + Mul<Output = Self>,
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedDivEuclid<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> Self: CheckedDivEuclid<&'a Self, Output = Option<Self>>
        + Mul<&'a Self, Output = Self>,
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: &Fraction<Self>) -> Self::Output {
        (self * &divisor.denominator).checked_div_euclid(&divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    BigInt<Digit, DIGIT_BITNESS>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>,
    for<'a> Self: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedDivEuclid<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedDivEuclid<
        &'a BigInt<Digit, DIGIT_BITNESS>,
        Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
    >,
{
    type Output = Option<BigInt<Digit, DIGIT_BITNESS>>;

    fn checked_div_euclid(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
    ) -> Self::Output {
        (self * &divisor.denominator).checked_div_euclid(&divisor.numerator)
    }
}

macro_rules! integer_checked_div_euclid_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDivEuclid for Fraction<$integer>
        {
            type Output = Option<$integer>;

            fn checked_div_euclid(self, divisor: Self) -> Self::Output {
                (self.numerator * divisor.denominator)
                    .checked_div_euclid(divisor.numerator * self.denominator)
            }
        }

        impl CheckedDivEuclid<$integer> for Fraction<$integer>
        {
            type Output = Option<$integer>;

            fn checked_div_euclid(self, divisor: $integer) -> Self::Output {
                self.numerator
                    .checked_div_euclid(divisor * self.denominator)
            }
        }

        impl CheckedDivEuclid<Fraction<Self>> for $integer {
            type Output = Option<Self>;

            fn checked_div_euclid(
                self,
                divisor: Fraction<Self>,
            ) -> Self::Output {
                (self * divisor.denominator)
                    .checked_div_euclid(divisor.numerator)
            }
        }
    )*)
}

integer_checked_div_euclid_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
