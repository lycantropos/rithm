use std::ops::{Add, Mul, Sub};

use traiter::numbers::{
    CheckedDivRemEuclid, CheckedRemEuclidInv, Signed, Unitary, Zeroable,
};

use super::types::BigInt;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclidInv
    for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: CheckedRemEuclidInv<&'a Self, Output = Option<Self>>,
{
    type Output = Option<Self>;

    fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
        self.checked_rem_euclid_inv(&divisor)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRemEuclidInv<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> Self: Clone
        + Mul<&'a Self, Output = Self>
        + Signed
        + Sub<Output = Self>
        + Unitary,
    for<'a> &'a Self: Add<Self, Output = Self>
        + CheckedDivRemEuclid<Output = Option<(Self, Self)>>,
{
    type Output = Option<Self>;

    fn checked_rem_euclid_inv(self, divisor: &Self) -> Self::Output {
        let mut candidate = Self::zero();
        let mut result = Self::one();
        let mut step_dividend = self;
        let mut step_divisor = divisor.clone();
        while !step_divisor.is_zero() {
            let (quotient, remainder) = unsafe {
                step_dividend
                    .checked_div_rem_euclid(&step_divisor)
                    .unwrap_unchecked()
            };
            step_dividend = step_divisor;
            step_divisor = remainder;
            (candidate, result) = (result - quotient * &candidate, candidate);
        }
        if step_dividend.is_one() {
            Some(if result.is_negative() {
                divisor + result
            } else {
                result
            })
        } else {
            None
        }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    CheckedRemEuclidInv<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRemEuclidInv<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Clone,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid_inv(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        self.clone().checked_rem_euclid_inv(&divisor)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> CheckedRemEuclidInv
    for &BigInt<Digit, SEPARATOR, SHIFT>
where
    for<'a> BigInt<Digit, SEPARATOR, SHIFT>: CheckedRemEuclidInv<
            &'a BigInt<Digit, SEPARATOR, SHIFT>,
            Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>,
        > + Clone,
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
        self.clone().checked_rem_euclid_inv(divisor)
    }
}
