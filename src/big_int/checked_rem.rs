use traiter::numbers::CheckedRem;

use super::digits::CheckedRemComponents;
use super::types::BigInt;

impl<
        Digit: CheckedRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRem for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRem<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_rem(self, divisor: &Self) -> Self::Output {
        Digit::checked_rem_components::<SHIFT>(
            self.sign,
            &self.digits,
            divisor.sign,
            &divisor.digits,
        )
        .map(|(sign, digits)| Self { sign, digits })
    }
}

impl<
        Digit: CheckedRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRem<BigInt<Digit, SEPARATOR, SHIFT>>
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem(
        self,
        divisor: BigInt<Digit, SEPARATOR, SHIFT>,
    ) -> Self::Output {
        Digit::checked_rem_components::<SHIFT>(
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
        Digit: CheckedRemComponents,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRem for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn checked_rem(self, divisor: Self) -> Self::Output {
        Digit::checked_rem_components::<SHIFT>(
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
