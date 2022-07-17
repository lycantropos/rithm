use std::ops::Neg;

use traiter::numbers::Signed;

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

pub trait TryDivAsFloat<Divisor, Output> {
    type Error;

    fn try_div_as_float(self, divisor: Divisor)
        -> Result<Output, Self::Error>;
}

macro_rules! impl_try_div_integer_as_float {
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

impl_try_div_integer_as_float!(
    u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize => f32
);
impl_try_div_integer_as_float!(
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

macro_rules! primitive_mantissa_digits_impl {
    ($($t:ty)*) => ($(
        impl MantissaDigits for $t {
            const MANTISSA_DIGITS: usize = <$t>::MANTISSA_DIGITS as usize;
        }
    )*)
}

primitive_mantissa_digits_impl!(f32 f64);

pub trait MaxExp {
    const MAX_EXP: i32;
}

macro_rules! primitive_max_exp_impl {
    ($($t:ty)*) => ($(
        impl MaxExp for $t {
            const MAX_EXP: i32 = <$t>::MAX_EXP;
        }
    )*)
}

primitive_max_exp_impl!(f32 f64);

pub trait MinExp {
    const MIN_EXP: i32;
}

macro_rules! primitive_min_exp_impl {
    ($($t:ty)*) => ($(
        impl MinExp for $t {
            const MIN_EXP: i32 = <$t>::MIN_EXP;
        }
    )*)
}

primitive_min_exp_impl!(f32 f64);

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

macro_rules! impl_float_unchecked_to_int {
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

impl_float_unchecked_to_int!(
    f32 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize
);
impl_float_unchecked_to_int!(
    f64 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize
);

pub trait WrappingSub<Subtrahend = Self> {
    type Output;

    fn wrapping_sub(self, subtrahend: Subtrahend) -> Self::Output;
}

macro_rules! primitive_wrapping_sub_impl {
    ($($t:ty)*) => ($(
        impl WrappingSub for $t {
            type Output = $t;

            #[inline(always)]
            fn wrapping_sub(self, subtrahend: Self) -> Self::Output {
                <$t>::wrapping_sub(self, subtrahend)
            }
        }
    )*)
}

primitive_wrapping_sub_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Result;
pub type OppositionOf<T> = <T as Oppose>::Result;
