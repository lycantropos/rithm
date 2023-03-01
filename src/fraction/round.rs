use std::cmp::Ordering;
use std::ops::{Add, Shl};

use traiter::numbers::{
    CheckedDivRemEuclid, Parity, Round, Signed, TieBreaking, Unitary,
};

use crate::big_int::BigInt;

use super::types::Fraction;

impl<Digit, const DIGIT_BITNESS: usize> Round
    for Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + CheckedDivRemEuclid<
            &'a BigInt<Digit, DIGIT_BITNESS>,
            Output = Option<(
                BigInt<Digit, DIGIT_BITNESS>,
                BigInt<Digit, DIGIT_BITNESS>,
            )>,
        > + Ord
        + Parity
        + Shl<usize, Output = BigInt<Digit, DIGIT_BITNESS>>
        + Signed
        + Unitary,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

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
                    quotient + BigInt::<Digit, DIGIT_BITNESS>::one()
                } else {
                    quotient
                }
            }
            Ordering::Greater => {
                quotient + BigInt::<Digit, DIGIT_BITNESS>::one()
            }
            Ordering::Less => quotient,
        }
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Round
    for &Fraction<BigInt<Digit, DIGIT_BITNESS>>
where
    for<'a> &'a BigInt<Digit, DIGIT_BITNESS>: CheckedDivRemEuclid<
        Output = Option<(
            BigInt<Digit, DIGIT_BITNESS>,
            BigInt<Digit, DIGIT_BITNESS>,
        )>,
    >,
    for<'a> BigInt<Digit, DIGIT_BITNESS>: Add<Output = BigInt<Digit, DIGIT_BITNESS>>
        + Ord
        + Parity
        + Shl<usize, Output = BigInt<Digit, DIGIT_BITNESS>>
        + Signed
        + Unitary,
{
    type Output = BigInt<Digit, DIGIT_BITNESS>;

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
                    quotient + BigInt::<Digit, DIGIT_BITNESS>::one()
                } else {
                    quotient
                }
            }
            Ordering::Greater => {
                quotient + BigInt::<Digit, DIGIT_BITNESS>::one()
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
