use std::ops::Neg;

use traiter::numbers::Signed;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

pub trait TryDivAsFloat<Divisor, Output> {
    type Error;

    fn try_div_as_float(self, divisor: Divisor)
        -> Result<Output, Self::Error>;
}

macro_rules! try_div_integer_as_float_impl {
    ($($integer:ty)+ => $float:ty) => {
        $(
            impl TryDivAsFloat<Self, $float> for $integer {
                type Error = &'static str;

                #[inline]
                fn try_div_as_float(self, divisor: Self) -> Result<$float, Self::Error> {
                    if divisor == 0 {
                        Err(UNDEFINED_DIVISION_ERROR_MESSAGE)
                    } else {
                        Ok((self as $float) / (divisor as $float))
                    }
                }
            }
        )+
    }
}

try_div_integer_as_float_impl!(
    u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize => f32
);
try_div_integer_as_float_impl!(
    u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize => f64
);

pub trait DoublePrecision: Sized {
    type Result: From<Self>;
}

impl DoublePrecision for i8 {
    type Result = i16;
}

impl DoublePrecision for i16 {
    type Result = i32;
}

impl DoublePrecision for i32 {
    type Result = i64;
}

impl DoublePrecision for i64 {
    type Result = i128;
}

impl DoublePrecision for u8 {
    type Result = u16;
}

impl DoublePrecision for u16 {
    type Result = u32;
}

impl DoublePrecision for u32 {
    type Result = u64;
}

impl DoublePrecision for u64 {
    type Result = u128;
}

pub trait HasSignBit {
    const RESULT: bool;
}

impl HasSignBit for i8 {
    const RESULT: bool = true;
}

impl HasSignBit for i16 {
    const RESULT: bool = true;
}

impl HasSignBit for i32 {
    const RESULT: bool = true;
}

impl HasSignBit for i64 {
    const RESULT: bool = true;
}

impl HasSignBit for i128 {
    const RESULT: bool = true;
}

impl HasSignBit for isize {
    const RESULT: bool = true;
}

impl HasSignBit for u8 {
    const RESULT: bool = false;
}

impl HasSignBit for u16 {
    const RESULT: bool = false;
}

impl HasSignBit for u32 {
    const RESULT: bool = false;
}

impl HasSignBit for u64 {
    const RESULT: bool = false;
}

impl HasSignBit for u128 {
    const RESULT: bool = false;
}

impl HasSignBit for usize {
    const RESULT: bool = false;
}

pub trait MantissaDigits {
    const MANTISSA_DIGITS: usize;
}

macro_rules! float_mantissa_digits_impl {
    ($($float:ty)*) => ($(
        impl MantissaDigits for $float {
            const MANTISSA_DIGITS: usize = <$float>::MANTISSA_DIGITS as usize;
        }
    )*)
}

float_mantissa_digits_impl!(f32 f64);

pub trait MaxExp {
    const MAX_EXP: i32;
}

macro_rules! float_max_exp_impl {
    ($($float:ty)*) => ($(
        impl MaxExp for $float {
            const MAX_EXP: i32 = <$float>::MAX_EXP;
        }
    )*)
}

float_max_exp_impl!(f32 f64);

pub trait MinExp {
    const MIN_EXP: i32;
}

macro_rules! float_min_exp_impl {
    ($($float:ty)*) => ($(
        impl MinExp for $float {
            const MIN_EXP: i32 = <$float>::MIN_EXP;
        }
    )*)
}

float_min_exp_impl!(f32 f64);

pub trait Oppose {
    type Result: Signed + Neg<Output = Self::Result>;
}

impl Oppose for i8 {
    type Result = i8;
}

impl Oppose for i16 {
    type Result = i16;
}

impl Oppose for i32 {
    type Result = i32;
}

impl Oppose for i64 {
    type Result = i64;
}

impl Oppose for i128 {
    type Result = i128;
}

impl Oppose for isize {
    type Result = isize;
}

impl Oppose for u8 {
    type Result = i8;
}

impl Oppose for u16 {
    type Result = i16;
}

impl Oppose for u32 {
    type Result = i32;
}

impl Oppose for u64 {
    type Result = i64;
}

impl Oppose for u128 {
    type Result = i128;
}

impl Oppose for usize {
    type Result = isize;
}

pub trait UncheckedToInt<Int> {
    unsafe fn unchecked_to_int(self) -> Int;
}

macro_rules! impl_float_unchecked_to_int_impl {
    ($float:ty => $($integer:ty)+) => {
        $(
            impl UncheckedToInt<$integer> for $float {
                #[inline(always)]
                unsafe fn unchecked_to_int(self) -> $integer {
                    self.to_int_unchecked::<$integer>()
                }
            }
        )+
    }
}

impl_float_unchecked_to_int_impl!(
    f32 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize
);
impl_float_unchecked_to_int_impl!(
    f64 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize
);

pub(crate) trait WrappingSub<Subtrahend = Self> {
    type Output;

    fn wrapping_sub(self, subtrahend: Subtrahend) -> Self::Output;
}

macro_rules! integer_wrapping_sub_impl {
    ($($integer:ty)*) => ($(
        impl WrappingSub for $integer {
            type Output = $integer;

            #[inline(always)]
            fn wrapping_sub(self, subtrahend: Self) -> Self::Output {
                <$integer>::wrapping_sub(self, subtrahend)
            }
        }
    )*)
}

integer_wrapping_sub_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Result;
pub type OppositionOf<T> = <T as Oppose>::Result;
