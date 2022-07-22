use std::ops::Mul;

use traiter::numbers::CheckedDivEuclid;

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (self.numerator * divisor.denominator)
            .checked_div_euclid(self.denominator * divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: &Self) -> Self::Output {
        (self.numerator * &divisor.denominator)
            .checked_div_euclid(self.denominator * &divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        (&self.numerator * divisor.denominator)
            .checked_div_euclid(&self.denominator * divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedDivEuclid
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (&self.numerator * &divisor.denominator)
            .checked_div_euclid(&self.denominator * &divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(self.denominator * divisor)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(self.denominator * divisor)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(&self.denominator * divisor)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.numerator
            .checked_div_euclid(&self.denominator * divisor)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    Self: CheckedDivEuclid<Output = Option<Self>> + Mul<Output = Self>,
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<&Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedDivEuclid<&'a Self, Output = Option<Self>>
        + Mul<&'a Self, Output = Self>,
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: &Fraction<Self>) -> Self::Output {
        (self * &divisor.denominator).checked_div_euclid(&divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    BigInt<Digit, SEPARATOR, SHIFT>:
        CheckedDivEuclid<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>,
    for<'a> Self: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivEuclid<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivEuclid<
        &'a BigInt<Digit, SEPARATOR, SHIFT>,
        Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
    >,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
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
