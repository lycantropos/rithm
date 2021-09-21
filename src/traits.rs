use std::num::ParseIntError;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, DivAssign, Mul, MulAssign, Neg,
    Rem, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait AdditiveMonoid<Other = Self> = Add<Other, Output = Self> + Zeroable;

pub trait AssigningAdditiveMonoid<Other = Self> = AdditiveMonoid<Other> + AddAssign<Other>;

pub trait AssigningBitwiseConjunctiveMagma<Other = Self> =
    BitwiseConjunctiveMagma<Other> + BitAndAssign<Other>;

pub trait AssigningBitwiseDisjunctiveMonoid<Other = Self> =
    BitwiseDisjunctiveMonoid<Other> + BitOrAssign<Other>;

pub trait AssigningDivisivePartialMagma<Divisor = Self> =
    DivisivePartialMagma<Divisor> + DivAssign<Divisor>;

pub trait AssigningMultiplicativeMonoid<Other = Self> =
    MultiplicativeMonoid<Other> + MulAssign<Other>;

pub trait AssigningShiftingLeftMonoid<Shift = Self> = ShiftingLeftMonoid<Shift> + ShlAssign<Shift>;

pub trait AssigningShiftingRightMonoid<Shift = Self> =
    ShiftingRightMonoid<Shift> + ShrAssign<Shift>;

pub trait AssigningSubtractiveMagma<Subtrahend = Self> =
    SubtractiveMagma<Subtrahend> + SubAssign<Subtrahend>;

pub trait BitwiseConjunctiveMagma<Other = Self> = BitAnd<Other, Output = Self> + Zeroable;

pub trait BitwiseDisjunctiveMonoid<Other = Self> = BitOr<Other, Output = Self> + Zeroable;

pub trait DivisivePartialMagma<Divisor = Self> = Div<Divisor, Output = Self>;

pub trait GcdMagma<Other = Self> = Gcd<Other, Output = Self>;

pub trait ModularUnaryAlgebra = Abs<Output = Self>;

pub trait ModularPartialMagma<Divisor = Self> = RemEuclid<Divisor, Output = Self>;

pub trait ModularSubtractiveMagma<Subtrahend = Self> = ModularSub<Subtrahend, Output = Self>;

pub trait MultiplicativeMonoid<Other = Self> = Mul<Other, Output = Self> + Unitary;

pub trait NegatableUnaryAlgebra = Neg<Output = Self>;

pub trait ShiftingLeftMonoid<Shift = Self> = Shl<Shift, Output = Self> + Zeroable;

pub trait ShiftingRightMonoid<Shift = Self> = Shr<Shift, Output = Self> + Zeroable;

pub trait SubtractiveMagma<Subtrahend = Self> = Sub<Subtrahend, Output = Self>;

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

pub trait DivEuclid<Divisor = Self> {
    type Output;

    fn div_euclid(self, divisor: Divisor) -> Self::Output;
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

pub trait CheckedDiv<Divisor = Self> {
    type Output;

    fn checked_div(self, divisor: Divisor) -> Self::Output;
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

pub trait CheckedDivEuclid<Divisor = Self> {
    type Output;

    fn checked_div_euclid(self, divisor: Divisor) -> Self::Output;
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

pub trait CheckedDivRem<Divisor = Self> {
    type Output;

    fn checked_div_rem(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_checked_div_rem_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivRem for $t {
            type Output = Option<(Self, Self)>;

            #[inline(always)]
            fn checked_div_rem(self, divisor: Self) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    Some((<$t>::div(self, divisor), <$t>::rem(self, divisor)))
                }
            }
        }
    )*)
}

plain_checked_div_rem_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait CheckedRem<Divisor = Self> {
    type Output;

    fn checked_rem(self, divisor: Divisor) -> Self::Output;
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

pub trait CheckedRemEuclid<Divisor = Self> {
    type Output;

    fn checked_rem_euclid(self, divisor: Divisor) -> Self::Output;
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

pub trait DivRem<Divisor = Self> {
    type Output;

    fn div_rem(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_div_rem_impl {
    ($($t:ty)*) => ($(
        impl DivRem for $t {
            type Output = (Self, Self);

            #[inline(always)]
            fn div_rem(self, divisor: Self) -> Self::Output {
                (<$t>::div(self, divisor), <$t>::rem(self, divisor))
            }
        }
    )*)
}

plain_div_rem_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait Gcd<Other = Self> {
    type Output;

    fn gcd(self, other: Other) -> Self::Output;
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

pub trait RemEuclid<Divisor = Self> {
    type Output;

    fn rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl RemEuclid for $t {
            type Output = $t;

            #[inline(always)]
            fn rem_euclid(self, divisor: Self) -> Self::Output {
                <$t>::rem_euclid(self, divisor)
            }
        }
    )*)
}

plain_rem_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait ModularSub<Subtrahend = Self> {
    type Output;

    fn wrapping_sub(self, subtrahend: Subtrahend) -> Self::Output;
}

macro_rules! plain_modular_sub_impl {
    ($($t:ty)*) => ($(
        impl ModularSub for $t {
            type Output = $t;

            #[inline(always)]
            fn wrapping_sub(self, subtrahend: Self) -> Self::Output {
                <$t>::wrapping_sub(self, subtrahend)
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
