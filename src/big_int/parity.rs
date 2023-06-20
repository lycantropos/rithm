use traiter::numbers::Parity;

use super::types::BigInt;

impl<'a, Digit, const DIGIT_BITNESS: usize> Parity
    for &'a BigInt<Digit, DIGIT_BITNESS>
where
    &'a Digit: Parity,
{
    fn is_even(self) -> bool {
        self.digits[0].is_even()
    }

    fn is_odd(self) -> bool {
        self.digits[0].is_odd()
    }
}

impl<Digit, const DIGIT_BITNESS: usize> Parity for BigInt<Digit, DIGIT_BITNESS>
where
    Digit: Parity,
{
    fn is_even(self) -> bool {
        unsafe { self.digits.into_iter().next().unwrap_unchecked() }.is_even()
    }

    fn is_odd(self) -> bool {
        unsafe { self.digits.into_iter().next().unwrap_unchecked() }.is_odd()
    }
}
