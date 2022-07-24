use std::ops::Mul;

use traiter::numbers::{CheckedRem, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (self.numerator * &divisor.denominator)
                    .checked_rem(&self.denominator * divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(self.denominator * divisor.denominator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRem<&Self>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Mul<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    Self: Zeroable,
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: &Self) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (self.numerator * &divisor.denominator)
                    .checked_rem(&self.denominator * &divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(self.denominator * &divisor.denominator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_rem(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (&self.numerator * &divisor.denominator)
                    .checked_rem(&self.denominator * divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(&self.denominator * divisor.denominator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRem
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (&self.numerator * &divisor.denominator)
                    .checked_rem(&self.denominator * &divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(&self.denominator * &divisor.denominator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: Mul<
        BigInt<Digit, SEPARATOR, SHIFT>,
        Output = BigInt<Digit, SEPARATOR, SHIFT>,
    >,
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_rem(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                self.numerator
                    .checked_rem(&self.denominator * divisor)
                    .unwrap_unchecked()
            }
            .normalize_moduli(self.denominator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<&BigInt<Digit, SEPARATOR, SHIFT>>
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_rem(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                self.numerator
                    .checked_rem(&self.denominator * divisor)
                    .unwrap_unchecked()
            }
            .normalize_moduli(self.denominator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Mul<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = BigInt<Digit, SEPARATOR, SHIFT>,
        >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_rem(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                self.numerator
                    .checked_rem(&self.denominator * divisor)
                    .unwrap_unchecked()
            }
            .normalize_moduli(&self.denominator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<&BigInt<Digit, SEPARATOR, SHIFT>>
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<
            BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        > + Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_rem(
        self,
        divisor: &BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                self.numerator
                    .checked_rem(&self.denominator * divisor)
                    .unwrap_unchecked()
            }
            .normalize_moduli(&self.denominator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedRem<Output = Option<Self>>
        + Mul<&'a Self, Output = Self>
        + NormalizeModuli<Output = (Self, Self)>,
    Fraction<Self>: Zeroable,
{
    type Output = Option<Fraction<Self>>;

    fn checked_rem(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (self * &divisor.denominator)
                    .checked_rem(divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(divisor.denominator);
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<&Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedRem<&'a Self, Output = Option<Self>>
        + Mul<&'a Self, Output = Self>
        + NormalizeModuli<&'a Self, Output = (Self, Self)>,
    Fraction<Self>: Zeroable,
{
    type Output = Option<Fraction<Self>>;

    fn checked_rem(self, divisor: &Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (self * &divisor.denominator)
                    .checked_rem(&divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(&divisor.denominator);
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_rem(
        self,
        divisor: Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (self * &divisor.denominator)
                    .checked_rem(divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(divisor.denominator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRem<&Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>:
        Mul<Output = BigInt<Digit, SEPARATOR, SHIFT>>,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRem<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + NormalizeModuli<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = (
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            ),
        >,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn checked_rem(
        self,
        divisor: &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>,
    ) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = unsafe {
                (self * &divisor.denominator)
                    .checked_rem(&divisor.numerator)
                    .unwrap_unchecked()
            }
            .normalize_moduli(&divisor.denominator);
            Some(Fraction::<BigInt<Digit, SEPARATOR, SHIFT>> {
                numerator,
                denominator,
            })
        }
    }
}

macro_rules! integer_checked_rem_fraction_impl {
    ($($integer:ty)*) => ($(
        impl CheckedRem for Fraction<$integer> {
            type Output = Option<Self>;

            fn checked_rem(self, divisor: Self) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = unsafe {
                        (self.numerator * divisor.denominator)
                            .checked_rem(divisor.numerator * self.denominator)
                            .unwrap_unchecked()
                    }
                    .normalize_moduli(self.denominator * divisor.denominator);
                    Some(Self {
                        numerator,
                        denominator,
                    })
                }
            }
        }

        impl CheckedRem<$integer> for Fraction<$integer> {
            type Output = Option<Self>;

            fn checked_rem(self, divisor: $integer) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = unsafe {
                        self.numerator
                            .checked_rem(divisor * self.denominator)
                            .unwrap_unchecked()
                    }
                    .normalize_moduli(self.denominator);
                    Some(Self {
                        numerator,
                        denominator,
                    })
                }
            }
        }

        impl CheckedRem<Fraction<Self>> for $integer {
            type Output = Option<Fraction<Self>>;

            fn checked_rem(self, divisor: Fraction<Self>) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = unsafe {
                        (self * divisor.denominator)
                            .checked_rem(divisor.numerator)
                            .unwrap_unchecked()
                    }
                    .normalize_moduli(divisor.denominator);
                    Some(Fraction::<Self> {
                        numerator,
                        denominator,
                    })
                }
            }
        }
    )*)
}

integer_checked_rem_fraction_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
