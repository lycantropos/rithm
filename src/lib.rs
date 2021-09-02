#![feature(destructuring_assignment)]
#![feature(option_result_unwrap_unchecked)]

use num::Zero;
use pyo3::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::ffi::Py_hash_t;
use pyo3::prelude::{pyclass, pymethods, pymodule, pyproto, PyModule, PyResult, Python};
use pyo3::PyNumberProtocol;

pub use crate::big_int::*;

mod big_int;
mod utils;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const BINARY_SHIFT: usize = (Digit::BITS - 1) as usize;

type _BigInt = BigInt<Digit, BINARY_SHIFT>;

#[pyclass(module = "rithm", subclass)]
#[derive(Clone)]
struct Int(_BigInt);

#[pymethods]
impl Int {
    #[new]
    #[args(_string = "\"0\"", base = 10)]
    fn new(_string: &str, base: u8) -> PyResult<Self> {
        Ok(Int {
            0: match _BigInt::new(_string, base) {
                Ok(value) => Ok(value),
                Err(reason) => Err(PyValueError::new_err(reason)),
            }?,
        })
    }
}

#[pyproto]
impl PyNumberProtocol for Int {
    fn __abs__(&self) -> Int {
        Int {
            0: self.0.clone().abs(),
        }
    }

    fn __add__(lhs: Int, rhs: Int) -> Int {
        Int { 0: lhs.0 + rhs.0 }
    }

    fn __neg__(&self) -> Int {
        Int { 0: -self.0.clone() }
    }

    fn __sub__(lhs: Int, rhs: Int) -> Int {
        Int { 0: lhs.0 - rhs.0 }
    }
}

#[pyproto]
impl PyObjectProtocol for Int {
    fn __bool__(self) -> bool {
        !self.0.is_zero()
    }

    fn __hash__(&self) -> Py_hash_t {
        self.0.hash() as Py_hash_t
    }

    fn __repr__(&self) -> String {
        format!("rithm.Int('{}')", self.0)
    }

    fn __richcmp__(&self, other: Int, op: CompareOp) -> bool {
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
    module.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    module.setattr("__doc__", env!("CARGO_PKG_DESCRIPTION"))?;
    module.add_class::<Int>()?;
    Ok(())
}
