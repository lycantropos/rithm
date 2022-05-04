use std::cmp::Ordering;
use std::convert::{FloatToInt, TryFrom, TryInto};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::big_int::BigInt;
use crate::digits::{
    AdditiveDigit, DigitConvertibleFromFloat, EuclidDivisibleDigit, GcdDigit, LeftShiftableDigit,
    MultiplicativeDigit, UnitaryDigit,
};
use crate::traits::{
    Abs, AdditiveMonoid, Ceil, CheckedDiv, CheckedDivAsF32, CheckedDivAsF64, CheckedDivEuclid,
    CheckedDivRemEuclid, CheckedPow, CheckedRemEuclid, CheckedShl, DivisivePartialMagma, Float,
    Floor, GcdMagma, Maybe, ModularUnaryAlgebra, MultiplicativeMonoid, NegatableUnaryAlgebra,
    Oppositive, Parity, Pow, Round, SubtractiveMagma, TieBreaking, Trunc, Unitary, Zeroable,
};
use crate::utils;

#[derive(Clone, Eq, PartialEq)]
pub struct Fraction<Component: Clone + Eq> {
    numerator: Component,
    denominator: Component,
}
#[derive(Eq, PartialEq)]
pub enum FromFloatConversionError {
    Infinity,
    NaN,
    OutOfBounds,
}

impl FromFloatConversionError {
    fn description(&self) -> &str {
        match self {
            FromFloatConversionError::Infinity => "Conversion of infinity is undefined.",
            FromFloatConversionError::NaN => "Conversion of NaN is undefined.",
            FromFloatConversionError::OutOfBounds => "Value is out of bounds.",
        }
    }
}

impl Debug for FromFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for FromFloatConversionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

impl<Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive> Fraction<Component> {
    pub fn new(mut numerator: Component, mut denominator: Component) -> Option<Self> {
        if denominator.is_zero() {
            None
        } else {
            (numerator, denominator) = normalize_components_sign(numerator, denominator);
            (numerator, denominator) = normalize_components_moduli(numerator, denominator);
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

impl<Component: Clone + Eq + Unitary> From<Component> for Fraction<Component> {
    fn from(value: Component) -> Self {
        Self {
            numerator: value,
            denominator: Component::one(),
        }
    }
}

impl<Component: Clone + Eq> Fraction<Component> {
    pub fn denominator(&self) -> &Component {
        &self.denominator
    }

    pub fn numerator(&self) -> &Component {
        &self.numerator
    }
}

impl<Component: Clone + Eq + ModularUnaryAlgebra> Abs for Fraction<Component> {
    type Output = Self;

    fn abs(self) -> <Self as Abs>::Output {
        Self {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}

impl<
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid,
    > Add for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator * other.denominator.clone() + other.numerator * self.denominator.clone(),
            self.denominator * other.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid,
    > Add<Component> for Fraction<Component>
{
    type Output = Self;

    fn add(self, other: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator + other * self.denominator.clone(),
            self.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

macro_rules! plain_add_fraction_impl {
    ($($t:ty)*) => ($(
    impl Add<Fraction<Self>> for $t {
        type Output = Fraction<Self>;

        fn add(self, other: Fraction<Self>) -> Self::Output {
            other + self
        }
    }
    )*)
}

plain_add_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<Digit: AdditiveDigit + Eq + GcdDigit, const SEPARATOR: char, const SHIFT: usize>
    Add<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn add(self, other: Fraction<Self>) -> Self::Output {
        other + self
    }
}

impl<
        Component: AdditiveMonoid
            + Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid,
    > AddAssign for Fraction<Component>
{
    fn add_assign(&mut self, other: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() * other.denominator.clone()
                + other.numerator * self.denominator.clone(),
            self.denominator.clone() * other.denominator,
        );
    }
}

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + Eq
            + MultiplicativeMonoid
            + NegatableUnaryAlgebra
            + Zeroable,
    > Ceil for Fraction<Component>
{
    type Output = Component;

    fn ceil(self) -> Self::Output {
        -unsafe {
            (-self.numerator)
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > CheckedDiv for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Self) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend_numerator, divisor_numerator) =
            normalize_components_moduli(self.numerator, divisor.numerator);
        let (dividend_denominator, divisor_denominator) =
            normalize_components_moduli(self.denominator, divisor.denominator);
        let (numerator, denominator) = normalize_components_sign(
            dividend_numerator * divisor_denominator,
            dividend_denominator * divisor_numerator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > CheckedDiv<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_div(self, divisor: Component) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend_numerator, divisor_numerator) =
            normalize_components_moduli(self.numerator, divisor);
        let (numerator, denominator) =
            normalize_components_sign(dividend_numerator, self.denominator * divisor_numerator);
        Some(Self {
            numerator,
            denominator,
        })
    }
}

macro_rules! plain_checked_div_fraction_impl {
    ($($t:ty)*) => ($(
    impl CheckedDiv<Fraction<Self>> for $t
    {
        type Output = Option<Fraction<Self>>;

        fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
            if divisor.is_zero() {
                return None;
            }
            let (dividend, divisor_numerator) = normalize_components_moduli(self, divisor.numerator);
            let (numerator, denominator) =
                normalize_components_sign(dividend * divisor.denominator, divisor_numerator);
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
    )*)
}

plain_checked_div_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<Digit: Eq + GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    CheckedDiv<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Fraction<Self>>;

    fn checked_div(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            return None;
        }
        let (dividend, divisor_numerator) = normalize_components_moduli(self, divisor.numerator);
        let (numerator, denominator) =
            normalize_components_sign(dividend * divisor.denominator, divisor_numerator);
        Some(Fraction::<Self> {
            numerator,
            denominator,
        })
    }
}

impl<
        Component: Clone + CheckedDivEuclid<Output = Option<Component>> + Eq + MultiplicativeMonoid + Zeroable,
    > CheckedDivEuclid for Fraction<Component>
{
    type Output = Option<Component>;

    fn checked_div_euclid(self, divisor: Self) -> Self::Output {
        (self.numerator * divisor.denominator)
            .checked_div_euclid(divisor.numerator * self.denominator)
    }
}

impl<
        Component: Clone + CheckedDivEuclid<Output = Option<Component>> + Eq + MultiplicativeMonoid,
    > CheckedDivEuclid<Component> for Fraction<Component>
{
    type Output = Option<Component>;

    fn checked_div_euclid(self, divisor: Component) -> Self::Output {
        self.numerator
            .checked_div_euclid(divisor * self.denominator)
    }
}

macro_rules! plain_checked_div_euclid_fraction_impl {
    ($($t:ty)*) => ($(
    impl CheckedDivEuclid<Fraction<Self>> for $t
    {
        type Output = Option<Self>;

        fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
            (self * divisor.denominator).checked_div_euclid(divisor.numerator)
        }
    }
    )*)
}

plain_checked_div_euclid_fraction_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl<
        Digit: Eq + EuclidDivisibleDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Self>;

    fn checked_div_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        (self * divisor.denominator).checked_div_euclid(divisor.numerator)
    }
}

impl<
        Component: Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + MultiplicativeMonoid
            + Zeroable,
    > CheckedDivRemEuclid for Fraction<Component>
{
    type Output = Option<(Component, Self)>;

    fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
        let (quotient, remainder_numerator) = (self.numerator * divisor.denominator.clone())
            .checked_div_rem_euclid(divisor.numerator * self.denominator.clone())?;
        let (remainder_numerator, remainder_denominator) = normalize_components_moduli(
            remainder_numerator,
            self.denominator * divisor.denominator,
        );
        Some((
            quotient,
            Self {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<
        Component: Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + MultiplicativeMonoid
            + Zeroable,
    > CheckedDivRemEuclid<Component> for Fraction<Component>
{
    type Output = Option<(Component, Self)>;

    fn checked_div_rem_euclid(self, divisor: Component) -> Self::Output {
        let (quotient, remainder_numerator) =
            (self.numerator).checked_div_rem_euclid(divisor * self.denominator.clone())?;
        let (remainder_numerator, remainder_denominator) =
            normalize_components_moduli(remainder_numerator, self.denominator);
        Some((
            quotient,
            Self {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

macro_rules! plain_checked_div_rem_euclid_fraction_impl {
    ($($t:ty)*) => ($(
    impl CheckedDivRemEuclid<Fraction<Self>> for $t
    {
        type Output = Option<(Self, Fraction<Self>)>;

        fn checked_div_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
            let (quotient, remainder_numerator) = (self * divisor.denominator.clone())
                .checked_div_rem_euclid(divisor.numerator)?;
            let (remainder_numerator, remainder_denominator) = normalize_components_moduli(
                remainder_numerator, divisor.denominator,
            );
            Some((
                quotient,
                Fraction::<Self> {
                    numerator: remainder_numerator,
                    denominator: remainder_denominator,
                },
            ))
        }
    }
    )*)
}

plain_checked_div_rem_euclid_fraction_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl<
        Digit: Eq + EuclidDivisibleDigit + GcdDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedDivRemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<(Self, Fraction<Self>)>;

    fn checked_div_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        let (quotient, remainder_numerator) =
            (self * divisor.denominator.clone()).checked_div_rem_euclid(divisor.numerator)?;
        let (remainder_numerator, remainder_denominator) =
            normalize_components_moduli(remainder_numerator, divisor.denominator);
        Some((
            quotient,
            Fraction::<Self> {
                numerator: remainder_numerator,
                denominator: remainder_denominator,
            },
        ))
    }
}

impl<
        Component: Clone
            + Eq
            + Oppositive
            + CheckedPow<Component, Output = Option<Component>>
            + Unitary
            + Zeroable,
    > CheckedPow<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                None
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = normalize_components_sign(
                    self.denominator.checked_pow(exponent.clone())?,
                    self.numerator.checked_pow(exponent)?,
                );
                Some(Self {
                    numerator,
                    denominator,
                })
            }
        } else {
            Some(Self {
                numerator: unsafe {
                    self.numerator
                        .checked_pow(exponent.clone())
                        .unwrap_unchecked()
                },
                denominator: unsafe { self.denominator.checked_pow(exponent).unwrap_unchecked() },
            })
        }
    }
}

macro_rules! plain_fraction_checked_pow_impl {
    ($($t:ty)*) => ($(
        impl CheckedPow<u32> for Fraction<$t> {
            type Output = Option<Self>;

            fn checked_pow(self, exponent: u32) -> Self::Output {
                Some(Self {
                    numerator: self.numerator.checked_pow(exponent)?,
                    denominator: self.denominator.checked_pow(exponent)?,
                })
            }
        }
        )*)
}

plain_fraction_checked_pow_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + MultiplicativeMonoid
            + Zeroable,
    > CheckedRemEuclid for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            (self.numerator * divisor.denominator.clone())
                .checked_rem_euclid(divisor.numerator * self.denominator.clone())?,
            self.denominator * divisor.denominator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

impl<
        Component: Clone
            + CheckedRemEuclid<Output = Option<Component>>
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + MultiplicativeMonoid
            + Zeroable,
    > CheckedRemEuclid<Component> for Fraction<Component>
{
    type Output = Option<Self>;

    fn checked_rem_euclid(self, divisor: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator
                .checked_rem_euclid(divisor * self.denominator.clone())?,
            self.denominator,
        );
        Some(Self {
            numerator,
            denominator,
        })
    }
}

macro_rules! plain_checked_rem_euclid_fraction_impl {
    ($($t:ty)*) => ($(
        impl CheckedRemEuclid<Fraction<Self>> for $t
        {
            type Output = Option<Fraction<Self>>;

            fn checked_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
                if divisor.is_zero() {
                    None
                } else {
                    let (numerator, denominator) = normalize_components_moduli(
                        (self * divisor.denominator)
                            .checked_rem_euclid(divisor.numerator)?,
                        divisor.denominator,
                    );
                    Some(Fraction::<Self> {
                        numerator,
                        denominator,
                    })
                }
            }
        }
        )*)
}

plain_checked_rem_euclid_fraction_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

impl<
        Digit: Eq + EuclidDivisibleDigit + GcdDigit + MultiplicativeDigit,
        const SEPARATOR: char,
        const SHIFT: usize,
    > CheckedRemEuclid<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Option<Fraction<Self>>;

    fn checked_rem_euclid(self, divisor: Fraction<Self>) -> Self::Output {
        if divisor.is_zero() {
            None
        } else {
            let (numerator, denominator) = normalize_components_moduli(
                (self * divisor.denominator.clone()).checked_rem_euclid(divisor.numerator)?,
                divisor.denominator,
            );
            Some(Fraction::<Self> {
                numerator,
                denominator,
            })
        }
    }
}

impl<Component: Clone + Debug + Eq + Unitary> Debug for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(
                formatter,
                "{}::from({:?})",
                std::any::type_name::<Self>(),
                self.numerator
            )
        } else {
            write!(
                formatter,
                "{}::new({:?}, {:?}).unwrap()",
                std::any::type_name::<Self>(),
                self.numerator,
                self.denominator
            )
        }
    }
}

impl<Component: Clone + Display + Eq + Unitary> Display for Fraction<Component> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > Div for Fraction<Component>
{
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        self.checked_div(divisor).unwrap()
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > Div<Component> for Fraction<Component>
{
    type Output = Self;

    fn div(self, divisor: Component) -> Self::Output {
        self.checked_div(divisor).unwrap()
    }
}

macro_rules! plain_div_fraction_impl {
    ($($t:ty)*) => ($(
    impl Div<Fraction<Self>> for $t
    {
        type Output = Fraction<Self>;

        fn div(self, divisor: Fraction<Self>) -> Self::Output {
            <$t as CheckedDiv<Fraction<Self>>>::checked_div(self, divisor).unwrap()
        }
    }
    )*)
}

plain_div_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<Digit: Eq + GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Div<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn div(self, divisor: Fraction<Self>) -> Self::Output {
        self.checked_div(divisor).unwrap()
    }
}

impl<
        Component: Clone + CheckedDivEuclid<Output = Option<Component>> + Eq + MultiplicativeMonoid + Zeroable,
    > Floor for Fraction<Component>
{
    type Output = Component;

    fn floor(self) -> Self::Output {
        unsafe {
            self.numerator
                .checked_div_euclid(self.denominator)
                .unwrap_unchecked()
        }
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > Mul for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let (numerator, other_denominator) =
            normalize_components_moduli(self.numerator, other.denominator);
        let (other_numerator, denominator) =
            normalize_components_moduli(other.numerator, self.denominator);
        Self {
            numerator: numerator * other_numerator,
            denominator: denominator * other_denominator,
        }
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > Mul<Component> for Fraction<Component>
{
    type Output = Self;

    fn mul(self, other: Component) -> Self::Output {
        let (other, denominator) = normalize_components_moduli(other, self.denominator);
        Self {
            numerator: self.numerator * other,
            denominator,
        }
    }
}

macro_rules! plain_mul_fraction_impl {
    ($($t:ty)*) => ($(
    impl Mul<Fraction<Self>> for $t {
        type Output = Fraction<Self>;

        fn mul(self, other: Fraction<Self>) -> Self::Output {
            other * self
        }
    }
    )*)
}

plain_mul_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<Digit: Eq + GcdDigit + MultiplicativeDigit, const SEPARATOR: char, const SHIFT: usize>
    Mul<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn mul(self, other: Fraction<Self>) -> Self::Output {
        other * self
    }
}

impl<
        Component: Clone + DivisivePartialMagma + Eq + GcdMagma + Oppositive + MultiplicativeMonoid,
    > MulAssign for Fraction<Component>
{
    fn mul_assign(&mut self, other: Self) {
        let (numerator, other_denominator) =
            normalize_components_moduli(self.numerator.clone(), other.denominator);
        let (other_numerator, denominator) =
            normalize_components_moduli(other.numerator, self.denominator.clone());
        self.numerator = numerator * other_numerator;
        self.denominator = denominator * other_denominator;
    }
}

impl<Component: Clone + Eq + NegatableUnaryAlgebra> Neg for Fraction<Component> {
    type Output = Self;

    fn neg(self) -> <Self as Neg>::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<Component: Clone + Eq + Oppositive + Unitary> Oppositive for Fraction<Component> {
    fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }

    fn is_positive(&self) -> bool {
        self.numerator.is_positive()
    }
}

impl<Component: Clone + Eq + MultiplicativeMonoid + PartialOrd> Ord for Fraction<Component> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<Component: Clone + Eq + Unitary> PartialEq<Component> for Fraction<Component> {
    fn eq(&self, other: &Component) -> bool {
        self.denominator.is_one() && self.numerator.eq(other)
    }
}

macro_rules! plain_partial_eq_fraction_impl {
    ($($t:ty)*) => ($(
    impl PartialEq<Fraction<Self>> for $t {
        fn eq(&self, other: &Fraction<Self>) -> bool {
            other.denominator.is_one() && other.numerator.eq(self)
        }
    }
    )*)
}

plain_partial_eq_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<Digit: Clone + Eq + UnitaryDigit, const SEPARATOR: char, const SHIFT: usize>
    PartialEq<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn eq(&self, other: &Fraction<Self>) -> bool {
        other.denominator.is_one() && other.numerator.eq(self)
    }
}

impl<Component: Clone + Eq + MultiplicativeMonoid + PartialOrd> PartialOrd for Fraction<Component> {
    fn ge(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            >= other.numerator.clone() * self.denominator.clone()
    }

    fn gt(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            > other.numerator.clone() * self.denominator.clone()
    }

    fn le(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            <= other.numerator.clone() * self.denominator.clone()
    }

    fn lt(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            < other.numerator.clone() * self.denominator.clone()
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl<Component: Clone + Eq + MultiplicativeMonoid + PartialOrd> PartialOrd<Component>
    for Fraction<Component>
{
    fn ge(&self, other: &Component) -> bool {
        self.numerator >= other.clone() * self.denominator.clone()
    }

    fn gt(&self, other: &Component) -> bool {
        self.numerator > other.clone() * self.denominator.clone()
    }

    fn le(&self, other: &Component) -> bool {
        self.numerator <= other.clone() * self.denominator.clone()
    }

    fn lt(&self, other: &Component) -> bool {
        self.numerator < other.clone() * self.denominator.clone()
    }

    fn partial_cmp(&self, other: &Component) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

macro_rules! plain_partial_ord_fraction_impl {
    ($($t:ty)*) => ($(
    impl PartialOrd<Fraction<Self>> for $t
    {
        fn ge(&self, other: &Fraction<Self>) -> bool {
            self.clone() * other.denominator.clone() >= other.numerator
        }

        fn gt(&self, other: &Fraction<Self>) -> bool {
            self.clone() * other.denominator.clone() > other.numerator
        }

        fn le(&self, other: &Fraction<Self>) -> bool {
            self.clone() * other.denominator.clone() <= other.numerator
        }

        fn lt(&self, other: &Fraction<Self>) -> bool {
            self.clone() * other.denominator.clone() < other.numerator
        }

        fn partial_cmp(&self, other: &Fraction<Self>) -> Option<Ordering> {
            Some(if self.lt(other) {
                Ordering::Less
            } else if self.gt(other) {
                Ordering::Greater
            } else {
                Ordering::Equal
            })
        }
    }
    )*)
}

plain_partial_ord_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<
        Digit: Clone + Eq + GcdDigit + MultiplicativeDigit + PartialOrd,
        const SEPARATOR: char,
        const SHIFT: usize,
    > PartialOrd<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn ge(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() >= other.numerator
    }

    fn gt(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() > other.numerator
    }

    fn le(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() <= other.numerator
    }

    fn lt(&self, other: &Fraction<Self>) -> bool {
        self.clone() * other.denominator.clone() < other.numerator
    }

    fn partial_cmp(&self, other: &Fraction<Self>) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl<
        Component: Clone + Eq + Oppositive + Pow<Component, Output = Component> + Unitary + Zeroable,
    > Pow<Component> for Fraction<Component>
{
    type Output = Self;

    fn pow(self, exponent: Component) -> Self::Output {
        if exponent.is_negative() {
            if self.is_zero() {
                panic!("Division by zero is undefined.")
            } else {
                let exponent = -exponent;
                let (numerator, denominator) = normalize_components_sign(
                    self.denominator.pow(exponent.clone()),
                    self.numerator.pow(exponent),
                );
                Self {
                    numerator,
                    denominator,
                }
            }
        } else {
            Self {
                numerator: self.numerator.pow(exponent.clone()),
                denominator: self.denominator.pow(exponent),
            }
        }
    }
}

impl<
        Component: AdditiveMonoid
            + Clone
            + CheckedDivRemEuclid<Output = Option<(Component, Component)>>
            + Eq
            + From<u8>
            + MultiplicativeMonoid
            + Oppositive
            + Ord
            + Parity
            + SubtractiveMagma,
    > Round for Fraction<Component>
{
    type Output = Component;

    fn round(self, tie_breaking: TieBreaking) -> Self::Output {
        let (quotient, remainder) = unsafe {
            self.numerator
                .checked_div_rem_euclid(self.denominator.clone())
                .unwrap_unchecked()
        };
        match (remainder * Component::from(2u8)).cmp(&self.denominator) {
            Ordering::Equal => {
                if match tie_breaking {
                    TieBreaking::AwayFromZero => !quotient.is_negative(),
                    TieBreaking::ToEven => quotient.is_odd(),
                    TieBreaking::ToOdd => quotient.is_even(),
                    TieBreaking::TowardZero => quotient.is_negative(),
                } {
                    quotient + Component::one()
                } else {
                    quotient
                }
            }
            Ordering::Greater => quotient + Component::one(),
            Ordering::Less => quotient,
        }
    }
}

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > Sub for Fraction<Component>
{
    type Output = Self;

    fn sub(self, subtrahend: Self) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator * subtrahend.denominator.clone()
                - subtrahend.numerator * self.denominator.clone(),
            self.denominator * subtrahend.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > Sub<Component> for Fraction<Component>
{
    type Output = Self;

    fn sub(self, other: Component) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self.numerator - other * self.denominator.clone(),
            self.denominator,
        );
        Self {
            numerator,
            denominator,
        }
    }
}

macro_rules! plain_sub_fraction_impl {
    ($($t:ty)*) => ($(
    impl Sub<Fraction<Self>> for $t {
        type Output = Fraction<Self>;

        fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
            let (numerator, denominator) = normalize_components_moduli(
                self * subtrahend.denominator - subtrahend.numerator,
                subtrahend.denominator,
            );
            Self::Output {
                numerator,
                denominator,
            }
        }
    }
    )*)
}

plain_sub_fraction_impl!(i8 i16 i32 i64 i128 isize);

impl<Digit: AdditiveDigit + Eq + GcdDigit, const SEPARATOR: char, const SHIFT: usize>
    Sub<Fraction<Self>> for BigInt<Digit, SEPARATOR, SHIFT>
{
    type Output = Fraction<Self>;

    fn sub(self, subtrahend: Fraction<Self>) -> Self::Output {
        let (numerator, denominator) = normalize_components_moduli(
            self * subtrahend.denominator.clone() - subtrahend.numerator,
            subtrahend.denominator,
        );
        Self::Output {
            numerator,
            denominator,
        }
    }
}

impl<
        Component: Clone
            + DivisivePartialMagma
            + Eq
            + GcdMagma
            + Oppositive
            + MultiplicativeMonoid
            + SubtractiveMagma,
    > SubAssign for Fraction<Component>
{
    fn sub_assign(&mut self, subtrahend: Self) {
        (self.numerator, self.denominator) = normalize_components_moduli(
            self.numerator.clone() * subtrahend.denominator.clone()
                - subtrahend.numerator * self.denominator.clone(),
            self.denominator.clone() * subtrahend.denominator,
        );
    }
}

impl<
        Component: Clone
            + CheckedDivEuclid<Output = Option<Component>>
            + Eq
            + MultiplicativeMonoid
            + Oppositive,
    > Trunc for Fraction<Component>
{
    type Output = Component;

    fn trunc(self) -> Self::Output {
        if self.is_negative() {
            self.ceil()
        } else {
            self.floor()
        }
    }
}

impl<Component: Clone + Eq + CheckedDivAsF32> TryInto<f32> for Fraction<Component> {
    type Error = <<Component as CheckedDivAsF32>::Output as Maybe>::Error;

    fn try_into(self) -> Result<f32, Self::Error> {
        let maybe = self.numerator.checked_div_as_f32(self.denominator);
        if maybe.is_result() {
            Ok(maybe.result())
        } else {
            Err(maybe.error())
        }
    }
}

impl<Component: Clone + Eq + CheckedDivAsF64> TryInto<f64> for Fraction<Component> {
    type Error = <<Component as CheckedDivAsF64>::Output as Maybe>::Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        let maybe = self.numerator.checked_div_as_f64(self.denominator);
        if maybe.is_result() {
            Ok(maybe.result())
        } else {
            Err(maybe.error())
        }
    }
}

impl<Component: Clone + Eq + Unitary> Unitary for Fraction<Component> {
    fn one() -> Self {
        Self {
            numerator: Component::one(),
            denominator: Component::one(),
        }
    }

    fn is_one(&self) -> bool {
        self.numerator.is_one() && self.denominator.is_one()
    }
}

impl<Component: Clone + Eq + Unitary + Zeroable> Zeroable for Fraction<Component> {
    fn zero() -> Self {
        Self {
            numerator: Component::zero(),
            denominator: Component::one(),
        }
    }

    fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
}

macro_rules! big_int_try_from_float_impl {
    ($($f:ty)*) => ($(
        impl<
                Digit: DigitConvertibleFromFloat + Eq + LeftShiftableDigit + UnitaryDigit,
                const SEPARATOR: char,
                const SHIFT: usize,
            > TryFrom<$f> for Fraction<BigInt<Digit, SEPARATOR, SHIFT>>
        where
            $f: FloatToInt<BigInt<Digit, SEPARATOR, SHIFT>>,
        {
            type Error = FromFloatConversionError;

            fn try_from(value: $f) -> Result<Self, Self::Error> {
                if value.is_infinite() {
                    Err(FromFloatConversionError::Infinity)
                } else if value.is_nan() {
                    Err(FromFloatConversionError::NaN)
                } else {
                    let (mut fraction, mut exponent) = value.frexp();
                    for _ in 0..300 {
                        if fraction == fraction.floor() {
                            break;
                        }
                        fraction *= 2.0 as $f;
                        exponent -= 1;
                    }
                    let mut numerator =
                        unsafe { fraction.to_int_unchecked::<BigInt<Digit, SEPARATOR, SHIFT>>() };
                    let mut denominator = BigInt::<Digit, SEPARATOR, SHIFT>::one();
                    if exponent.is_negative() {
                        denominator = denominator.checked_shl((-exponent) as u32).result();
                    } else {
                        numerator = numerator.checked_shl(exponent as u32).result();
                    }
                    Ok(Self {
                        numerator,
                        denominator,
                    })
                }
            }
        }
    )*)
}

big_int_try_from_float_impl!(f32 f64);

macro_rules! plain_try_from_float_impl {
    ($f:ty => $($t:ty)*) => ($(
        impl TryFrom<$f> for Fraction<$t> {
            type Error = FromFloatConversionError;

            fn try_from(value: $f) -> Result<Self, Self::Error> {
                if value.is_infinite() {
                    Err(FromFloatConversionError::Infinity)
                } else if value.is_nan() {
                    Err(FromFloatConversionError::NaN)
                } else if value.round() < (<$t>::MIN as $f) || value.round() > (<$t>::MAX as $f) {
                    Err(FromFloatConversionError::OutOfBounds)
                } else {
                    let (mut fraction, mut exponent) = value.frexp();
                    const MAX_EXPONENT_MODULUS: u32 = <$t>::BITS - 1 - (utils::is_signed::<$t>() as u32);
                    if (exponent.abs() as u32) > MAX_EXPONENT_MODULUS {
                        if exponent.is_negative() {
                            fraction *= ((exponent + (MAX_EXPONENT_MODULUS as i32)) as $f).exp2();
                            exponent = -(MAX_EXPONENT_MODULUS as i32);
                        } else {
                            fraction *= ((exponent - (MAX_EXPONENT_MODULUS as i32)) as $f).exp2();
                            exponent = MAX_EXPONENT_MODULUS as i32;
                        };
                    }
                    while fraction != fraction.floor()
                        && (fraction.round() as $t) >= <$t>::MIN / 2
                        && (fraction.round() as $t) <= <$t>::MAX / 2
                        && (!exponent.is_negative() || ((-exponent) as u32) < MAX_EXPONENT_MODULUS)
                    {
                        fraction *= 2.0 as $f;
                        exponent -= 1;
                    }
                    if exponent.is_negative() {
                        let (numerator, denominator) = normalize_components_moduli(
                            fraction.round() as $t,
                            <$t>::one() << ((-exponent) as u32),
                        );
                        Ok(Self {
                            numerator,
                            denominator,
                        })
                    } else {
                        Ok(Self {
                            numerator: value.round() as $t,
                            denominator: <$t>::one(),
                        })
                    }
                }
            }
        }
    )*)
}

plain_try_from_float_impl!(f32 => i8 i16 i32 i64 u8 u16 u32 u64);
plain_try_from_float_impl!(f64 => i8 i16 i32 i64 u8 u16 u32 u64);

#[inline]
fn normalize_components_moduli<Component: Clone + DivisivePartialMagma + GcdMagma>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    let gcd = numerator.clone().gcd(denominator.clone());
    (numerator / gcd.clone(), denominator / gcd)
}

#[inline]
fn normalize_components_sign<Component: Oppositive>(
    numerator: Component,
    denominator: Component,
) -> (Component, Component) {
    if denominator.is_negative() {
        (-numerator, -denominator)
    } else {
        (numerator, denominator)
    }
}
