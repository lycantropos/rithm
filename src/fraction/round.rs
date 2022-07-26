use std::cmp::Ordering;
use std::ops::{Add, Shl};

use traiter::numbers::{
    CheckedDivRemEuclid, Parity, Round, Signed, TieBreaking, Unitary,
};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Round
    for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + CheckedDivRemEuclid<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<(
                BigInt<Digit, SEPARATOR, SHIFT>,
                BigInt<Digit, SEPARATOR, SHIFT>,
            )>,
        > + Ord
        + Parity
        + Shl<usize, Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Signed
        + Unitary,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn round(self, tie_breaking: TieBreaking) -> Self::Output {
        let (quotient, remainder) = unsafe {
            self.numerator
                .checked_div_rem_euclid(&self.denominator)
                .unwrap_unchecked()
        };
        match (remainder << 1).cmp(&self.denominator) {
            Ordering::Equal => {
                if match tie_breaking {
                    TieBreaking::AwayFromZero => !quotient.is_negative(),
                    TieBreaking::ToEven => quotient.is_odd(),
                    TieBreaking::ToOdd => quotient.is_even(),
                    TieBreaking::TowardZero => quotient.is_negative(),
                } {
                    quotient + BigInt::<Digit, SEPARATOR, SHIFT>::one()
                } else {
                    quotient
                }
            }
            Ordering::Greater => {
                quotient + BigInt::<Digit, SEPARATOR, SHIFT>::one()
            }
            Ordering::Less => quotient,
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Round
    for &Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
where
    for<'a> &'a BigInt<Digit, SEPARATOR, SHIFT>: CheckedDivRemEuclid<
        Output = Option<(
            BigInt<Digit, SEPARATOR, SHIFT>,
            BigInt<Digit, SEPARATOR, SHIFT>,
        )>,
    >,
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: Add<Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Ord
        + Parity
        + Shl<usize, Output = BigInt<Digit, SEPARATOR, SHIFT>>
        + Signed
        + Unitary,
{
    type Output = BigInt<Digit, SEPARATOR, SHIFT>;

    fn round(self, tie_breaking: TieBreaking) -> Self::Output {
        let (quotient, remainder) = unsafe {
            self.numerator
                .checked_div_rem_euclid(&self.denominator)
                .unwrap_unchecked()
        };
        match (remainder << 1).cmp(&self.denominator) {
            Ordering::Equal => {
                if match tie_breaking {
                    TieBreaking::AwayFromZero => !quotient.is_negative(),
                    TieBreaking::ToEven => quotient.is_odd(),
                    TieBreaking::ToOdd => quotient.is_even(),
                    TieBreaking::TowardZero => quotient.is_negative(),
                } {
                    quotient + BigInt::<Digit, SEPARATOR, SHIFT>::one()
                } else {
                    quotient
                }
            }
            Ordering::Greater => {
                quotient + BigInt::<Digit, SEPARATOR, SHIFT>::one()
            }
            Ordering::Less => quotient,
        }
    }
}

macro_rules! signed_integer_fraction_round_impl {
    ($($integer:ty)*) => ($(
        impl Round for Fraction<$integer> {
            type Output = $integer;

            fn round(self, tie_breaking: TieBreaking) -> Self::Output {
                let (quotient, remainder) = unsafe {
                    self.numerator
                        .checked_div_rem_euclid(self.denominator)
                        .unwrap_unchecked()
                };
                match (remainder << 1).cmp(&self.denominator) {
                    Ordering::Equal => {
                        if match tie_breaking {
                            TieBreaking::AwayFromZero => {
                                !quotient.is_negative()
                            }
                            TieBreaking::ToEven => quotient.is_odd(),
                            TieBreaking::ToOdd => quotient.is_even(),
                            TieBreaking::TowardZero => quotient.is_negative(),
                        } {
                            quotient + 1
                        } else {
                            quotient
                        }
                    }
                    Ordering::Greater => quotient + 1,
                    Ordering::Less => quotient,
                }
            }
        }
    )*)
}

signed_integer_fraction_round_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_fraction_round_impl {
    ($($integer:ty)*) => ($(
        impl Round for Fraction<$integer> {
            type Output = $integer;

            fn round(self, tie_breaking: TieBreaking) -> Self::Output {
                let (quotient, remainder) = unsafe {
                    self.numerator
                        .checked_div_rem_euclid(self.denominator)
                        .unwrap_unchecked()
                };
                match (remainder << 1).cmp(&self.denominator) {
                    Ordering::Equal => {
                        if match tie_breaking {
                            TieBreaking::AwayFromZero => true,
                            TieBreaking::ToEven => quotient.is_odd(),
                            TieBreaking::ToOdd => quotient.is_even(),
                            TieBreaking::TowardZero => false,
                        } {
                            quotient + 1
                        } else {
                            quotient
                        }
                    }
                    Ordering::Greater => quotient + 1,
                    Ordering::Less => quotient,
                }
            }
        }
    )*)
}

unsigned_integer_fraction_round_impl!(u8 u16 u32 u64 u128 usize);
