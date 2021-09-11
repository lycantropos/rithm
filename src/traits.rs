use std::ops::{
    Add, AddAssign, BitAnd, BitOr, Div, Mul, MulAssign, Neg, Rem, Shl, ShlAssign, Shr, ShrAssign,
    Sub, SubAssign,
};

pub trait AdditiveMonoid<Rhs = Self> = Add<Rhs, Output = Self> + Zero;

pub trait AssigningAdditiveMonoid<Rhs = Self> = AdditiveMonoid<Rhs> + AddAssign<Rhs>;

pub trait AssigningMultiplicativeMonoid<Rhs = Self> = MultiplicativeMonoid<Rhs> + MulAssign<Rhs>;

pub trait AssigningShiftingLeftMonoid<Rhs = Self> = ShiftingLeftMonoid<Rhs> + ShlAssign<Rhs>;

pub trait AssigningShiftingRightMonoid<Rhs = Self> = ShiftingRightMonoid<Rhs> + ShrAssign<Rhs>;

pub trait AssigningSubtractiveMagma<Rhs = Self> = SubtractiveMagma<Rhs> + SubAssign<Rhs>;

pub trait BitwiseAndMagma<Rhs = Self> = BitAnd<Rhs, Output = Self> + Zero;

pub trait BitwiseOrMonoid<Rhs = Self> = BitOr<Rhs, Output = Self> + Zero;

pub trait DivisivePartialMagma<Rhs = Self> = Div<Rhs, Output = Self>;

pub trait ModularPartialMagma<Rhs = Self> = Rem<Rhs, Output = Self>;

pub trait ModularSubtractiveMagma<Rhs = Self> = ModularSub<Rhs, Output = Self>;

pub trait MultiplicativeMonoid<Rhs = Self> = Mul<Rhs, Output = Self> + One;

pub trait ShiftingLeftMonoid<Rhs = Self> = Shl<Rhs, Output = Self> + Zero;

pub trait ShiftingRightMonoid<Rhs = Self> = Shr<Rhs, Output = Self> + Zero;

pub trait SubtractiveMagma<Rhs = Self> = Sub<Rhs, Output = Self>;

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

pub trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(self, other: Rhs) -> Self::Output;
}

macro_rules! plain_gcd_impl {
    ($($t:ty)*) => ($(
        impl Gcd for $t {
            type Output = Self;

            #[inline]
            fn gcd(self, other: Self) -> Self::Output {
                use crate::utils;
                utils::gcd::<$t>(self, other)
            }
        }
    )*)
}

plain_gcd_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait ModularSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

macro_rules! plain_modular_sub_impl {
    ($($t:ty)*) => ($(
        impl ModularSub for $t {
            type Output = $t;

            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self::Output {
                <$t>::wrapping_sub(self, rhs)
            }
        }
    )*)
}

plain_modular_sub_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait One {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

macro_rules! plain_one_impl {
    ($($t:ty)*) => ($(
        impl One for $t {
            fn one() -> $t {1}

            #[inline]
            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }
    )*)
}

plain_one_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Oppositive = Neg<Output = Self>;

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

pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

macro_rules! plain_zero_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            fn zero() -> $t {0}

            #[inline]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    )*)
}

plain_zero_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Result;
pub type OppositionOf<T> = <T as Oppose>::Result;
