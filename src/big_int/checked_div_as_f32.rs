use crate::traits::CheckedDivAsF32;

use super::digits::{checked_div_as_float, DivisibleAsFloatDigit};
use super::types::{BigInt, CheckedDivAsFloatError};

impl<Digit: DivisibleAsFloatDigit<f32>, const SEPARATOR: char, const SHIFT: usize> CheckedDivAsF32
    for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivAsFloatError>;

    fn checked_div_as_f32(self, divisor: Self) -> Self::Output {
        checked_div_as_float::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<Digit: DivisibleAsFloatDigit<f32>, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivAsF32<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivAsFloatError>;

    fn checked_div_as_f32(self, divisor: &Self) -> Self::Output {
        checked_div_as_float::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<Digit: DivisibleAsFloatDigit<f32>, const SEPARATOR: char, const SHIFT: usize>
    CheckedDivAsF32<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivAsFloatError>;

    fn checked_div_as_f32(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_as_float::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<Digit: DivisibleAsFloatDigit<f32>, const SEPARATOR: char, const SHIFT: usize> CheckedDivAsF32
    for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivAsFloatError>;

    fn checked_div_as_f32(self, divisor: Self) -> Self::Output {
        checked_div_as_float::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}
