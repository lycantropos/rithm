use super::digits::{
    checked_div_approximation, BinaryDigitConvertibleToFloat, CheckedDivApproximationError,
    DivisibleDigit,
};
use super::types::BigInt;
use crate::traits::{BitwiseNegatableUnaryAlgebra, CheckedDivAsF64};

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64 for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivApproximationError>;

    fn checked_div_as_f64(self, divisor: Self) -> Self::Output {
        checked_div_approximation::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivApproximationError>;

    fn checked_div_as_f64(self, divisor: &Self) -> Self::Output {
        checked_div_approximation::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivApproximationError>;

    fn checked_div_as_f64(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_approximation::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f64> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF64 for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f64, CheckedDivApproximationError>;

    fn checked_div_as_f64(self, divisor: Self) -> Self::Output {
        checked_div_approximation::<Digit, f64, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f64) * modulus)
    }
}
