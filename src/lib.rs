#![feature(const_float_bits_conv)]
#![feature(convert_float_to_int)]
#![feature(specialization)]
#![feature(trait_alias)]

use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};

use pyo3::basic::CompareOp;
use pyo3::exceptions::*;
use pyo3::prelude::{pyclass, pymethods, pymodule, PyModule, PyResult, Python};
use pyo3::type_object::PyTypeObject;
use pyo3::types::{PyBytes, PyFloat, PyLong, PyString, PyType};
use pyo3::{intern, AsPyPointer, IntoPy, Py, PyAny, PyErr, PyObject, PyRef, ToPyObject};
use pyo3_ffi as ffi;

use crate::traits::{
    Abs, BitLength, Ceil, CheckedDiv, CheckedDivEuclid, CheckedDivRemEuclid, CheckedPow,
    CheckedPowRemEuclid, CheckedRemEuclid, CheckedShl, CheckedShr, Endianness, Floor, FromBytes,
    FromStrRadix, Gcd, Oppositive, Parity, Round, TieBreaking, ToBytes, Trunc, Unitary, Zeroable,
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
const PICKLE_SERIALIZATION_ENDIANNESS: Endianness = Endianness::Little;

type BigInt = big_int::BigInt<Digit, '_', BINARY_SHIFT>;
type Fraction = fraction::Fraction<BigInt>;

#[pyclass(name = "Endianness", module = "rithm")]
#[derive(Clone)]
struct PyEndianness(Endianness);

#[pyclass(name = "Fraction", module = "rithm", subclass)]
#[pyo3(text_signature = "(numerator=None, denominator=None, /)")]
#[derive(Clone)]
struct PyFraction(Fraction);

#[pyclass(name = "Int", module = "rithm", subclass)]
#[pyo3(text_signature = "(value=None, base=None, /)")]
#[derive(Clone)]
struct PyInt(BigInt);

#[pyclass(name = "TieBreaking", module = "rithm")]
#[derive(Clone)]
struct PyTieBreaking(TieBreaking);

#[pymethods]
impl PyEndianness {
    #[classattr]
    const BIG: PyEndianness = PyEndianness(Endianness::Big);
    #[classattr]
    const LITTLE: PyEndianness = PyEndianness(Endianness::Little);

    fn __repr__(&self) -> String {
        format!(
            "rithm.Endianness.{}",
            match self.0 {
                Endianness::Big => "BIG",
                Endianness::Little => "LITTLE",
            }
        )
    }
}

#[pymethods]
impl PyTieBreaking {
    #[classattr]
    const AWAY_FROM_ZERO: PyTieBreaking = PyTieBreaking(TieBreaking::AwayFromZero);
    #[classattr]
    const TO_EVEN: PyTieBreaking = PyTieBreaking(TieBreaking::ToEven);
    #[classattr]
    const TO_ODD: PyTieBreaking = PyTieBreaking(TieBreaking::ToOdd);
    #[classattr]
    const TOWARD_ZERO: PyTieBreaking = PyTieBreaking(TieBreaking::TowardZero);

    fn __repr__(&self) -> String {
        format!(
            "rithm.TieBreaking.{}",
            match self.0 {
                TieBreaking::AwayFromZero => "AWAY_FROM_ZERO",
                TieBreaking::ToEven => "TO_EVEN",
                TieBreaking::ToOdd => "TO_ODD",
                TieBreaking::TowardZero => "TOWARD_ZERO",
            }
        )
    }
}

#[pymethods]
impl PyInt {
    #[new]
    fn new(_value: Option<&PyAny>, _base: Option<&PyLong>) -> PyResult<Self> {
        match _value {
            None => Ok(PyInt(BigInt::zero())),
            Some(value) => {
                let py = value.py();
                if _base.is_some() || value.is_instance(PyString::type_object(py))? {
                    let base = match _base {
                        Some(base) => {
                            base.extract::<u32>().or(Err(PyValueError::new_err(format!(
                                "Base should be zero or in range from {} to {}, but found: {}.",
                                big_int::MIN_REPRESENTABLE_BASE,
                                big_int::MAX_REPRESENTABLE_BASE,
                                base.repr()?
                            ))))?
                        }
                        None => 10,
                    };
                    match BigInt::from_str_radix(value.extract::<&str>()?, base) {
                        Ok(value) => Ok(PyInt(value)),
                        Err(reason) => Err(PyValueError::new_err(reason.to_string())),
                    }
                } else if value.is_instance(PyFloat::type_object(py))? {
                    Ok(PyInt(
                        BigInt::try_from(value.extract::<&PyFloat>()?.value()).map_err(
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
                    Ok(PyInt(try_py_integral_to_big_int(value)?))
                }
            }
        }
    }

    fn to_bytes(&self, endianness: PyEndianness, py: Python) -> PyObject {
        PyBytes::new(py, &self.0.to_bytes(endianness.0)).to_object(py)
    }

    #[classmethod]
    fn from_bytes(_cls: &PyType, mut bytes: Vec<u8>, endianness: PyEndianness) -> PyInt {
        PyInt(BigInt::from_bytes(bytes.as_mut_slice(), endianness.0))
    }

    fn bit_length(&self) -> PyInt {
        PyInt(self.0.clone().bit_length())
    }

    #[pyo3(text_signature = "($self, other, /)")]
    fn gcd(&self, other: Self) -> PyInt {
        PyInt(self.0.clone().gcd(other.0))
    }

    #[getter]
    fn numerator(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[getter]
    fn denominator(&self) -> Self {
        PyInt(BigInt::one())
    }

    fn __abs__(&self) -> PyInt {
        PyInt(self.0.clone().abs())
    }

    fn __add__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(PyInt(self.0.clone() + other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__radd__(other)
        }
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(PyInt(self.0.clone() & other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__rand__(other)
        }
    }

    fn __bool__(&self) -> bool {
        !self.0.is_zero()
    }

    fn __ceil__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __divmod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        match try_py_any_to_maybe_big_int(divisor)? {
            Some(divisor) => try_divmod(self.0.clone(), divisor)
                .map(|(quotient, remainder)| (PyInt(quotient), PyInt(remainder)).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
    }

    fn __float__(&self) -> PyResult<PyObject> {
        match f64::try_from(self.0.clone()) {
            Ok(value) => Ok(Python::with_gil(|py| value.into_py(py))),
            Err(reason) => Err(PyOverflowError::new_err(reason.to_string())),
        }
    }

    fn __floor__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __floordiv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        match try_py_any_to_maybe_big_int(divisor)? {
            Some(divisor) => {
                try_floordiv(self.0.clone(), divisor).map(|result| PyInt(result).into_py(py))
            }
            None => Ok(py.NotImplemented()),
        }
    }

    fn __getstate__(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.0.to_bytes(PICKLE_SERIALIZATION_ENDIANNESS)).to_object(py)
    }

    fn __hash__(&self) -> ffi::Py_hash_t {
        hash(&self.0) as ffi::Py_hash_t
    }

    fn __int__(&self, py: Python) -> PyObject {
        big_int_to_py_long(&self.0, py)
    }

    fn __invert__(&self) -> PyInt {
        PyInt(!self.0.clone())
    }

    fn __lshift__(&self, shift: &PyAny) -> PyResult<PyObject> {
        let py = shift.py();
        match try_py_any_to_maybe_big_int(shift)? {
            Some(shift) => {
                try_lshift(self.0.clone(), shift).map(|result| PyInt(result).into_py(py))
            }
            None => Ok(py.NotImplemented()),
        }
    }

    fn __mod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        match try_py_any_to_maybe_big_int(divisor)? {
            Some(divisor) => {
                try_mod(self.0.clone(), divisor).map(|result| PyInt(result).into_py(py))
            }
            None => Ok(py.NotImplemented()),
        }
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(PyInt(self.0.clone() * other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__rmul__(other)
        }
    }

    fn __neg__(&self) -> PyInt {
        PyInt(-self.0.clone())
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(PyInt(self.0.clone() | other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__ror__(other)
        }
    }

    fn __pos__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __pow__(&self, exponent: &PyAny, divisor: Option<&PyAny>) -> PyResult<PyObject> {
        let py = exponent.py();
        match try_py_any_to_maybe_big_int(exponent)? {
            Some(exponent) => match divisor {
                Some(divisor) => match try_py_any_to_maybe_big_int(divisor)? {
                    Some(divisor) => try_pow_mod(self.0.clone(), exponent, divisor, py),
                    None => Ok(py.NotImplemented()),
                },
                None => try_pow(self.0.clone(), exponent, py),
            },
            None => Ok(py.NotImplemented()),
        }
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(self.0.clone() + try_py_long_to_big_int(other)?).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rand__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(self.0.clone() & try_py_long_to_big_int(other)?).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rdivmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if dividend.is_instance(PyLong::type_object(py))? {
            try_divmod(try_py_long_to_big_int(dividend)?, self.0.clone())
                .map(|(quotient, remainder)| (PyInt(quotient), PyInt(remainder)).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self) -> String {
        format!("rithm.Int({})", self.0)
    }

    fn __rfloordiv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if dividend.is_instance(PyLong::type_object(py))? {
            try_floordiv(try_py_long_to_big_int(dividend)?, self.0.clone())
                .map(|result| PyInt(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        match try_py_any_to_maybe_big_int(other)? {
            Some(other) => Ok(compare(&self.0, &other, op).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
    }

    fn __rlshift__(&self, base: &PyAny) -> PyResult<PyObject> {
        let py = base.py();
        if base.is_instance(PyLong::type_object(py))? {
            try_lshift(try_py_long_to_big_int(base)?, self.0.clone())
                .map(|result| PyInt(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if dividend.is_instance(PyLong::type_object(py))? {
            try_mod(try_py_long_to_big_int(dividend)?, self.0.clone())
                .map(|result| PyInt(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(self.0.clone() * try_py_long_to_big_int(other)?).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __ror__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(self.0.clone() | try_py_long_to_big_int(other)?).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __round__(&self, digits: Option<&PyLong>, py: Python) -> PyResult<Self> {
        Ok(match digits {
            Some(digits) => {
                if digits.lt(0.into_py(py))? {
                    let ten_to_digits_power = unsafe {
                        BigInt::from(10u8)
                            .checked_pow(-try_py_long_to_big_int(digits)?)
                            .unwrap_unchecked()
                    };
                    PyInt(self.0.clone() - try_mod_to_near(self.0.clone(), ten_to_digits_power)?)
                } else {
                    self.clone()
                }
            }
            None => self.clone(),
        })
    }

    fn __rpow__(&self, base: &PyAny, divisor: Option<&PyAny>) -> PyResult<PyObject> {
        let py = base.py();
        let base = if base.is_instance(PyLong::type_object(py))? {
            try_py_long_to_big_int(base)?
        } else {
            return Ok(py.NotImplemented());
        };
        match divisor {
            Some(divisor) => match try_py_any_to_maybe_big_int(divisor)? {
                Some(divisor) => try_pow_mod(base, self.0.clone(), divisor, py),
                None => Ok(py.NotImplemented()),
            },
            None => try_pow(base, self.0.clone(), py),
        }
    }

    fn __rrshift__(&self, base: &PyAny) -> PyResult<PyObject> {
        let py = base.py();
        if base.is_instance(PyLong::type_object(py))? {
            try_rshift(try_py_long_to_big_int(base)?, self.0.clone())
                .map(|result| PyInt(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rshift__(&self, shift: &PyAny) -> PyResult<PyObject> {
        let py = shift.py();
        match try_py_any_to_maybe_big_int(shift)? {
            Some(shift) => {
                try_rshift(self.0.clone(), shift).map(|result| PyInt(result).into_py(py))
            }
            None => Ok(py.NotImplemented()),
        }
    }

    fn __rsub__(&self, minuend: &PyAny) -> PyResult<PyObject> {
        let py = minuend.py();
        if minuend.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(try_py_long_to_big_int(minuend)? - self.0.clone()).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rtruediv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if dividend.is_instance(PyLong::type_object(py))? {
            try_truediv(try_py_long_to_big_int(dividend)?, self.0.clone())
                .map(|result| PyFraction(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rxor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(self.0.clone() ^ try_py_long_to_big_int(other)?).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __setstate__(&mut self, state: PyObject, py: Python) -> PyResult<()> {
        state
            .extract::<&PyBytes>(py)
            .and_then(|py_bytes| py_bytes.extract::<Vec<u8>>())
            .map(|bytes| {
                self.0 = BigInt::from_bytes(&bytes, PICKLE_SERIALIZATION_ENDIANNESS);
            })
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __sub__(&self, subtrahend: &PyAny) -> PyResult<PyObject> {
        let py = subtrahend.py();
        match try_py_any_to_maybe_big_int(subtrahend)? {
            Some(subtrahend) => Ok(PyInt(self.0.clone() - subtrahend).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
    }

    fn __truediv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        match try_py_any_to_maybe_big_int(divisor)? {
            Some(divisor) => {
                try_truediv(self.0.clone(), divisor).map(|result| PyFraction(result).into_py(py))
            }
            None => Ok(py.NotImplemented()),
        }
    }

    fn __trunc__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __xor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(PyInt(self.0.clone() ^ other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__rxor__(other)
        }
    }
}

#[inline]
fn big_int_to_py_long(value: &BigInt, py: Python) -> PyObject {
    let buffer = value.to_bytes(Endianness::Little);
    unsafe {
        PyObject::from_owned_ptr(
            py,
            ffi::_PyLong_FromByteArray(buffer.as_ptr(), buffer.len(), 1, 1),
        )
    }
}

#[inline]
fn try_divmod<
    Dividend: CheckedDivRemEuclid<Divisor, Output = Option<(Quotient, Remainder)>>,
    Divisor,
    Quotient,
    Remainder,
>(
    dividend: Dividend,
    divisor: Divisor,
) -> PyResult<(Quotient, Remainder)> {
    match dividend.checked_div_rem_euclid(divisor) {
        Some((quotient, remainder)) => Ok((quotient, remainder)),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
fn try_floordiv(dividend: BigInt, divisor: BigInt) -> PyResult<BigInt> {
    match dividend.checked_div_euclid(divisor) {
        Some(result) => Ok(result),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
fn try_lshift(base: BigInt, shift: BigInt) -> PyResult<BigInt> {
    base.checked_shl(shift).map_err(|reason| match reason {
        big_int::LeftShiftError::NegativeShift => PyValueError::new_err(reason.to_string()),
        big_int::LeftShiftError::OutOfMemory => PyMemoryError::new_err(reason.to_string()),
        big_int::LeftShiftError::TooLarge => PyOverflowError::new_err(reason.to_string()),
    })
}

#[inline]
fn try_mod(dividend: BigInt, divisor: BigInt) -> PyResult<BigInt> {
    match dividend.checked_rem_euclid(divisor) {
        Some(result) => Ok(result),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
fn try_mod_to_near(dividend: BigInt, divisor: BigInt) -> PyResult<BigInt> {
    let (quotient, remainder) = match dividend.checked_div_rem_euclid(divisor.clone()) {
        Some((quotient, remainder)) => Ok((quotient, remainder)),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }?;
    let double_remainder = remainder
        .clone()
        .checked_shl(BigInt::one())
        .map_err(|reason| match reason {
            big_int::LeftShiftError::NegativeShift => PyValueError::new_err(reason.to_string()),
            big_int::LeftShiftError::OutOfMemory => PyMemoryError::new_err(reason.to_string()),
            big_int::LeftShiftError::TooLarge => PyOverflowError::new_err(reason.to_string()),
        })?;
    let greater_than_half = if divisor.is_positive() {
        double_remainder > divisor
    } else {
        double_remainder < divisor
    };
    let exactly_half = double_remainder == divisor;
    Ok(
        if greater_than_half || (exactly_half && quotient.is_odd()) {
            remainder - divisor
        } else {
            remainder
        },
    )
}

#[inline]
fn try_pow(base: BigInt, exponent: BigInt, py: Python) -> PyResult<PyObject> {
    if exponent.is_negative() {
        match unsafe { Fraction::new(base, BigInt::one()).unwrap_unchecked() }.checked_pow(exponent)
        {
            Some(power) => Ok(PyFraction(power).into_py(py)),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    } else {
        Ok(PyInt(unsafe { base.checked_pow(exponent).unwrap_unchecked() }).into_py(py))
    }
}

#[inline]
fn try_pow_mod(base: BigInt, exponent: BigInt, divisor: BigInt, py: Python) -> PyResult<PyObject> {
    let is_zero_divisor = divisor.is_zero();
    match base.checked_pow_rem_euclid(exponent, divisor) {
        Some(remainder) => Ok(PyInt(remainder).into_py(py)),
        None => Err(PyValueError::new_err(if is_zero_divisor {
            "Divisor cannot be zero."
        } else {
            "Base is not invertible for the given divisor."
        })),
    }
}

#[inline]
fn try_rshift(base: BigInt, shift: BigInt) -> PyResult<BigInt> {
    base.checked_shr(shift).map_err(|reason| match reason {
        big_int::RightShiftError::NegativeShift => PyValueError::new_err(reason.to_string()),
    })
}

#[inline]
fn try_py_any_to_maybe_big_int(value: &PyAny) -> PyResult<Option<BigInt>> {
    let py = value.py();
    if value.is_instance(PyInt::type_object(py))? {
        Ok(Some(value.extract::<PyInt>()?.0))
    } else if value.is_instance(PyLong::type_object(py))? {
        try_py_long_to_big_int(value).map(Some)
    } else {
        Ok(None)
    }
}

#[inline]
fn try_py_any_to_maybe_fraction(value: &PyAny) -> PyResult<Option<Fraction>> {
    let py = value.py();
    match value.getattr(intern!(py, "numerator")) {
        Ok(numerator) => match try_py_any_to_maybe_big_int(numerator)? {
            Some(numerator) => match value.getattr(intern!(py, "denominator")) {
                Ok(denominator) => match try_py_any_to_maybe_big_int(denominator)? {
                    Some(denominator) => try_truediv(numerator, denominator).map(Some),
                    None => Ok(None),
                },
                Err(_) => Ok(None),
            },
            None => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

#[inline]
fn try_py_integral_to_big_int(value: &PyAny) -> PyResult<BigInt> {
    if value.is_instance(PyInt::type_object(value.py()))? {
        Ok(value.extract::<PyInt>()?.0)
    } else {
        try_py_long_to_big_int(value)
    }
}

#[inline]
fn try_py_long_to_big_int(value: &PyAny) -> PyResult<BigInt> {
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
            Ordering::Equal => Ok(BigInt::zero()),
            Ordering::Greater => {
                let bytes_count = (bits_count as usize) / (u8::BITS as usize) + 1;
                let mut buffer = vec![0u8; bytes_count];
                if ffi::_PyLong_AsByteArray(
                    Py::<PyLong>::from_owned_ptr(py, value).as_ptr() as *mut ffi::PyLongObject,
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    1,
                    1,
                ) < 0
                {
                    Err(PyErr::fetch(py))
                } else {
                    Ok(BigInt::from_bytes(
                        buffer.as_mut_slice(),
                        Endianness::Little,
                    ))
                }
            }
        }
    }
}

#[inline]
fn try_truediv(dividend: BigInt, divisor: BigInt) -> PyResult<Fraction> {
    match Fraction::new(dividend, divisor) {
        Some(quotient) => Ok(quotient),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[pymethods]
impl PyFraction {
    #[new]
    fn new(_numerator: Option<&PyAny>, _denominator: Option<&PyAny>) -> PyResult<Self> {
        match _denominator {
            Some(denominator) => match _numerator {
                Some(numerator) => {
                    match Fraction::new(
                        try_py_integral_to_big_int(numerator)?,
                        try_py_integral_to_big_int(denominator)?,
                    ) {
                        Some(fraction) => Ok(PyFraction(fraction)),
                        None => Err(PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )),
                    }
                }
                None => Err(PyTypeError::new_err(
                    "Numerator should be of type `Int` or `int`, but found `None`",
                )),
            },
            None => match _numerator {
                Some(value) => {
                    let py = value.py();
                    if value.is_instance(PyFraction::type_object(py))? {
                        value.extract::<PyFraction>()
                    } else if value.is_instance(PyFloat::type_object(py))? {
                        match Fraction::try_from(value.extract::<f64>()?) {
                            Ok(fraction) => Ok(PyFraction(fraction)),
                            Err(reason) => Err(match reason {
                                fraction::FromFloatConversionError::NaN => {
                                    PyValueError::new_err(reason.to_string())
                                }
                                _ => PyOverflowError::new_err(reason.to_string()),
                            }),
                        }
                    } else {
                        match try_py_any_to_maybe_big_int(value)? {
                            Some(numerator) => Ok(PyFraction(Fraction::from(numerator))),
                            None => match try_py_any_to_maybe_fraction(value)? {
                                Some(fraction) => Ok(PyFraction(fraction)),
                                None => Err(PyTypeError::new_err(
                                    format!("Value should be rational or floating point number, but found: {}",
                                            value.get_type().repr()?),
                                )),
                            },
                        }
                    }
                }
                None => Ok(PyFraction(Fraction::zero())),
            },
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

    fn round(&self, tie_breaking: PyTieBreaking) -> PyInt {
        PyInt(self.0.clone().round(tie_breaking.0))
    }

    fn __abs__(&self) -> PyFraction {
        PyFraction(self.0.clone().abs())
    }

    fn __add__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyFraction::type_object(py))? {
            Ok(PyFraction(self.0.clone() + other.extract::<PyFraction>()?.0).into_py(py))
        } else {
            self.__radd__(other)
        }
    }

    fn __bool__(&self) -> bool {
        self.numerator().__bool__()
    }

    fn __ceil__(&self) -> PyInt {
        PyInt(self.0.clone().ceil())
    }

    fn __divmod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if divisor.is_instance(PyFraction::type_object(py))? {
            let divisor = divisor.extract::<PyFraction>()?.0;
            try_divmod(self.0.clone(), divisor)
                .map(|(quotient, remainder)| (PyInt(quotient), PyFraction(remainder)).into_py(py))
        } else {
            match try_py_any_to_maybe_big_int(divisor)? {
                Some(divisor) => {
                    try_divmod(self.0.clone(), divisor).map(|(quotient, remainder)| {
                        (PyInt(quotient), PyFraction(remainder)).into_py(py)
                    })
                }
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __float__(&self, py: Python) -> PyResult<PyObject> {
        match <Fraction as TryInto<f64>>::try_into(self.0.clone()) {
            Ok(float) => Ok(float.into_py(py)),
            Err(reason) => Err(PyOverflowError::new_err(reason.to_string())),
        }
    }

    fn __floor__(&self) -> PyInt {
        PyInt(self.0.clone().floor())
    }

    fn __floordiv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if divisor.is_instance(PyFraction::type_object(py))? {
            match self
                .0
                .clone()
                .checked_div_euclid(divisor.extract::<PyFraction>()?.0)
            {
                Some(quotient) => Ok(PyInt(quotient).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            match try_py_any_to_maybe_big_int(divisor)? {
                Some(divisor) => match self.0.clone().checked_div_euclid(divisor) {
                    Some(quotient) => Ok(PyInt(quotient).into_py(py)),
                    None => Err(PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )),
                },
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __getstate__(&self, py: Python) -> PyObject {
        (
            self.numerator().__getstate__(py),
            self.denominator().__getstate__(py),
        )
            .to_object(py)
    }

    fn __mod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if divisor.is_instance(PyFraction::type_object(py))? {
            match self
                .0
                .clone()
                .checked_rem_euclid(divisor.extract::<PyFraction>()?.0)
            {
                Some(remainder) => Ok(PyFraction(remainder).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            match try_py_any_to_maybe_big_int(divisor)? {
                Some(divisor) => match self.0.clone().checked_rem_euclid(divisor) {
                    Some(remainder) => Ok(PyFraction(remainder).into_py(py)),
                    None => Err(PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )),
                },
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyFraction::type_object(py))? {
            Ok(PyFraction(self.0.clone() * other.extract::<PyFraction>()?.0).into_py(py))
        } else {
            self.__rmul__(other)
        }
    }

    fn __neg__(&self) -> PyFraction {
        PyFraction(-self.0.clone())
    }

    fn __pos__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __pow__(&self, exponent: &PyAny, modulo: &PyAny) -> PyResult<PyObject> {
        let py = exponent.py();
        if !modulo.is_none() {
            Ok(py.NotImplemented())
        } else {
            match try_py_any_to_maybe_big_int(exponent)? {
                Some(exponent) => match self.0.clone().checked_pow(exponent) {
                    Some(power) => Ok(PyFraction(power).into_py(py)),
                    None => Err(PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )),
                },
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        match try_py_any_to_maybe_big_int(other)? {
            Some(other) => Ok(PyFraction(other + self.0.clone()).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
    }

    fn __rdivmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        match try_py_any_to_maybe_big_int(dividend)? {
            Some(dividend) => try_divmod(dividend, self.0.clone())
                .map(|(quotient, remainder)| (PyInt(quotient), PyFraction(remainder)).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
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
        if other.is_instance(PyFraction::type_object(py))? {
            Ok(compare(&self.0, &other.extract::<PyFraction>()?.0, op).into_py(py))
        } else {
            match try_py_any_to_maybe_big_int(other)? {
                Some(other) => Ok(compare(&self.0, &other, op).into_py(py)),
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __rfloordiv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        match try_py_any_to_maybe_big_int(dividend)? {
            Some(dividend) => match dividend.checked_div_euclid(self.0.clone()) {
                Some(quotient) => Ok(PyInt(quotient).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            },
            None => Ok(py.NotImplemented()),
        }
    }

    fn __rmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        match try_py_any_to_maybe_big_int(dividend)? {
            Some(dividend) => match dividend.checked_rem_euclid(self.0.clone()) {
                Some(remainder) => Ok(PyFraction(remainder).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            },
            None => Ok(py.NotImplemented()),
        }
    }

    fn __rmul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        match try_py_any_to_maybe_big_int(other)? {
            Some(other) => Ok(PyFraction(self.0.clone() * other).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
    }

    fn __round__(&self, digits: Option<&PyLong>, py: Python) -> PyResult<PyObject> {
        match digits {
            Some(digits) => {
                let digits = try_py_long_to_big_int(digits)?;
                let shift = unsafe {
                    BigInt::from(10)
                        .checked_pow(digits.clone().abs())
                        .unwrap_unchecked()
                };
                if digits.is_positive() {
                    Ok(PyFraction(unsafe {
                        Fraction::new(
                            (self.0.clone() * shift.clone()).round(TieBreaking::ToEven),
                            shift,
                        )
                        .unwrap_unchecked()
                    })
                    .into_py(py))
                } else {
                    Ok(PyFraction(Fraction::from(
                        (self.0.clone() / shift.clone()).round(TieBreaking::ToEven) * shift,
                    ))
                    .into_py(py))
                }
            }
            None => Ok(PyInt(self.0.clone().round(TieBreaking::ToEven)).into_py(py)),
        }
    }

    fn __rsub__(&self, subtrahend: &PyAny) -> PyResult<PyObject> {
        let py = subtrahend.py();
        match try_py_any_to_maybe_big_int(subtrahend)? {
            Some(subtrahend) => Ok(PyFraction(subtrahend - self.0.clone()).into_py(py)),
            None => Ok(py.NotImplemented()),
        }
    }

    fn __rtruediv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        match try_py_any_to_maybe_big_int(dividend)? {
            Some(dividend) => match dividend.checked_div(self.0.clone()) {
                Some(quotient) => Ok(PyFraction(quotient).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            },
            None => Ok(py.NotImplemented()),
        }
    }

    fn __setstate__(&mut self, state: (PyObject, PyObject), py: Python) -> PyResult<()> {
        let (numerator_state, denominator_state) = state;
        let mut numerator = PyInt(BigInt::zero());
        numerator.__setstate__(numerator_state, py)?;
        let mut denominator = PyInt(BigInt::zero());
        denominator.__setstate__(denominator_state, py)?;
        match Fraction::new(numerator.0, denominator.0) {
            Some(fraction) => {
                self.0 = fraction;
                Ok(())
            }
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __sub__(&self, minuend: &PyAny) -> PyResult<PyObject> {
        let py = minuend.py();
        if minuend.is_instance(PyFraction::type_object(py))? {
            Ok(PyFraction(self.0.clone() - minuend.extract::<PyFraction>()?.0).into_py(py))
        } else {
            match try_py_any_to_maybe_big_int(minuend)? {
                Some(minuend) => Ok(PyFraction(self.0.clone() - minuend).into_py(py)),
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __truediv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if divisor.is_instance(PyFraction::type_object(py))? {
            match self
                .0
                .clone()
                .checked_div(divisor.extract::<PyFraction>()?.0)
            {
                Some(quotient) => Ok(PyFraction(quotient).into_py(py)),
                None => Err(PyZeroDivisionError::new_err(
                    UNDEFINED_DIVISION_ERROR_MESSAGE,
                )),
            }
        } else {
            match try_py_any_to_maybe_big_int(divisor)? {
                Some(divisor) => match self.0.clone().checked_div(divisor) {
                    Some(quotient) => Ok(PyFraction(quotient).into_py(py)),
                    None => Err(PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )),
                },
                None => Ok(py.NotImplemented()),
            }
        }
    }

    fn __trunc__(&self) -> PyInt {
        PyInt(self.0.clone().trunc())
    }
}

fn hash(value: &BigInt) -> usize {
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
fn _rithm(py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr(intern!(py, "__doc__"), env!("CARGO_PKG_DESCRIPTION"))?;
    module.setattr(intern!(py, "__version__"), env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyEndianness>()?;
    module.add_class::<PyFraction>()?;
    module.add_class::<PyInt>()?;
    module.add_class::<PyTieBreaking>()?;
    let numbers_module = py.import("numbers")?;
    let rational_cls = numbers_module.getattr(intern!(py, "Rational"))?;
    let integral_cls = numbers_module.getattr(intern!(py, "Integral"))?;
    integral_cls.call_method1("register", (PyInt::type_object(py),))?;
    rational_cls.call_method1("register", (PyFraction::type_object(py),))?;
    Ok(())
}

#[doc = include_str!("../README.md")]
type _DoctestReadme = ();
