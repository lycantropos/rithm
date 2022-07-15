use std::fmt::Debug;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Shl, ShlAssign,
    Shr, ShrAssign, Sub, SubAssign,
};

use traiter::numbers::{
    Abs, Floor, FractExp, Gcd, LoadExp, RemEuclid, Signed, Unitary, Zeroable,
};

pub trait AdditiveMonoid = Add<Self, Output = Self> + Sized + Zeroable;

pub trait AdditiveGroup = AdditiveMonoid + SubtractiveMagma;

pub trait AssigningAdditiveMonoid = AdditiveMonoid + AddAssign<Self>;

pub trait AssigningAdditiveGroup =
    AssigningAdditiveMonoid + AssigningSubtractiveMagma;

pub trait AssigningBitwiseConjunctiveMagma =
    BitwiseConjunctiveMagma + BitAndAssign<Self>;

pub trait AssigningBitwiseDisjunctiveMonoid =
    BitwiseDisjunctiveMonoid + BitOrAssign<Self>;

pub trait AssigningBitwiseExclusiveDisjunctiveMonoid =
    BitwiseExclusiveDisjunctiveMonoid + BitXorAssign<Self>;

pub trait AssigningDivisivePartialMagma =
    DivisivePartialMagma + DivAssign<Self>;

pub trait AssigningMultiplicativeMonoid =
    MultiplicativeMonoid + MulAssign<Self>;

pub trait AssigningShiftableLeftBy<Shift = Self> =
    ShiftableLeftBy<Shift> + ShlAssign<Shift>;

pub trait AssigningShiftableRightBy<Shift = Self> =
    ShiftableRightBy<Shift> + ShrAssign<Shift>;

pub trait AssigningSubtractiveMagma = SubtractiveMagma + SubAssign<Self>;

pub trait BitwiseConjunctiveMagma =
    BitAnd<Self, Output = Self> + Sized + Zeroable;

pub trait BitwiseDisjunctiveMonoid =
    BitOr<Self, Output = Self> + Sized + Zeroable;

pub trait BitwiseExclusiveDisjunctiveMonoid =
    BitXor<Self, Output = Self> + Sized + Zeroable;

pub trait BitwiseNegatableUnaryAlgebra = Not<Output = Self>;

pub trait DivisivePartialMagma = Div<Self, Output = Self> + Sized;

pub trait Float = AssigningAdditiveMonoid
    + AssigningDivisivePartialMagma
    + AssigningMultiplicativeMonoid
    + AssigningSubtractiveMagma
    + Copy
    + Floor<Output = Self>
    + FractExp<Output = (Self, i32)>
    + From<f32>
    + LoadExp<i32, Output = Self>
    + MantissaDigits
    + MaxExp
    + MinExp
    + PartialEq;

pub trait GcdMagma = Gcd<Self, Output = Self> + Sized;

pub trait ModularUnaryAlgebra = Abs<Output = Self>;

pub trait ModularPartialMagma = RemEuclid<Self, Output = Self> + Sized;

pub trait ModularSubtractiveMagma = WrappingSub<Self, Output = Self> + Sized;

pub trait MultiplicativeMonoid = Mul<Self, Output = Self> + Sized + Unitary;

pub trait NegatableUnaryAlgebra = Neg<Output = Self>;

pub trait ShiftableLeftBy<Shift = Self> = Shl<Shift, Output = Self>;

pub trait ShiftableRightBy<Shift = Self> = Shr<Shift, Output = Self>;

pub trait SubtractiveMagma = Sized + Sub<Self, Output = Self>;

pub trait CheckedDivAsF32<Divisor = Self> {
    type Output: Maybe<Result = f32>;

    fn checked_div_as_f32(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_checked_div_as_f32_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivAsF32 for $t {
            type Output = Option<f32>;

            #[inline(always)]
            fn checked_div_as_f32(self, divisor: Self) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    Some((self as f32) / (divisor as f32))
                }
            }
        }
    )*)
}

primitive_checked_div_as_f32_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedDivAsF64<Divisor = Self> {
    type Output: Maybe<Result = f64>;

    fn checked_div_as_f64(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_checked_div_as_f64_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivAsF64 for $t {
            type Output = Option<f64>;

            #[inline(always)]
            fn checked_div_as_f64(self, divisor: Self) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    Some((self as f64) / (divisor as f64))
                }
            }
        }
    )*)
}

primitive_checked_div_as_f64_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

impl HasSignBit for isize {
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

pub trait Maybe {
    type Result;
    type Error;

    fn error(self) -> Self::Error;
    fn is_error(&self) -> bool;
    fn is_result(&self) -> bool;
    fn result(self) -> Self::Result;
}

impl<T: Debug> Maybe for Option<T> {
    type Result = T;
    type Error = Option<T>;

    #[inline(always)]
    fn error(self) -> Self::Error {
        match self {
            Some(value) => {
                panic!("called `Option::error()` on `Some{:?}` value", value)
            }
            None => None,
        }
    }

    #[inline(always)]
    fn is_error(&self) -> bool {
        Option::<T>::is_none(self)
    }

    #[inline(always)]
    fn is_result(&self) -> bool {
        Option::<T>::is_some(self)
    }

    #[inline(always)]
    fn result(self) -> Self::Result {
        Option::<T>::unwrap(self)
    }
}

impl<T: Debug, E: Debug> Maybe for Result<T, E> {
    type Result = T;
    type Error = E;

    #[inline(always)]
    fn error(self) -> Self::Error {
        Result::<T, E>::unwrap_err(self)
    }

    #[inline(always)]
    fn is_error(&self) -> bool {
        Result::<T, E>::is_err(self)
    }

    #[inline(always)]
    fn is_result(&self) -> bool {
        Result::<T, E>::is_ok(self)
    }

    #[inline(always)]
    fn result(self) -> Self::Result {
        Result::<T, E>::unwrap(self)
    }
}

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
    type Result: Signed + NegatableUnaryAlgebra;
}

impl Oppose for i8 {
    type Result = i8;
}

impl Oppose for isize {
    type Result = isize;
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
