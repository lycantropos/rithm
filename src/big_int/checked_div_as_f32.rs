use super::digits::{
    checked_div_approximation, BinaryDigitConvertibleToFloat, CheckedDivApproximationError,
    DivisibleDigit,
};
use super::types::BigInt;
use crate::traits::{BitwiseNegatableUnaryAlgebra, CheckedDivAsF32};

impl<
        Digit: BinaryDigitConvertibleToFloat<f32> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF32 for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivApproximationError>;

    fn checked_div_as_f32(self, divisor: Self) -> Self::Output {
        checked_div_approximation::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f32> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF32<&Self> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivApproximationError>;

    fn checked_div_as_f32(self, divisor: &Self) -> Self::Output {
        checked_div_approximation::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f32> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF32<BigInt<Digit, SEPARATOR, SHIFT>> for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivApproximationError>;

    fn checked_div_as_f32(self, divisor: BigInt<Digit, SEPARATOR, SHIFT>) -> Self::Output {
        checked_div_approximation::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}

impl<
        Digit: BinaryDigitConvertibleToFloat<f32> + BitwiseNegatableUnaryAlgebra + DivisibleDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivAsF32 for &BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Result<f32, CheckedDivApproximationError>;

    fn checked_div_as_f32(self, divisor: Self) -> Self::Output {
        checked_div_approximation::<Digit, f32, SHIFT>(&self.digits, &divisor.digits)
            .map(|modulus| ((self.sign * divisor.sign) as f32) * modulus)
    }
}
