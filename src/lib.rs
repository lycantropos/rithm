#![feature(destructuring_assignment)]
#![feature(option_result_unwrap_unchecked)]

use num::{Num, One, Signed as SignedNumber, Zero};
use pyo3::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::ffi::Py_hash_t;
use pyo3::prelude::{pyclass, pymethods, pymodule, pyproto, PyModule, PyResult, Python};
use pyo3::PyNumberProtocol;

use traits::SignedOf;

pub use crate::big_int::*;
pub use crate::traits::*;

mod big_int;
mod traits;
mod utils;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const BINARY_SHIFT: usize = (SignedOf::<Digit>::BITS - 2) as usize;

type _BigInt = BigInt<Digit, '_', BINARY_SHIFT>;

#[pyclass(name = "Int", module = "rithm", subclass)]
#[derive(Clone)]
struct PyInt(_BigInt);

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
        PyInt(self.0.gcd(&other.0))
    }
}

#[pyproto]
impl PyNumberProtocol for PyInt {
    fn __abs__(&self) -> PyInt {
        PyInt(self.0.abs())
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

#[pymodule]
fn _rithm(_py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr("__doc__", env!("CARGO_PKG_DESCRIPTION"))?;
    module.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyInt>()?;
    Ok(())
}
