use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::mem;
use std::num::ParseIntError;
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait AdditiveMonoid<Other = Self> = Add<Other, Output = Self> + Zeroable;

pub trait AssigningAdditiveMonoid<Other = Self> = AdditiveMonoid<Other> + AddAssign<Other>;

pub trait AssigningBitwiseConjunctiveMagma<Other = Self> =
    BitwiseConjunctiveMagma<Other> + BitAndAssign<Other>;

pub trait AssigningBitwiseDisjunctiveMonoid<Other = Self> =
    BitwiseDisjunctiveMonoid<Other> + BitOrAssign<Other>;

pub trait AssigningBitwiseExclusiveDisjunctiveMonoid<Other = Self> =
    BitwiseExclusiveDisjunctiveMonoid<Other> + BitXorAssign<Other>;

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

pub trait BitwiseExclusiveDisjunctiveMonoid<Other = Self> = BitXor<Other, Output = Self> + Zeroable;

pub trait BitwiseNegatableUnaryAlgebra = Not<Output = Self>;

pub trait DivisivePartialMagma<Divisor = Self> = Div<Divisor, Output = Self>;

pub trait Float = AssigningAdditiveMonoid
    + AssigningDivisivePartialMagma
    + AssigningMultiplicativeMonoid
    + AssigningSubtractiveMagma
    + Copy
    + Floor<Output = Self>
    + FrExp<Output = (Self, i32)>
    + From<f32>
    + LdExp<i32, Output = Self>
    + MantissaDigits
    + MaxExp
    + MinExp
    + PartialEq;

pub trait GcdMagma<Other = Self> = Gcd<Other, Output = Self>;

pub trait ModularUnaryAlgebra = Abs<Output = Self>;

pub trait ModularPartialMagma<Divisor = Self> = RemEuclid<Divisor, Output = Self>;

pub trait ModularSubtractiveMagma<Subtrahend = Self> = WrappingSub<Subtrahend, Output = Self>;

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

plain_abs_impl!(f32 f64 i8 i16 i32 i64 i128 isize);

pub trait Ceil {
    type Output;

    fn ceil(self) -> Self::Output;
}

macro_rules! plain_ceil_impl {
    ($($t:ty)*) => ($(
        impl Ceil for $t {
            type Output = $t;

            #[inline(always)]
            fn ceil(self) -> Self::Output {
                <$t>::ceil(self)
            }
        }
    )*)
}

plain_ceil_impl!(f32 f64);

#[derive(Clone, Eq, PartialEq)]
pub enum Endianness {
    Big,
    Little,
}

pub trait FromBytes {
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self;
}

macro_rules! plain_from_bytes_impl {
    ($($t:ty)*) => ($(
        impl FromBytes for $t {
            #[inline(always)]
            fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
                match endianness {
                   Endianness::Big => Self::from_be_bytes(bytes.try_into().unwrap()),
                   Endianness::Little => Self::from_le_bytes(bytes.try_into().unwrap()),
                }
            }
        }
    )*)
}

plain_from_bytes_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait ToBytes {
    type Output;

    fn to_bytes(self, endianness: Endianness) -> Self::Output;
}

macro_rules! plain_to_bytes_impl {
    ($($t:ty)*) => ($(
        impl ToBytes for $t {
            type Output = [u8; mem::size_of::<Self>()];

            #[inline(always)]
            fn to_bytes(self, endianness: Endianness) -> Self::Output {
                match endianness {
                   Endianness::Big => self.to_be_bytes(),
                   Endianness::Little => self.to_le_bytes(),
                }
            }
        }
    )*)
}

plain_to_bytes_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait BitLength {
    type Output;

    fn bit_length(self) -> Self::Output;
}

macro_rules! plain_bit_length_impl {
    ($($t:ty)*) => ($(
        impl BitLength for $t {
            type Output = usize;

            #[inline(always)]
            fn bit_length(self) -> Self::Output {
                (<$t>::BITS as usize) - (self.leading_zeros() as usize)
            }
        }
    )*)
}

plain_bit_length_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait CheckedDivAsF32<Divisor = Self> {
    type Output: Maybe<Result = f32>;

    fn checked_div_as_f32(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_checked_div_as_f32_impl {
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

plain_checked_div_as_f32_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedDivAsF64<Divisor = Self> {
    type Output: Maybe<Result = f64>;

    fn checked_div_as_f64(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_checked_div_as_f64_impl {
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

plain_checked_div_as_f64_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait CheckedDivRemEuclid<Divisor = Self> {
    type Output;

    fn checked_div_rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_checked_div_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivRemEuclid for $t {
            type Output = Option<(Self, Self)>;

            #[inline(always)]
            fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    Some((<$t>::div_euclid(self, divisor), <$t>::rem_euclid(self, divisor)))
                }
            }
        }
    )*)
}

plain_checked_div_rem_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait CheckedPowRemEuclid<Exponent, Divisor> {
    type Output;

    fn checked_pow_rem_euclid(self, exponent: Exponent, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_signed_checked_pow_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedPowRemEuclid<u32, $t> for $t {
            type Output = Option<$t>;

            #[inline]
            fn checked_pow_rem_euclid(self, exponent: u32, divisor: $t) -> Self::Output {
                if divisor.is_zero() {
                    return None;
                }
                let is_negative = divisor < 0;
                let divisor = divisor.abs();
                if divisor.is_one() {
                    return Some(Self::zero());
                }
                let base = if self < 0 || self > divisor {
                    self.rem_euclid(divisor)
                } else {
                    self
                };
                let mut result = base;
                let mut exponent_mask = 2u32;
                loop {
                    if exponent_mask > exponent {
                        exponent_mask >>= 1;
                        break;
                    }
                    exponent_mask <<= 1;
                }
                exponent_mask >>= 1;
                while !exponent_mask.is_zero() {
                    result = (result * result).rem_euclid(divisor);
                    if !(exponent & exponent_mask).is_zero() {
                        result = (result * base).rem_euclid(divisor);
                    }
                    exponent_mask >>= 1;
                }
                Some(if is_negative && !result.is_zero() {
                    result - divisor
                } else {
                    result
                })
            }
        }
    )*)
}

plain_signed_checked_pow_rem_euclid_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! plain_unsigned_checked_pow_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedPowRemEuclid<u32, $t> for $t {
            type Output = Option<$t>;

            #[inline]
            fn checked_pow_rem_euclid(self, exponent: u32, divisor: $t) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else if divisor.is_one() {
                    Some(Self::zero())
                } else {
                    let base = if self > divisor { self.rem_euclid(divisor) } else { self };
                    let mut result = base;
                    let mut exponent_mask = 2u32;
                    loop {
                        if exponent_mask > exponent {
                            exponent_mask >>= 1;
                            break;
                        }
                        exponent_mask <<= 1;
                    }
                    exponent_mask >>= 1;
                    while !exponent_mask.is_zero() {
                        result = (result * result).rem_euclid(divisor);
                        if !(exponent & exponent_mask).is_zero() {
                            result = (result * base).rem_euclid(divisor);
                        }
                        exponent_mask >>= 1;
                    }
                    Some(result)
                }
            }
        }
    )*)
}

plain_unsigned_checked_pow_rem_euclid_impl!(u8 u16 u32 u64 u128 usize);

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

pub trait CheckedRemEuclidInv<Divisor = Self> {
    type Output;

    fn checked_rem_euclid_inv(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_signed_checked_rem_euclid_inv_impl {
    ($($t:ty)*) => ($(
        impl CheckedRemEuclidInv for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
                let mut candidate = Self::zero();
                let mut result = Self::one();
                let mut step_dividend = self;
                let mut step_divisor = divisor;
                while !step_divisor.is_zero() {
                    let (quotient, remainder) = step_dividend.div_rem_euclid(step_divisor);
                    step_dividend = step_divisor;
                    step_divisor = remainder;
                    (result, candidate) = (candidate, result - quotient * candidate);
                }
                if step_dividend.is_one() {
                    Some(if result.is_negative() {
                        divisor + result
                    } else {
                        result
                    })
                } else {
                    None
                }
            }
        }
    )*)
}

plain_signed_checked_rem_euclid_inv_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! plain_unsigned_checked_rem_euclid_inv_impl {
    ($($t:ty)*) => ($(
        impl CheckedRemEuclidInv for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
                let mut candidate_modulus = Self::zero();
                let mut result_modulus = Self::one();
                let mut is_result_negative = false;
                let mut is_candidate_negative = false;
                let mut step_dividend = self;
                let mut step_divisor = divisor;
                while !step_divisor.is_zero() {
                    let (quotient, remainder) = step_dividend.div_rem_euclid(step_divisor);
                    step_dividend = step_divisor;
                    step_divisor = remainder;
                    let subtrahend_modulus = quotient * candidate_modulus;
                    (
                        is_result_negative,
                        result_modulus,
                        (is_candidate_negative, candidate_modulus),
                    ) = (
                        is_candidate_negative,
                        candidate_modulus,
                        if is_result_negative {
                            if is_candidate_negative {
                                if result_modulus > subtrahend_modulus {
                                    (true, result_modulus - subtrahend_modulus)
                                } else {
                                    (false, subtrahend_modulus - result_modulus)
                                }
                            } else {
                                (true, subtrahend_modulus + result_modulus)
                            }
                        } else if is_candidate_negative {
                            (false, subtrahend_modulus + result_modulus)
                        } else if result_modulus > subtrahend_modulus {
                            (false, result_modulus - subtrahend_modulus)
                        } else {
                            (true, subtrahend_modulus - result_modulus)
                        },
                    );
                }
                if step_dividend.is_one() {
                    Some(if is_result_negative {
                        divisor - result_modulus
                    } else {
                        result_modulus
                    })
                } else {
                    None
                }
            }
        }
    )*)
}

plain_unsigned_checked_rem_euclid_inv_impl!(u8 u16 u32 u64 u128 usize);

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

pub trait CheckedShl<Shift = Self> {
    type Output: Maybe<Result = Self>;

    fn checked_shl(self, shift: Shift) -> Self::Output;
}

macro_rules! signed_checked_shl_impl {
    ($t:ty, $f:ty) => {
        impl CheckedShl<$f> for $t {
            type Output = Option<$t>;

            #[inline(always)]
            fn checked_shl(self, other: $f) -> Self::Output {
                if other < 0 {
                    None
                } else {
                    if self.leading_zeros() < u32::try_from(other).ok()? {
                        None
                    } else {
                        Some(self << other)
                    }
                }
            }
        }
    };
}

macro_rules! unsigned_checked_shl_impl {
    ($t:ty, $f:ty) => {
        impl CheckedShl<$f> for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_shl(self, other: $f) -> Self::Output {
                if self.leading_zeros() < u32::try_from(other).ok()? {
                    None
                } else {
                    Some(self << other)
                }
            }
        }
    };
}

macro_rules! plain_checked_shl_impl {
    ($($t:ty)*) => ($(
        signed_checked_shl_impl! { $t, i8 }
        signed_checked_shl_impl! { $t, i16 }
        signed_checked_shl_impl! { $t, i32 }
        signed_checked_shl_impl! { $t, i64 }
        signed_checked_shl_impl! { $t, i128 }
        signed_checked_shl_impl! { $t, isize }

        unsigned_checked_shl_impl! { $t, u8 }
        unsigned_checked_shl_impl! { $t, u16 }
        unsigned_checked_shl_impl! { $t, u32 }
        unsigned_checked_shl_impl! { $t, u64 }
        unsigned_checked_shl_impl! { $t, u128 }
        unsigned_checked_shl_impl! { $t, usize }
    )*)
}

plain_checked_shl_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait CheckedShr<Shift = Self> {
    type Output: Maybe<Result = Self>;

    fn checked_shr(self, shift: Shift) -> Self::Output;
}

macro_rules! signed_checked_shr_impl {
    ($t:ty, $f:ty) => {
        impl CheckedShr<$f> for $t {
            type Output = Option<$t>;

            #[inline(always)]
            fn checked_shr(self, other: $f) -> Self::Output {
                if other < 0 {
                    None
                } else {
                    <$t>::checked_shr(self, u32::try_from(other).ok()?)
                }
            }
        }
    };
}

macro_rules! unsigned_checked_shr_impl {
    ($t:ty, $f:ty) => {
        impl CheckedShr<$f> for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_shr(self, other: $f) -> Self::Output {
                <$t>::checked_shr(self, u32::try_from(other).ok()?)
            }
        }
    };
}

macro_rules! plain_checked_shr_impl {
    ($($t:ty)*) => ($(
        signed_checked_shr_impl! { $t, i8 }
        signed_checked_shr_impl! { $t, i16 }
        signed_checked_shr_impl! { $t, i32 }
        signed_checked_shr_impl! { $t, i64 }
        signed_checked_shr_impl! { $t, i128 }
        signed_checked_shr_impl! { $t, isize }

        unsigned_checked_shr_impl! { $t, u8 }
        unsigned_checked_shr_impl! { $t, u16 }
        unsigned_checked_shr_impl! { $t, u32 }
        unsigned_checked_shr_impl! { $t, u64 }
        unsigned_checked_shr_impl! { $t, u128 }
        unsigned_checked_shr_impl! { $t, usize }
    )*)
}

plain_checked_shr_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait DivRemEuclid<Divisor = Self> {
    type Output;

    fn div_rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! plain_div_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl DivRemEuclid for $t {
            type Output = (Self, Self);

            #[inline(always)]
            fn div_rem_euclid(self, divisor: Self) -> Self::Output {
                (<$t>::div_euclid(self, divisor), <$t>::rem_euclid(self, divisor))
            }
        }
    )*)
}

plain_div_rem_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

pub trait Floor {
    type Output;

    fn floor(self) -> Self::Output;
}

macro_rules! plain_floor_impl {
    ($($t:ty)*) => ($(
        impl Floor for $t {
            type Output = $t;

            #[inline(always)]
            fn floor(self) -> Self::Output {
                <$t>::floor(self)
            }
        }
    )*)
}

plain_floor_impl!(f32 f64);

pub trait FrExp {
    type Output;

    fn frexp(self) -> Self::Output;
}

impl FrExp for f64 {
    type Output = (Self, i32);

    fn frexp(self) -> Self::Output {
        let bits = self.to_bits();
        let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
        if exponent_bits.is_zero() {
            if !self.is_zero() {
                const _0X1P64: f64 = f64::from_bits(0x43f0000000000000);
                let (fraction, exponent) = (self * _0X1P64).frexp();
                (fraction, exponent - 64)
            } else {
                (self, 0)
            }
        } else if exponent_bits == 0x7ff {
            (self, 0)
        } else {
            (
                f64::from_bits(bits & 0x800fffffffffffff | 0x3fe0000000000000),
                exponent_bits - 0x3fe,
            )
        }
    }
}

impl FrExp for f32 {
    type Output = (Self, i32);

    fn frexp(self) -> Self::Output {
        let bits = self.to_bits();
        let exponent_bits = ((bits >> 23) & 0xff) as i32;
        if exponent_bits.is_zero() {
            if !self.is_zero() {
                const _0X1P64: f32 = f32::from_bits(0x5f800000);
                let (fraction, exponent) = (self * _0X1P64).frexp();
                (fraction, exponent - 64)
            } else {
                (self, 0)
            }
        } else if exponent_bits == 0xff {
            (self, 0)
        } else {
            (
                f32::from_bits(bits & 0x807fffff | 0x3f000000),
                exponent_bits - 0x7e,
            )
        }
    }
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
                let mut first = self;
                let mut second = other;
                while !second.is_zero() {
                    (first, second) = (second, first.rem_euclid(second));
                }
                first
            }
        }
    )*)
}

plain_gcd_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait LdExp<Exponent> {
    type Output;

    fn ldexp(self, exponent: Exponent) -> Self::Output;
}

impl LdExp<i32> for f32 {
    type Output = Self;

    fn ldexp(mut self, mut exponent: i32) -> f32 {
        const _0X1P127: f32 = f32::from_bits(0x7f000000);
        const _0X1P_126: f32 = f32::from_bits(0x800000);
        const _0X1P24: f32 = f32::from_bits(0x4b800000);
        if exponent > 127 {
            self *= _0X1P127;
            exponent -= 127;
            if exponent > 127 {
                self *= _0X1P127;
                exponent -= 127;
                if exponent > 127 {
                    exponent = 127;
                }
            }
        } else if exponent < -126 {
            self *= _0X1P_126 * _0X1P24;
            exponent += 126 - 24;
            if exponent < -126 {
                self *= _0X1P_126 * _0X1P24;
                exponent += 126 - 24;
                if exponent < -126 {
                    exponent = -126;
                }
            }
        }
        self * f32::from_bits(((0x7f + exponent) as u32) << 23)
    }
}

impl LdExp<i32> for f64 {
    type Output = Self;

    fn ldexp(mut self, mut exponent: i32) -> Self::Output {
        if exponent > 1023 {
            const _0X1P1023: f64 = f64::from_bits(0x7fe0000000000000);
            self *= _0X1P1023;
            exponent -= 1023;
            if exponent > 1023 {
                self *= _0X1P1023;
                exponent -= 1023;
                if exponent > 1023 {
                    exponent = 1023;
                }
            }
        } else if exponent < -1022 {
            const _0X1P53: f64 = f64::from_bits(0x4340000000000000);
            const _0X1P_1022: f64 = f64::from_bits(0x0010000000000000);
            self *= _0X1P_1022 * _0X1P53;
            exponent += 1022 - 53;
            if exponent < -1022 {
                self *= _0X1P_1022 * _0X1P53;
                exponent += 1022 - 53;
                if exponent < -1022 {
                    exponent = -1022;
                }
            }
        }
        self * f64::from_bits(((0x3ff + exponent) as u64) << 52)
    }
}

pub trait MantissaDigits {
    const MANTISSA_DIGITS: usize;
}

macro_rules! plain_mantissa_digits_impl {
    ($($t:ty)*) => ($(
        impl MantissaDigits for $t {
            const MANTISSA_DIGITS: usize = <$t>::MANTISSA_DIGITS as usize;
        }
    )*)
}

plain_mantissa_digits_impl!(f32 f64);

pub trait MaxExp {
    const MAX_EXP: i32;
}

macro_rules! plain_max_exp_impl {
    ($($t:ty)*) => ($(
        impl MaxExp for $t {
            const MAX_EXP: i32 = <$t>::MAX_EXP;
        }
    )*)
}

plain_max_exp_impl!(f32 f64);

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
            Some(value) => panic!("called `Option::error()` on `Some{:?}` value", value),
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

macro_rules! plain_min_exp_impl {
    ($($t:ty)*) => ($(
        impl MinExp for $t {
            const MIN_EXP: i32 = <$t>::MIN_EXP;
        }
    )*)
}

plain_min_exp_impl!(f32 f64);

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

pub trait Parity {
    fn is_even(&self) -> bool;

    fn is_odd(&self) -> bool;
}

macro_rules! plain_parity_impl {
    ($($t:ty)*) => ($(
        impl Parity for $t {
            #[inline(always)]
            fn is_even(&self) -> bool {
                self & 1 == 0
            }

            #[inline(always)]
            fn is_odd(&self) -> bool {
                self & 1 == 1
            }
        }
    )*)
}

plain_parity_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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

#[derive(Clone, Eq, PartialEq)]
pub enum TieBreaking {
    AwayFromZero,
    ToEven,
    ToOdd,
    TowardZero,
}

pub trait Round {
    type Output;

    fn round(self, tie_breaking: TieBreaking) -> Self::Output;
}

macro_rules! plain_round_impl {
    ($($t:ty)*) => ($(
        impl Round for $t {
            type Output = $t;

            #[inline(always)]
            fn round(self, tie_breaking: TieBreaking) -> Self::Output {
                match tie_breaking {
                    TieBreaking::AwayFromZero => <$t>::round(self),
                    TieBreaking::ToEven => {
                        if self.ceil() - self == (0.5 as $t) {
                            2.0 * <$t>::round(self / 2.0)
                        } else {
                            <$t>::round(self)
                        }
                    }
                    TieBreaking::ToOdd => {
                        if self.ceil() - self == (0.5 as $t) {
                            2.0 * (self / 2.0).floor() + 1.0
                        } else {
                            <$t>::round(self)
                        }
                    }
                    TieBreaking::TowardZero => {
                        self.trunc() + ((self.fract().abs() > (0.5 as $t)) as i32 as $t) * self.signum()
                    }
                }
            }
        }
    )*)
}

plain_round_impl!(f32 f64);

pub trait Trunc {
    type Output;

    fn trunc(self) -> Self::Output;
}

macro_rules! plain_trunc_impl {
    ($($t:ty)*) => ($(
        impl Trunc for $t {
            type Output = $t;

            #[inline(always)]
            fn trunc(self) -> Self::Output {
                <$t>::trunc(self)
            }
        }
    )*)
}

plain_trunc_impl!(f32 f64);

pub trait Unitary {
    fn one() -> Self;

    fn is_one(&self) -> bool;
}

macro_rules! plain_unitary_impl {
    ($($t:ty)*) => ($(
        impl Unitary for $t {
            #[inline(always)]
            fn one() -> $t {1 as $t}

            #[inline(always)]
            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }
    )*)
}

plain_unitary_impl!(f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait WrappingSub<Subtrahend = Self> {
    type Output;

    fn wrapping_sub(self, subtrahend: Subtrahend) -> Self::Output;
}

macro_rules! plain_wrapping_sub_impl {
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

plain_wrapping_sub_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub trait Zeroable {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

macro_rules! plain_zero_impl {
    ($($t:ty)*) => ($(
        impl Zeroable for $t {
            #[inline(always)]
            fn zero() -> $t {0 as $t}

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    )*)
}

plain_zero_impl!(f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

pub type DoublePrecisionOf<T> = <T as DoublePrecision>::Result;
pub type OppositionOf<T> = <T as Oppose>::Result;
