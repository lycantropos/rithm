#![feature(destructuring_assignment)]
#![feature(option_result_unwrap_unchecked)]
#![feature(trait_alias)]

use crate::traits::{
    Abs, CheckedDiv, CheckedDivEuclid, CheckedRemEuclid, FromStrRadix, Gcd, Oppositive, Pow,
    Unitary, Zeroable,
};
use pyo3::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::ffi::Py_hash_t;
use pyo3::prelude::{pyclass, pymethods, pymodule, pyproto, PyModule, PyResult, Python};
use pyo3::PyNumberProtocol;
use std::convert::TryFrom;

pub mod big_int;
mod digits;
pub mod fraction;
pub mod traits;
mod utils;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const BINARY_SHIFT: usize = (traits::OppositionOf::<Digit>::BITS - 2) as usize;
const UNDEFINED_DIVISION_ERROR_MESSAGE: &str = "Division by zero is undefined.";

type _BigInt = big_int::BigInt<Digit, '_', BINARY_SHIFT>;
type _Fraction = fraction::Fraction<_BigInt>;

#[pyclass(name = "Int", module = "rithm", subclass)]
#[derive(Clone)]
struct PyInt(_BigInt);

#[pyclass(name = "Fraction", module = "rithm", subclass)]
#[derive(Clone)]
struct PyFraction(_Fraction);

#[pymethods]
impl PyInt {
    #[new]
    #[args(_string = "\"0\"", base = 10)]
    fn new(_string: &str, base: u32) -> PyResult<Self> {
        match _BigInt::from_str_radix(_string, base) {
            Ok(value) => Ok(PyInt(value)),
            Err(reason) => Err(PyValueError::new_err(reason.to_string())),
        }
    }

    fn gcd(&self, other: Self) -> PyInt {
        PyInt(self.0.clone().gcd(other.0))
    }
}

#[pymethods]
impl PyFraction {
    #[new]
    fn new(_numerator: Option<PyInt>, _denominator: Option<PyInt>) -> PyResult<Self> {
        match _Fraction::new(
            _numerator
                .map(|value| value.0)
                .unwrap_or_else(_BigInt::zero),
            _denominator
                .map(|value| value.0)
                .unwrap_or_else(_BigInt::one),
        ) {
            Some(value) => Ok(PyFraction(value)),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }

    #[getter]
    fn denominator(&self) -> PyInt {
        PyInt(self.0.denominator().clone())
    }

    #[getter]
    fn numerator(&self) -> PyInt {
        PyInt(self.0.numerator().clone())
    }
}

#[pyproto]
impl PyNumberProtocol for PyInt {
    fn __abs__(&self) -> PyInt {
        PyInt(self.0.clone().abs())
    }

    fn __add__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 + rhs.0)
    }

    fn __divmod__(lhs: PyInt, rhs: PyInt) -> PyResult<(PyInt, PyInt)> {
        match checked_div_rem_euclid(lhs.0, rhs.0) {
            Some((quotient, remainder)) => Ok((PyInt(quotient), PyInt(remainder))),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }

    fn __floordiv__(lhs: PyInt, rhs: PyInt) -> PyResult<PyInt> {
        match lhs.0.checked_div_euclid(rhs.0) {
            Some(result) => Ok(PyInt(result)),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }

    fn __invert__(&self) -> PyInt {
        PyInt(!self.0.clone())
    }

    fn __mod__(lhs: PyInt, rhs: PyInt) -> PyResult<PyInt> {
        match lhs.0.checked_rem_euclid(rhs.0) {
            Some(result) => Ok(PyInt(result)),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }

    fn __mul__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 * rhs.0)
    }

    fn __neg__(&self) -> PyInt {
        PyInt(-self.0.clone())
    }

    fn __pow__(lhs: PyInt, rhs: PyInt, _modulo: Option<PyInt>) -> PyInt {
        debug_assert!(_modulo.is_none());
        PyInt(lhs.0.pow(rhs.0))
    }

    fn __sub__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 - rhs.0)
    }

    fn __truediv__(lhs: PyInt, rhs: PyInt) -> PyResult<PyFraction> {
        match _Fraction::new(lhs.0, rhs.0) {
            Some(result) => Ok(PyFraction(result)),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }
}

fn checked_div_rem_euclid(dividend: _BigInt, divisor: _BigInt) -> Option<(_BigInt, _BigInt)> {
    let (mut quotient, mut modulo) = dividend.checked_div_rem(&divisor)?;
    if (divisor.is_negative() && modulo.is_positive())
        || (divisor.is_positive() && modulo.is_negative())
    {
        quotient -= _BigInt::one();
        modulo += divisor;
    }
    Some((quotient, modulo))
}

fn hash(value: &_BigInt) -> usize {
    #[cfg(target_arch = "x86")]
    const HASH_BITS: usize = 31;
    #[cfg(not(target_arch = "x86"))]
    const HASH_BITS: usize = 61;
    const HASH_MODULUS: usize = (1 << HASH_BITS) - 1;
    if value.digits().len() == 1 {
        return if value.is_negative() {
            usize::MAX
                - unsafe {
                    usize::try_from(
                        value.digits()[0] + <Digit as From<bool>>::from(value.digits()[0].is_one()),
                    )
                    .unwrap_unchecked()
                }
                + 1
        } else {
            unsafe { usize::try_from(value.digits()[0]).unwrap_unchecked() }
        };
    };
    let mut result = 0;
    for &position in value.digits().iter().rev() {
        result = ((result << BINARY_SHIFT) & HASH_MODULUS) | (result >> (HASH_BITS - BINARY_SHIFT));
        result += unsafe { usize::try_from(position).unwrap_unchecked() };
        if result >= HASH_MODULUS {
            result -= HASH_MODULUS;
        }
    }
    if value.is_negative() {
        result = usize::MAX - result + 1
    };
    result - ((result == usize::MAX) as usize)
}

#[pyproto]
impl PyObjectProtocol for PyInt {
    fn __bool__(self) -> bool {
        !self.0.is_zero()
    }

    fn __hash__(&self) -> Py_hash_t {
        hash(&self.0) as Py_hash_t
    }

    fn __repr__(&self) -> String {
        format!("rithm.Int('{}')", self.0)
    }

    fn __richcmp__(&self, other: PyInt, op: CompareOp) -> bool {
        compare(&self.0, &other.0, op)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

#[pyproto]
impl PyObjectProtocol for PyFraction {
    fn __bool__(self) -> bool {
        self.numerator().__bool__()
    }

    fn __repr__(&self) -> String {
        format!(
            "rithm.Fraction({}, {})",
            self.numerator().__repr__(),
            self.denominator().__repr__()
        )
    }

    fn __richcmp__(&self, other: PyFraction, op: CompareOp) -> bool {
        compare(&self.0, &other.0, op)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

#[pyproto]
impl PyNumberProtocol for PyFraction {
    fn __abs__(&self) -> PyFraction {
        PyFraction(self.0.clone().abs())
    }

    fn __add__(lhs: PyFraction, rhs: PyFraction) -> PyFraction {
        PyFraction(lhs.0 + rhs.0)
    }

    fn __mul__(lhs: PyFraction, rhs: PyFraction) -> PyFraction {
        PyFraction(lhs.0 * rhs.0)
    }

    fn __neg__(&self) -> PyFraction {
        PyFraction(-self.0.clone())
    }

    fn __sub__(lhs: PyFraction, rhs: PyFraction) -> PyFraction {
        PyFraction(lhs.0 - rhs.0)
    }

    fn __truediv__(lhs: PyFraction, rhs: PyFraction) -> PyResult<PyFraction> {
        match lhs.0.checked_div(rhs.0) {
            Some(result) => Ok(PyFraction(result)),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }
}

fn compare<T: PartialOrd>(left: &T, right: &T, op: CompareOp) -> bool {
    match op {
        CompareOp::Eq => left == right,
        CompareOp::Ge => left >= right,
        CompareOp::Gt => left > right,
        CompareOp::Le => left <= right,
        CompareOp::Lt => left < right,
        CompareOp::Ne => left != right,
    }
}

#[pymodule]
fn _rithm(_py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr("__doc__", env!("CARGO_PKG_DESCRIPTION"))?;
    module.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyInt>()?;
    module.add_class::<PyFraction>()?;
    Ok(())
}
