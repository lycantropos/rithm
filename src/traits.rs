use crate::utils;
use std::ops::Neg;

pub trait Gcd<Rhs = Self> {
    type Output;

    fn gcd(self, other: Rhs) -> Self::Output;
}

macro_rules! plain_gcd_impl {
    ($t:ty) => {
        impl Gcd for $t {
            type Output = Self;

            #[inline]
            fn gcd(self, other: Self) -> Self::Output {
                utils::gcd::<$t>(self, other)
            }
        }
    };
}

plain_gcd_impl!(i8);
plain_gcd_impl!(i16);
plain_gcd_impl!(i32);
plain_gcd_impl!(i64);
plain_gcd_impl!(i128);
plain_gcd_impl!(u8);
plain_gcd_impl!(u16);
plain_gcd_impl!(u32);
plain_gcd_impl!(u64);
plain_gcd_impl!(u128);
plain_gcd_impl!(usize);

pub trait DoublePrecision: Sized {
    type Type: From<Self>;
}

impl DoublePrecision for i8 {
    type Type = i16;
}

impl DoublePrecision for i16 {
    type Type = i32;
}

impl DoublePrecision for i32 {
    type Type = i64;
}

impl DoublePrecision for i64 {
    type Type = i128;
}

impl DoublePrecision for u8 {
    type Type = u16;
}

impl DoublePrecision for u16 {
    type Type = u32;
}

impl DoublePrecision for u32 {
    type Type = u64;
}

impl DoublePrecision for u64 {
    type Type = u128;
}

pub trait Signed {
    type Type: Neg<Output = Self::Type>;
}

impl Signed for u8 {
    type Type = i8;
}

impl Signed for u16 {
    type Type = i16;
}

impl Signed for u32 {
    type Type = i32;
}

impl Signed for u64 {
    type Type = i64;
}

impl Signed for u128 {
    type Type = i128;
}

impl Signed for i8 {
    type Type = i8;
}

impl Signed for i16 {
    type Type = i16;
}

impl Signed for i32 {
    type Type = i32;
}

impl Signed for i64 {
    type Type = i64;
}

impl Signed for i128 {
    type Type = i128;
}

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Type;
pub type SignedOf<T> = <T as Signed>::Type;
