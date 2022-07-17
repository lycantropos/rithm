use traiter::numbers::CheckedRemEuclid;

use super::digits::CheckedRemEuclidComponents;
use super::types::BigInt;

impl<
        Digit: CheckedRemEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRemEuclid for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_euclid_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedRemEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRemEuclid<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: &Self) -> Self::Output {
        Digit::checked_rem_euclid_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedRemEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRemEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        Digit::checked_rem_euclid_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> {
            sign,
            digits,
        })
    }
}

impl<
        Digit: CheckedRemEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRemEuclid for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_euclid_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| BigInt::<Digit, SEPARATOR, SHIFT> {
            sign,
            digits,
        })
    }
}
