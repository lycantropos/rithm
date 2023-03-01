use std::ops::Mul;

use traiter::numbers::{CheckedRem, Zeroable};

use crate::big_int::BigInt;

use super::types::{Fraction, NormalizeModuli};

impl<Digit, const DIGIT_BITNESS: usize> CheckedRem
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize> CheckedRem<&Self>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + Mul<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        > + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_rem(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedRem
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

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
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: Mul<
        BigInt<Digit, DIGIT_BITNESS>,
        Output = BigInt<Digit, DIGIT_BITNESS>,
    >,
    BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_rem(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<&BigInt<Digit, DIGIT_BITNESS>>
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
{
    type Output = Option<Self>;

    fn checked_rem(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedRem<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Mul<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = BigInt<Digit, DIGIT_BITNESS>,
        >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_rem(
        self,
        divisor: BigInt<Digit, DIGIT_BITNESS>,
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
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<&BigInt<Digit, DIGIT_BITNESS>>
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedRem<
            BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        > + Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_rem(
        self,
        divisor: &BigInt<Digit, DIGIT_BITNESS>,
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
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> CheckedRem<Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit, const DIGIT_BITNESS: usize> CheckedRem<&Fraction<Self>>
    for BigInt<Digit, DIGIT_BITNESS>
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

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    BigInt<Digit, DIGIT_BITNESS>: CheckedRem<Output = Option<BigInt<Digit, DIGIT_BITNESS>>>
        + NormalizeModuli<
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_rem(
        self,
        divisor: Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize>
    CheckedRem<&Fraction<BigInt<Digit, DIGIT_BITNESS>>>
    for &BigInt<Digit, DIGIT_BITNESS>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>:
        Mul<Output = BigInt<Digit, DIGIT_BITNESS>>,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: CheckedRem<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<BigInt<Digit, DIGIT_BITNESS>>,
        > + NormalizeModuli<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = (
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            ),
        >,
    Fraction<BigInt<Digit, DIGIT_BITNESS>>: Zeroable,
{
    type Output = Option<Fraction<BigInt<Digit, DIGIT_BITNESS>>>;

    fn checked_rem(
        self,
        divisor: &Fraction<BigInt<Digit, DIGIT_BITNESS>>,
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
            Some(Fraction::<BigInt<Digit, DIGIT_BITNESS>> {
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
