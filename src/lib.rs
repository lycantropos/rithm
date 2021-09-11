#![feature(destructuring_assignment)]
#![feature(option_result_unwrap_unchecked)]
#![feature(trait_alias)]

use crate::traits::{Gcd, Modular, Oppositive, Unitary, Zeroable};
use pyo3::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::ffi::Py_hash_t;
use pyo3::prelude::{pyclass, pymethods, pymodule, pyproto, PyModule, PyResult, Python};
use pyo3::PyNumberProtocol;

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
            Err(reason) => Err(PyValueError::new_err(reason)),
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
            _numerator.map(|value| value.0).unwrap_or(_BigInt::zero()),
            _denominator.map(|value| value.0).unwrap_or(_BigInt::one()),
        ) {
            Ok(value) => Ok(PyFraction(value)),
            Err(reason) => Err(PyZeroDivisionError::new_err(reason)),
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
        match divmod(lhs.0, rhs.0) {
            Ok((quotient, remainder)) => Ok((PyInt(quotient), PyInt(remainder))),
            Err(reason) => Err(PyZeroDivisionError::new_err(reason)),
        }
    }

    fn __floordiv__(lhs: PyInt, rhs: PyInt) -> PyResult<PyInt> {
        match divmod(lhs.0, rhs.0) {
            Ok((result, _)) => Ok(PyInt(result)),
            Err(reason) => Err(PyZeroDivisionError::new_err(reason)),
        }
    }
    fn __mod__(lhs: PyInt, rhs: PyInt) -> PyResult<PyInt> {
        match divmod(lhs.0, rhs.0) {
            Ok((_, result)) => Ok(PyInt(result)),
            Err(reason) => Err(PyZeroDivisionError::new_err(reason)),
        }
    }

    fn __mul__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 * rhs.0)
    }

    fn __neg__(&self) -> PyInt {
        PyInt(-self.0.clone())
    }

    fn __sub__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 - rhs.0)
    }

    fn __truediv__(lhs: PyInt, rhs: PyInt) -> PyResult<PyFraction> {
        match _Fraction::new(lhs.0, rhs.0) {
            Ok(result) => Ok(PyFraction(result)),
            Err(reason) => Err(PyZeroDivisionError::new_err(reason)),
        }
    }
}

fn divmod(dividend: _BigInt, divisor: _BigInt) -> Result<(_BigInt, _BigInt), &'static str> {
    let (mut quotient, mut modulo) = dividend.divrem(&divisor)?;
    if (divisor.is_negative() && modulo.is_positive())
        || (divisor.is_positive() && modulo.is_negative())
    {
        quotient = quotient - _BigInt::one();
        modulo = modulo + divisor;
    }
    Ok((quotient, modulo))
}

#[pyproto]
impl PyObjectProtocol for PyInt {
    fn __bool__(self) -> bool {
        !self.0.is_zero()
    }

    fn __hash__(&self) -> Py_hash_t {
        self.0.hash() as Py_hash_t
    }

    fn __repr__(&self) -> String {
        format!("rithm.Int('{}')", self.0)
    }

    fn __richcmp__(&self, other: PyInt, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Ne => self.0 != other.0,
        }
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
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Ne => self.0 != other.0,
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
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
