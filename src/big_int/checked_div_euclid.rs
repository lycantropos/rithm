use traiter::numbers::CheckedDivEuclid;

use super::digits::CheckedDivEuclidComponents;
use super::types::BigInt;

impl<
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivEuclid for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_euclid_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivEuclid<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: &Self) -> Self::Output {
        Digit::checked_div_euclid_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivEuclid<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        Digit::checked_div_euclid_components::<SHIFT>(
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
        Digit: CheckedDivEuclidComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivEuclid for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        Digit::checked_div_euclid_components::<SHIFT>(
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
