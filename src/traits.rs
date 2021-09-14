use std::num::ParseIntError;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, DivAssign, Mul, MulAssign, Neg,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait AdditiveMonoid<Rhs = Self> = Add<Rhs, Output = Self> + Zeroable;

pub trait AssigningAdditiveMonoid<Rhs = Self> = AdditiveMonoid<Rhs> + AddAssign<Rhs>;

pub trait AssigningBitwiseConjunctiveMagma<Rhs = Self> =
    BitwiseConjunctiveMagma<Rhs> + BitAndAssign<Rhs>;

pub trait AssigningBitwiseDisjunctiveMonoid<Rhs = Self> =
    BitwiseDisjunctiveMonoid<Rhs> + BitOrAssign<Rhs>;

pub trait AssigningDivisivePartialMagma<Rhs = Self> = DivisivePartialMagma<Rhs> + DivAssign<Rhs>;

pub trait AssigningMultiplicativeMonoid<Rhs = Self> = MultiplicativeMonoid<Rhs> + MulAssign<Rhs>;

pub trait AssigningShiftingLeftMonoid<Rhs = Self> = ShiftingLeftMonoid<Rhs> + ShlAssign<Rhs>;

pub trait AssigningShiftingRightMonoid<Rhs = Self> = ShiftingRightMonoid<Rhs> + ShrAssign<Rhs>;

pub trait AssigningSubtractiveMagma<Rhs = Self> = SubtractiveMagma<Rhs> + SubAssign<Rhs>;

pub trait BitwiseConjunctiveMagma<Rhs = Self> = BitAnd<Rhs, Output = Self> + Zeroable;

pub trait BitwiseDisjunctiveMonoid<Rhs = Self> = BitOr<Rhs, Output = Self> + Zeroable;

pub trait DivisivePartialMagma<Rhs = Self> = Div<Rhs, Output = Self>;

pub trait GcdMagma<Rhs = Self> = Gcd<Rhs, Output = Self>;

pub trait ModularUnaryAlgebra = Abs<Output = Self>;

pub trait ModularPartialMagma<Rhs = Self> = RemEuclid<Rhs, Output = Self>;

pub trait ModularSubtractiveMagma<Rhs = Self> = ModularSub<Rhs, Output = Self>;

pub trait MultiplicativeMonoid<Rhs = Self> = Mul<Rhs, Output = Self> + Unitary;

pub trait NegatableUnaryAlgebra = Neg<Output = Self>;

pub trait ShiftingLeftMonoid<Rhs = Self> = Shl<Rhs, Output = Self> + Zeroable;

pub trait ShiftingRightMonoid<Rhs = Self> = Shr<Rhs, Output = Self> + Zeroable;

pub trait SubtractiveMagma<Rhs = Self> = Sub<Rhs, Output = Self>;

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

macro_rules! plain_abs_impl {
    ($($t:ty)*) => ($(
        impl Abs for $t {
            type Output = $t;

            #[inline(always)]
            fn abs(self) -> Self::Output {
                <$t>::abs(self)
            }
        }
    )*)
}

plain_abs_impl!(i8 i16 i32 i64 i128 isize);

pub trait DivEuclid<Rhs = Self> {
    type Output;

    fn div_euclid(self, divisor: Rhs) -> Self::Output;
}

macro_rules! plain_div_euclid_impl {
    ($($t:ty)*) => ($(
        impl DivEuclid for $t {
            type Output = Self;

            #[inline(always)]
            fn div_euclid(self, divisor: Self) -> Self::Output {
                <$t>::div_euclid(self, divisor)
            }
        }
    )*)
}

plain_div_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, divisor: Rhs) -> Self::Output;
}

macro_rules! plain_checked_div_impl {
    ($($t:ty)*) => ($(
        impl CheckedDiv for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_div(self, divisor: Self) -> Self::Output {
                <$t>::checked_div(self, divisor)
            }
        }
    )*)
}

plain_checked_div_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedDivEuclid<Rhs = Self> {
    type Output;

    fn checked_div_euclid(self, divisor: Rhs) -> Self::Output;
}

macro_rules! plain_checked_div_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivEuclid for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_div_euclid(self, divisor: Self) -> Self::Output {
                <$t>::checked_div_euclid(self, divisor)
            }
        }
    )*)
}

plain_checked_div_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedPow<Exponent> {
    type Output;

    fn checked_pow(self, exponent: Exponent) -> Self::Output;
}

macro_rules! plain_checked_pow_impl {
    ($($t:ty)*) => ($(
        impl CheckedPow<u32> for $t {
            type Output = Option<$t>;

            #[inline(always)]
            fn checked_pow(self, exponent: u32) -> Self::Output {
                <$t>::checked_pow(self, exponent)
            }
        }
    )*)
}

plain_checked_pow_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedRem<Rhs = Self> {
    type Output;

    fn checked_rem(self, divisor: Rhs) -> Self::Output;
}

macro_rules! plain_checked_rem_impl {
    ($($t:ty)*) => ($(
        impl CheckedRem for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem(self, divisor: Self) -> Self::Output {
                <$t>::checked_rem(self, divisor)
            }
        }
    )*)
}

plain_checked_rem_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedRemEuclid<Rhs = Self> {
    type Output;

    fn checked_rem_euclid(self, divisor: Rhs) -> Self::Output;
}

macro_rules! plain_checked_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedRemEuclid for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
                <$t>::checked_rem_euclid(self, divisor)
            }
        }
    )*)
}

plain_checked_rem_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait FromStrRadix: Sized {
    type Error;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error>;
}

macro_rules! plain_from_str_radix_impl {
    ($($t:ty)*) => ($(
        impl FromStrRadix for $t {
            type Error = ParseIntError;

            #[inline(always)]
            fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error> {
                <$t>::from_str_radix(string, radix)
            }
        }
    )*)
}

plain_from_str_radix_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(self, other: Rhs) -> Self::Output;
}

macro_rules! plain_gcd_impl {
    ($($t:ty)*) => ($(
        impl Gcd for $t {
            type Output = Self;

            #[inline(always)]
            fn gcd(self, other: Self) -> Self::Output {
                use crate::utils;
                utils::gcd::<$t>(self, other)
            }
        }
    )*)
}

plain_gcd_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Pow<Exponent> {
    type Output;

    fn pow(self, exponent: Exponent) -> Self::Output;
}

macro_rules! plain_pow_impl {
    ($($t:ty)*) => ($(
        impl Pow<u32> for $t {
            type Output = $t;

            #[inline(always)]
            fn pow(self, exponent: u32) -> Self::Output {
                <$t>::pow(self, exponent)
            }
        }
    )*)
}

plain_pow_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait RemEuclid<Rhs = Self> {
    type Output;

    fn rem_euclid(self, other: Rhs) -> Self::Output;
}

macro_rules! plain_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl RemEuclid for $t {
            type Output = $t;

            #[inline(always)]
            fn rem_euclid(self, other: Self) -> Self::Output {
                <$t>::rem_euclid(self, other)
            }
        }
    )*)
}

plain_rem_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait ModularSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

macro_rules! plain_modular_sub_impl {
    ($($t:ty)*) => ($(
        impl ModularSub for $t {
            type Output = $t;

            #[inline(always)]
            fn wrapping_sub(self, rhs: Self) -> Self::Output {
                <$t>::wrapping_sub(self, rhs)
            }
        }
    )*)
}

plain_modular_sub_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Oppositive: NegatableUnaryAlgebra + Zeroable {
    fn is_negative(&self) -> bool;
    fn is_positive(&self) -> bool;
}

macro_rules! plain_oppositive_impl {
    ($($t:ty)*) => ($(
        impl Oppositive for $t {
            #[inline(always)]
            fn is_negative(&self) -> bool {
                <$t>::is_negative(*self)
            }

            #[inline(always)]
            fn is_positive(&self) -> bool {
                <$t>::is_positive(*self)
            }
        }
    )*)
}

plain_oppositive_impl!(i8 i16 i32 i64 i128 isize);

pub trait Oppose {
    type Result: Oppositive;
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

pub trait Unitary {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

macro_rules! plain_unitary_impl {
    ($($t:ty)*) => ($(
        impl Unitary for $t {
            #[inline(always)]
            fn one() -> $t {1}

            #[inline(always)]
            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }
    )*)
}

plain_unitary_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Zeroable {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

macro_rules! plain_zero_impl {
    ($($t:ty)*) => ($(
        impl Zeroable for $t {
            #[inline(always)]
            fn zero() -> $t {0}

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    )*)
}

plain_zero_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Result;
pub type OppositionOf<T> = <T as Oppose>::Result;
