use crate::big_int::types::CheckedDivAsFloatError;
use crate::traits::{BitwiseNegatableUnaryAlgebra, CheckedDivAsF64};

use super::digits::{checked_div_as_float, BinaryDigitConvertibleToFloat, DivisibleDigit};
use super::types::BigInt;

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64 for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivAsFloatError>;

    fn checked_div_as_f64(self, divisor: Self) -> Self::Output {
        checked_div_as_float::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivAsFloatError>;

    fn checked_div_as_f64(self, divisor: &Self) -> Self::Output {
        checked_div_as_float::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivAsFloatError>;

    fn checked_div_as_f64(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_as_float::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64 for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivAsFloatError>;

    fn checked_div_as_f64(self, divisor: Self) -> Self::Output {
        checked_div_as_float::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}
