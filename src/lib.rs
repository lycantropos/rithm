#![feature(const_float_bits_conv)]
#![feature(const_fn_trait_bound)]
#![feature(convert_float_to_int)]
#![feature(destructuring_assignment)]
#![feature(option_result_unwrap_unchecked)]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(try_reserve)]

use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};

use pyo3::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::prelude::{pyclass, pymethods, pymodule, pyproto, PyModule, PyResult, Python};
use pyo3::types::{PyBytes, PyFloat, PyLong, PyString};
use pyo3::{ffi, AsPyPointer, Py, PyAny, PyErr, PyNativeType, PyRef, ToPyObject};
use pyo3::{IntoPy, PyNumberProtocol, PyObject};

use crate::traits::{
    Abs, BitLength, CheckedDiv, CheckedDivEuclid, CheckedDivRemEuclid, CheckedPow,
    CheckedPowRemEuclid, CheckedRemEuclid, CheckedShl, CheckedShr, FromStrRadix, Gcd, Oppositive,
    Pow, Unitary, Zeroable,
};

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
#[pyo3(text_signature = "(value=None, base=None, /)")]
#[derive(Clone)]
struct PyInt(_BigInt);

#[pyclass(name = "Fraction", module = "rithm", subclass)]
#[pyo3(text_signature = "(numerator=None, denominator=None, /)")]
#[derive(Clone)]
struct PyFraction(_Fraction);

#[pymethods]
impl PyInt {
    #[new]
    fn new(_value: Option<&PyAny>, _base: Option<u32>) -> PyResult<Self> {
        match _value {
            None => Ok(PyInt(_BigInt::zero())),
            Some(value) => {
                if _base.is_some() || value.is_instance::<PyString>()? {
                    match _BigInt::from_str_radix(value.extract::<&str>()?, _base.unwrap_or(10)) {
                        Ok(value) => Ok(PyInt(value)),
                        Err(reason) => Err(PyValueError::new_err(reason.to_string())),
                    }
                } else if value.is_instance::<PyInt>()? {
                    value.extract::<PyInt>()
                } else if value.is_instance::<PyFloat>()? {
                    Ok(PyInt(
                        _BigInt::try_from(value.extract::<&PyFloat>()?.value()).map_err(
                            |reason| match reason {
                                big_int::FromFloatConversionError::Infinity => {
                                    PyOverflowError::new_err(reason.to_string())
                                }
                                big_int::FromFloatConversionError::NaN => {
                                    PyValueError::new_err(reason.to_string())
                                }
                            },
                        )?,
                    ))
                } else {
                    let ptr = value.as_ptr();
                    let py = value.py();
                    unsafe {
                        let value = ffi::PyNumber_Index(ptr);
                        if value.is_null() {
                            return Err(PyErr::fetch(py));
                        }
                        let bits_count = ffi::_PyLong_NumBits(value);
                        match bits_count.cmp(&0) {
                            Ordering::Less => Err(PyErr::fetch(py)),
                            Ordering::Equal => Ok(PyInt(_BigInt::zero())),
                            Ordering::Greater => {
                                let bytes_count = (bits_count as usize) / (u8::BITS as usize) + 1;
                                let mut buffer = vec![0u8; bytes_count];
                                if ffi::_PyLong_AsByteArray(
                                    Py::<PyLong>::from_owned_ptr(py, value).as_ptr()
                                        as *mut ffi::PyLongObject,
                                    buffer.as_mut_ptr(),
                                    buffer.len(),
                                    1,
                                    1,
                                ) < 0
                                {
                                    Err(PyErr::fetch(py))
                                } else {
                                    Ok(PyInt(_BigInt::from_bytes(buffer)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn bit_length(&self) -> PyInt {
        PyInt(self.0.clone().bit_length())
    }

    #[pyo3(text_signature = "($self, other, /)")]
    fn gcd(&self, other: Self) -> PyInt {
        PyInt(self.0.clone().gcd(other.0))
    }

    fn __ceil__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __floor__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __getstate__(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.0.as_bytes()).to_object(py)
    }

    fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        state.extract::<&PyBytes>(py).map(|py_bytes| {
            self.0 = _BigInt::from_bytes(py_bytes.extract().unwrap());
        })
    }

    fn __trunc__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }
}

#[inline]
fn big_int_to_py_long(value: &_BigInt) -> PyObject {
    let buffer = value.as_bytes();
    Python::with_gil(|py| unsafe {
        PyObject::from_owned_ptr(
            py,
            ffi::_PyLong_FromByteArray(buffer.as_ptr(), buffer.len(), 1, 1),
        )
    })
}

#[pymethods]
impl PyFraction {
    #[new]
    fn new(_numerator: Option<&PyAny>, _denominator: Option<PyInt>) -> PyResult<Self> {
        match _denominator {
            Some(denominator) => match _Fraction::new(
                _numerator
                    .and_then(|value| value.extract::<PyInt>().ok())
                    .map(|value| value.0)
                    .unwrap_or_else(_BigInt::zero),
                denominator.0,
            ) {
                Some(value) => Ok(PyFraction(value)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            },
            None => Ok(PyFraction(match _numerator {
                Some(value) => {
                    if value.is_instance::<PyInt>()? {
                        _Fraction::new(value.extract::<PyInt>()?.0, _BigInt::one()).unwrap()
                    } else {
                        _Fraction::try_from(value.extract::<f64>()?).map_err(
                            |reason| match reason {
                                fraction::FromFloatConversionError::NaN => {
                                    PyValueError::new_err(reason.to_string())
                                }
                                _ => PyOverflowError::new_err(reason.to_string()),
                            },
                        )?
                    }
                }
                None => _Fraction::zero(),
            })),
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

    fn __getstate__(&self, py: Python) -> PyObject {
        (
            self.numerator().__getstate__(py),
            self.denominator().__getstate__(py),
        )
            .to_object(py)
    }

    fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        state.extract::<(PyObject, PyObject)>(py).and_then(
            |(numerator_state, denominator_state)| {
                let mut numerator = PyInt(_BigInt::zero());
                numerator.__setstate__(py, numerator_state)?;
                let mut denominator = PyInt(_BigInt::zero());
                denominator.__setstate__(py, denominator_state)?;
                self.0 = unsafe { _Fraction::new(numerator.0, denominator.0).unwrap_unchecked() };
                Ok(())
            },
        )
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

    fn __and__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 & rhs.0)
    }

    fn __divmod__(lhs: PyInt, rhs: PyInt) -> PyResult<(PyInt, PyInt)> {
        match lhs.0.checked_div_rem_euclid(rhs.0) {
            Some((quotient, remainder)) => Ok((PyInt(quotient), PyInt(remainder))),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }

    fn __float__(&self) -> PyResult<PyObject> {
        match f64::try_from(self.0.clone()) {
            Ok(value) => Ok(Python::with_gil(|py| value.into_py(py))),
            Err(reason) => Err(PyOverflowError::new_err(reason.to_string())),
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

    fn __int__(&self) -> PyObject {
        big_int_to_py_long(&self.0)
    }

    fn __invert__(&self) -> PyInt {
        PyInt(!self.0.clone())
    }

    fn __lshift__(lhs: PyInt, rhs: PyInt) -> PyResult<PyInt> {
        lhs.0
            .checked_shl(rhs.0)
            .map(PyInt)
            .map_err(|reason| match reason {
                big_int::LeftShiftError::NegativeShift => PyValueError::new_err(reason.to_string()),
                big_int::LeftShiftError::OutOfMemory => PyMemoryError::new_err(reason.to_string()),
                big_int::LeftShiftError::TooLarge => PyOverflowError::new_err(reason.to_string()),
            })
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

    fn __or__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 | rhs.0)
    }

    fn __pow__(lhs: PyInt, rhs: PyInt, divisor: Option<PyInt>) -> PyResult<PyObject> {
        match divisor {
            Some(value) => {
                let is_zero_divisor = value.0.is_zero();
                match lhs.0.checked_pow_rem_euclid(rhs.0, value.0) {
                    Some(value) => Ok(to_py_object(PyInt(value))),
                    None => Err(PyValueError::new_err(if is_zero_divisor {
                        "Divisor cannot be zero."
                    } else {
                        "Base is not invertible for the given divisor."
                    })),
                }
            }
            None => Ok({
                if rhs.0.is_negative() {
                    to_py_object(match unsafe {
                        _Fraction::new(lhs.0, _BigInt::one()).unwrap_unchecked()
                    }
                    .checked_pow(rhs.0)
                    {
                        Some(value) => Ok(PyFraction(value)),
                        None => Err(PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )),
                    }?)
                } else {
                    to_py_object(PyInt(lhs.0.pow(rhs.0)))
                }
            }),
        }
    }

    fn __rshift__(lhs: PyInt, rhs: PyInt) -> PyResult<PyInt> {
        lhs.0
            .checked_shr(rhs.0)
            .map(PyInt)
            .map_err(|reason| match reason {
                big_int::RightShiftError::NegativeShift => {
                    PyValueError::new_err(reason.to_string())
                }
            })
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

    fn __xor__(lhs: PyInt, rhs: PyInt) -> PyInt {
        PyInt(lhs.0 ^ rhs.0)
    }
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
                    usize::try_from(value.digits()[0] + Digit::from(value.digits()[0].is_one()))
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

    fn __hash__(&self) -> ffi::Py_hash_t {
        hash(&self.0) as ffi::Py_hash_t
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
impl PyNumberProtocol for PyFraction {
    fn __abs__(&self) -> PyFraction {
        PyFraction(self.0.clone().abs())
    }

    fn __add__(lhs: PyFraction, rhs: &PyAny) -> PyResult<PyObject> {
        let py = rhs.py();
        if rhs.is_instance::<PyFraction>()? {
            Ok(PyFraction(lhs.0 + rhs.extract::<PyFraction>()?.0).into_py(py))
        } else if rhs.is_instance::<PyInt>()? {
            Ok(PyFraction(lhs.0 + rhs.extract::<PyInt>()?.0).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __float__(&self) -> PyResult<PyObject> {
        match <_Fraction as TryInto<f64>>::try_into(self.0.clone()) {
            Ok(value) => Ok(Python::with_gil(|py| value.into_py(py))),
            Err(reason) => Err(PyOverflowError::new_err(reason.to_string())),
        }
    }

    fn __floordiv__(lhs: PyFraction, rhs: &PyAny) -> PyResult<PyObject> {
        let py = rhs.py();
        if rhs.is_instance::<PyFraction>()? {
            match lhs.0.checked_div_euclid(rhs.extract::<PyFraction>()?.0) {
                Some(value) => Ok(PyInt(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else if rhs.is_instance::<PyInt>()? {
            match lhs.0.checked_div_euclid(rhs.extract::<PyInt>()?.0) {
                Some(value) => Ok(PyInt(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __mod__(lhs: PyFraction, rhs: &PyAny) -> PyResult<PyObject> {
        let py = rhs.py();
        if rhs.is_instance::<PyFraction>()? {
            match lhs
                .0
                .clone()
                .checked_rem_euclid(rhs.extract::<PyFraction>()?.0)
            {
                Some(value) => Ok(PyFraction(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else if rhs.is_instance::<PyInt>()? {
            match lhs.0.clone().checked_rem_euclid(rhs.extract::<PyInt>()?.0) {
                Some(value) => Ok(PyFraction(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __mul__(lhs: PyFraction, rhs: &PyAny) -> PyResult<PyObject> {
        let py = rhs.py();
        if rhs.is_instance::<PyFraction>()? {
            Ok(PyFraction(lhs.0 * rhs.extract::<PyFraction>()?.0).into_py(py))
        } else if rhs.is_instance::<PyInt>()? {
            Ok(PyFraction(lhs.0 * rhs.extract::<PyInt>()?.0).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __neg__(&self) -> PyFraction {
        PyFraction(-self.0.clone())
    }

    fn __pow__(lhs: PyFraction, rhs: PyInt, modulo: Option<PyInt>) -> PyResult<PyObject> {
        if modulo.is_some() {
            Ok(Python::with_gil(|py| py.NotImplemented()))
        } else {
            match lhs.0.checked_pow(rhs.0) {
                Some(value) => Ok(to_py_object(PyFraction(value))),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        }
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance::<PyInt>()? {
            Ok(PyFraction(other.extract::<PyInt>()?.0 + self.0.clone()).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rfloordiv__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance::<PyInt>()? {
            match other
                .extract::<PyInt>()?
                .0
                .checked_div_euclid(self.0.clone())
            {
                Some(value) => Ok(PyInt(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance::<PyInt>()? {
            Ok(PyFraction(other.extract::<PyInt>()?.0 * self.0.clone()).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rsub__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance::<PyInt>()? {
            Ok(PyFraction(other.extract::<PyInt>()?.0 - self.0.clone()).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rtruediv__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance::<PyInt>()? {
            match other.extract::<PyInt>()?.0.checked_div(self.0.clone()) {
                Some(value) => Ok(PyFraction(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __sub__(lhs: PyFraction, rhs: &PyAny) -> PyResult<PyObject> {
        let py = rhs.py();
        if rhs.is_instance::<PyFraction>()? {
            Ok(PyFraction(lhs.0 - rhs.extract::<PyFraction>()?.0).into_py(py))
        } else if rhs.is_instance::<PyInt>()? {
            Ok(PyFraction(lhs.0 - rhs.extract::<PyInt>()?.0).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __truediv__(lhs: PyFraction, rhs: &PyAny) -> PyResult<PyObject> {
        let py = rhs.py();
        if rhs.is_instance::<PyFraction>()? {
            match lhs.0.checked_div(rhs.extract::<PyFraction>()?.0) {
                Some(value) => Ok(PyFraction(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else if rhs.is_instance::<PyInt>()? {
            match lhs.0.checked_div(rhs.extract::<PyInt>()?.0) {
                Some(value) => Ok(PyFraction(value).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            Ok(py.NotImplemented())
        }
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

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance::<PyFraction>()? {
            Ok(compare(&self.0, &other.extract::<PyFraction>()?.0, op).into_py(py))
        } else if other.is_instance::<PyInt>()? {
            Ok(compare(&self.0, &other.extract::<PyInt>()?.0, op).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

fn compare<T: PartialOrd<U>, U>(left: &T, right: &U, op: CompareOp) -> bool {
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

fn to_py_object<T: IntoPy<PyObject>>(result: T) -> PyObject {
    Python::with_gil(|py| result.into_py(py))
}
