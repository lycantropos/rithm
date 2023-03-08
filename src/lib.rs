use std::cmp::Ordering;
use std::convert::TryFrom;

use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyMemoryError, PyOverflowError, PyTypeError, PyValueError,
    PyZeroDivisionError,
};
use pyo3::prelude::{
    pyclass, pymethods, pymodule, PyModule, PyResult, Python,
};
use pyo3::type_object::PyTypeInfo;
use pyo3::types::{PyBytes, PyFloat, PyLong, PyString, PyType};
use pyo3::{
    intern, AsPyPointer, IntoPy, Py, PyAny, PyCell, PyClass, PyErr, PyObject,
    PyRef, ToPyObject,
};
use pyo3_ffi as ffi;
use traiter::numbers::{
    Abs, BitLength, Ceil, CheckedDiv, CheckedDivEuclid, CheckedDivRemEuclid,
    CheckedPow, CheckedPowRemEuclid, CheckedRemEuclid, CheckedShl, CheckedShr,
    Endianness, Floor, FromBytes, FromStrRadix, Gcd, IsPowerOfTwo, Parity,
    Round, Signed, TieBreaking, ToBytes, Trunc, Unitary, Zeroable,
};

use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;

pub mod big_int;
mod constants;
mod contracts;
pub mod fraction;
mod traits;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;
const DIGIT_BITNESS: usize = (Digit::BITS - 1) as usize;
const _: () =
    assert!(big_int::is_valid_digit_bitness::<Digit, DIGIT_BITNESS>());
#[cfg(target_arch = "x86")]
const HASH_BITS: usize = 31;
#[cfg(not(target_arch = "x86"))]
const HASH_BITS: usize = 61;
const HASH_INF: ffi::Py_hash_t = 314_159;
const HASH_MODULUS: usize = (1 << HASH_BITS) - 1;
const PICKLE_SERIALIZATION_ENDIANNESS: Endianness = Endianness::Little;

type BigInt = big_int::BigInt<Digit, DIGIT_BITNESS>;
type Fraction = fraction::Fraction<BigInt>;

#[pyclass(name = "Endianness", module = "rithm.enums", frozen)]
#[derive(Clone)]
struct PyEndianness(Endianness);

#[pyclass(name = "Fraction", module = "rithm.fraction")]
#[pyo3(text_signature = "(numerator=None, denominator=None, /)")]
#[derive(Clone)]
struct PyFraction(Fraction);

#[pyclass(name = "Int", module = "rithm.integer")]
#[pyo3(text_signature = "(value=None, base=None, /)")]
#[derive(Clone)]
struct PyInt(BigInt);

#[pyclass(name = "TieBreaking", module = "rithm.enums", frozen)]
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
            "Endianness.{}",
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
    const AWAY_FROM_ZERO: PyTieBreaking =
        PyTieBreaking(TieBreaking::AwayFromZero);
    #[classattr]
    const TO_EVEN: PyTieBreaking = PyTieBreaking(TieBreaking::ToEven);
    #[classattr]
    const TO_ODD: PyTieBreaking = PyTieBreaking(TieBreaking::ToOdd);
    #[classattr]
    const TOWARD_ZERO: PyTieBreaking = PyTieBreaking(TieBreaking::TowardZero);

    fn __repr__(&self) -> String {
        format!(
            "TieBreaking.{}",
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
                if _base.is_some()
                    || value.is_instance(PyString::type_object(py))?
                {
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
                    match BigInt::from_str_radix(
                        value.extract::<&str>()?,
                        base,
                    ) {
                        Ok(value) => Ok(PyInt(value)),
                        Err(reason) => {
                            Err(PyValueError::new_err(reason.to_string()))
                        }
                    }
                } else if value.is_instance(PyFloat::type_object(py))? {
                    Ok(PyInt(
                        BigInt::try_from(value.extract::<&PyFloat>()?.value())
                            .map_err(|reason| match reason {
                                big_int::TryFromFloatError::Infinity => {
                                    PyOverflowError::new_err(
                                        reason.to_string(),
                                    )
                                }
                                big_int::TryFromFloatError::NaN => {
                                    PyValueError::new_err(reason.to_string())
                                }
                            })?,
                    ))
                } else {
                    try_big_int_from_py_any(value).map(Self)
                }
            }
        }
    }

    fn to_bytes(&self, endianness: &PyEndianness, py: Python) -> PyObject {
        PyBytes::new(py, &self.0.to_bytes(endianness.0)).to_object(py)
    }

    #[classmethod]
    fn from_bytes(
        _cls: &PyType,
        mut bytes: Vec<u8>,
        endianness: &PyEndianness,
    ) -> PyInt {
        PyInt(BigInt::from_bytes(bytes.as_mut_slice(), endianness.0))
    }

    fn bit_length(&self) -> PyInt {
        PyInt(self.0.bit_length())
    }

    fn is_power_of_two(&self) -> bool {
        self.0.is_power_of_two()
    }

    #[pyo3(text_signature = "($self, other, /)")]
    fn gcd(&self, other: &Self) -> PyInt {
        Self((&self.0).gcd(&other.0))
    }

    #[getter]
    fn numerator(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    #[getter]
    fn denominator(_slf: PyRef<Self>) -> Self {
        Self(BigInt::one())
    }

    fn __abs__(&self) -> PyInt {
        Self((&self.0).abs())
    }

    fn __add__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<Self>(other)
            .map(|other| Self(&self.0 + &other.0).into_py(py))
            .or_else(|_| self.__radd__(other))
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<Self>(other)
            .map(|other| Self(&self.0 & &other.0).into_py(py))
            .or_else(|_| self.__rand__(other))
    }

    fn __bool__(&self) -> bool {
        !self.0.is_zero()
    }

    fn __ceil__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __divmod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            try_divmod(&self.0, &divisor.0).map(|(quotient, remainder)| {
                (Self(quotient), Self(remainder)).into_py(py)
            })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_divmod(&self.0, divisor).map(|(quotient, remainder)| {
                (Self(quotient), Self(remainder)).into_py(py)
            })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __float__(&self, py: Python) -> PyResult<PyObject> {
        f64::try_from(&self.0)
            .map(|value| value.into_py(py))
            .map_err(|reason| PyOverflowError::new_err(reason.to_string()))
    }

    fn __floor__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __floordiv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            try_floordiv(&self.0, &divisor.0)
                .map(|result| Self(result).into_py(py))
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_floordiv(&self.0, divisor)
                .map(|result| Self(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __getstate__(&self, py: Python) -> PyObject {
        PyBytes::new(py, &self.0.to_bytes(PICKLE_SERIALIZATION_ENDIANNESS))
            .to_object(py)
    }

    fn __hash__(&self) -> ffi::Py_hash_t {
        hash(&self.0) as ffi::Py_hash_t
    }

    fn __index__(&self, py: Python) -> PyObject {
        to_py_long::<BigInt>(&self.0, py)
    }

    fn __int__(&self, py: Python) -> PyObject {
        to_py_long::<BigInt>(&self.0, py)
    }

    fn __invert__(&self) -> PyInt {
        Self(!&self.0)
    }

    fn __lshift__(&self, shift: &PyAny) -> PyResult<PyObject> {
        let py = shift.py();
        if let Ok(shift) = to_py_ref::<Self>(shift) {
            try_lshift(&self.0, &shift.0)
                .map(|result| Self(result).into_py(py))
        } else if let Ok(shift) = try_big_int_from_py_integral(shift) {
            try_lshift(&self.0, shift).map(|result| Self(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __mod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            try_mod(&self.0, &divisor.0).map(|result| Self(result).into_py(py))
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_mod(&self.0, divisor).map(|result| Self(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<Self>(other)
            .map(|other| Self(&self.0 * &other.0).into_py(py))
            .or_else(|_| self.__rmul__(other))
    }

    fn __neg__(&self) -> PyInt {
        Self(-&self.0)
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(PyInt(&self.0 | other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__ror__(other)
        }
    }

    fn __pos__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __pow__(
        &self,
        exponent: &PyAny,
        divisor: Option<&PyAny>,
    ) -> PyResult<PyObject> {
        let py = exponent.py();
        if let Ok(exponent) = try_big_int_from_py_any(exponent) {
            match divisor {
                Some(divisor) => {
                    if let Ok(divisor) = to_py_ref::<Self>(divisor) {
                        (&self.0)
                            .checked_pow_rem_euclid(exponent, &divisor.0)
                            .map(|remainder| Self(remainder).into_py(py))
                            .map_err(|reason| {
                                PyValueError::new_err(reason.to_string())
                            })
                    } else if let Ok(divisor) =
                        try_big_int_from_py_integral(divisor)
                    {
                        (&self.0)
                            .checked_pow_rem_euclid(exponent, divisor)
                            .map(|remainder| Self(remainder).into_py(py))
                            .map_err(|reason| {
                                PyValueError::new_err(reason.to_string())
                            })
                    } else {
                        Ok(py.NotImplemented())
                    }
                }
                None => {
                    if exponent.is_negative() {
                        try_pow_negative_exponent(self.0.clone(), exponent, py)
                    } else {
                        Ok(Self(pow_non_negative_exponent(&self.0, &exponent))
                            .into_py(py))
                    }
                }
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        try_big_int_from_py_integral(other)
            .map(|other| Self(other + &self.0).into_py(py))
            .or_else(|_| Ok(py.NotImplemented()))
    }

    fn __rand__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        try_big_int_from_py_integral(other)
            .map(|other| Self(other & &self.0).into_py(py))
            .or_else(|_| Ok(py.NotImplemented()))
    }

    fn __rdivmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            try_divmod(dividend, &self.0).map(|(quotient, remainder)| {
                (PyInt(quotient), PyInt(remainder)).into_py(py)
            })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self) -> String {
        format!("Int({})", self.0)
    }

    fn __rfloordiv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            try_floordiv(dividend, &self.0)
                .map(|result| Self(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<Self>(other)
            .map(|other| compare(&self.0, &other.0, op).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(other)
                    .map(|other| compare(&self.0, &other, op).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __rlshift__(&self, base: &PyAny) -> PyResult<PyObject> {
        let py = base.py();
        if let Ok(base) = try_big_int_from_py_integral(base) {
            try_lshift(base, &self.0).map(|result| Self(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if dividend.is_instance(PyLong::type_object(py))? {
            try_mod(try_big_int_from_py_integral(dividend)?, &self.0)
                .map(|result| PyInt(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(try_big_int_from_py_integral(other)? * &self.0)
                .into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __ror__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(&self.0 | try_big_int_from_py_integral(other)?)
                .into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __round__(
        &self,
        digits: Option<&PyLong>,
        py: Python,
    ) -> PyResult<Self> {
        Ok(match digits {
            Some(digits) => {
                if digits.lt(0.into_py(py))? {
                    let ten_to_digits_power = unsafe {
                        BigInt::from(10u8)
                            .checked_pow(-try_big_int_from_py_integral(
                                digits,
                            )?)
                            .unwrap_unchecked()
                    };
                    PyInt(
                        &self.0
                            - try_mod_to_near(&self.0, &ten_to_digits_power)?,
                    )
                } else {
                    self.clone()
                }
            }
            None => self.clone(),
        })
    }

    fn __rpow__(
        &self,
        base: &PyAny,
        divisor: Option<&PyAny>,
    ) -> PyResult<PyObject> {
        let py = base.py();
        let base = if base.is_instance(PyLong::type_object(py))? {
            try_big_int_from_py_integral(base)?
        } else {
            return Ok(py.NotImplemented());
        };
        match divisor {
            Some(divisor) => {
                if let Ok(divisor) = to_py_ref::<Self>(divisor) {
                    base.checked_pow_rem_euclid(&self.0, &divisor.0)
                        .map(|remainder| PyInt(remainder).into_py(py))
                        .map_err(|reason| {
                            PyValueError::new_err(reason.to_string())
                        })
                } else if let Ok(divisor) =
                    try_big_int_from_py_integral(divisor)
                {
                    base.checked_pow_rem_euclid(&self.0, divisor)
                        .map(|remainder| PyInt(remainder).into_py(py))
                        .map_err(|reason| {
                            PyValueError::new_err(reason.to_string())
                        })
                } else {
                    Ok(py.NotImplemented())
                }
            }
            None => {
                if self.0.is_negative() {
                    try_pow_negative_exponent(base, self.0.clone(), py)
                } else {
                    Ok(PyInt(pow_non_negative_exponent(&base, &self.0))
                        .into_py(py))
                }
            }
        }
    }

    fn __rrshift__(&self, base: &PyAny) -> PyResult<PyObject> {
        let py = base.py();
        if base.is_instance(PyLong::type_object(py))? {
            try_rshift(try_big_int_from_py_integral(base)?, &self.0)
                .map(|result| PyInt(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rshift__(&self, shift: &PyAny) -> PyResult<PyObject> {
        let py = shift.py();
        if let Ok(shift) = to_py_ref::<Self>(shift) {
            try_rshift(&self.0, &shift.0)
                .map(|result| Self(result).into_py(py))
        } else if let Ok(shift) = try_big_int_from_py_integral(shift) {
            try_rshift(&self.0, shift).map(|result| Self(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rsub__(&self, minuend: &PyAny) -> PyResult<PyObject> {
        let py = minuend.py();
        try_big_int_from_py_integral(minuend)
            .map(|minuend| PyInt(minuend - &self.0).into_py(py))
            .or_else(|_| Ok(py.NotImplemented()))
    }

    fn __rtruediv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if dividend.is_instance(PyLong::type_object(py))? {
            let dividend = try_big_int_from_py_integral(dividend)?;
            try_truediv(dividend, self.0.clone())
                .map(|result| PyFraction(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rxor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyLong::type_object(py))? {
            Ok(PyInt(&self.0 ^ try_big_int_from_py_integral(other)?)
                .into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __setstate__(&mut self, state: PyObject, py: Python) -> PyResult<()> {
        state
            .extract::<&PyBytes>(py)
            .and_then(|py_bytes| py_bytes.extract::<Vec<u8>>())
            .map(|bytes| {
                self.0 = BigInt::from_bytes(
                    &bytes,
                    PICKLE_SERIALIZATION_ENDIANNESS,
                );
            })
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __sub__(&self, subtrahend: &PyAny) -> PyResult<PyObject> {
        let py = subtrahend.py();
        to_py_ref::<Self>(subtrahend)
            .map(|subtrahend| Self(&self.0 - &subtrahend.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(subtrahend)
                    .map(|subtrahend| Self(&self.0 - subtrahend).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __truediv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = try_big_int_from_py_any(divisor) {
            try_truediv(self.0.clone(), divisor)
                .map(|result| PyFraction(result).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __trunc__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __xor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyInt::type_object(py))? {
            Ok(Self(&self.0 ^ other.extract::<PyInt>()?.0).into_py(py))
        } else {
            self.__rxor__(other)
        }
    }
}

#[inline]
fn to_py_long<T: ToBytes<Output = Vec<u8>>>(
    value: &T,
    py: Python,
) -> PyObject {
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
fn try_floordiv<
    Dividend: CheckedDivEuclid<Divisor, Output = Option<Value>>,
    Divisor,
    Value,
>(
    dividend: Dividend,
    divisor: Divisor,
) -> PyResult<Value> {
    match dividend.checked_div_euclid(divisor) {
        Some(result) => Ok(result),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
fn try_lshift<
    Base: CheckedShl<Shift, Output = Result<Value, big_int::ShlError>>,
    Shift,
    Value,
>(
    base: Base,
    shift: Shift,
) -> PyResult<Value> {
    base.checked_shl(shift).map_err(|reason| match reason {
        big_int::ShlError::NegativeShift => {
            PyValueError::new_err(reason.to_string())
        }
        big_int::ShlError::OutOfMemory => {
            PyMemoryError::new_err(reason.to_string())
        }
        big_int::ShlError::TooLarge => {
            PyOverflowError::new_err(reason.to_string())
        }
    })
}

#[inline]
fn try_mod<
    Dividend: CheckedRemEuclid<Divisor, Output = Option<Value>>,
    Divisor,
    Value,
>(
    dividend: Dividend,
    divisor: Divisor,
) -> PyResult<Value> {
    match dividend.checked_rem_euclid(divisor) {
        Some(result) => Ok(result),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
fn try_mod_to_near(dividend: &BigInt, divisor: &BigInt) -> PyResult<BigInt> {
    let (quotient, remainder) = match dividend.checked_div_rem_euclid(divisor)
    {
        Some((quotient, remainder)) => Ok((quotient, remainder)),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }?;
    let double_remainder = (&remainder).checked_shl(BigInt::one()).map_err(
        |reason| match reason {
            big_int::ShlError::NegativeShift => {
                PyValueError::new_err(reason.to_string())
            }
            big_int::ShlError::OutOfMemory => {
                PyMemoryError::new_err(reason.to_string())
            }
            big_int::ShlError::TooLarge => {
                PyOverflowError::new_err(reason.to_string())
            }
        },
    )?;
    let greater_than_half = if divisor.is_positive() {
        &double_remainder > divisor
    } else {
        &double_remainder < divisor
    };
    let exactly_half = &double_remainder == divisor;
    Ok(
        if greater_than_half || (exactly_half && quotient.is_odd()) {
            remainder - divisor
        } else {
            remainder
        },
    )
}

#[inline]
fn pow_non_negative_exponent(base: &BigInt, exponent: &BigInt) -> BigInt {
    debug_assert!(!exponent.is_negative());
    unsafe { base.checked_pow(exponent).unwrap_unchecked() }
}

#[inline]
fn try_pow_negative_exponent(
    base: BigInt,
    exponent: BigInt,
    py: Python,
) -> PyResult<PyObject> {
    debug_assert!(exponent.is_negative());
    match Fraction::from(base).checked_pow(exponent) {
        Some(power) => Ok(PyFraction(power).into_py(py)),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
fn try_rshift<
    Base: CheckedShr<Shift, Output = Result<Value, big_int::ShrError>>,
    Shift,
    Value,
>(
    base: Base,
    shift: Shift,
) -> PyResult<Value> {
    base.checked_shr(shift).map_err(|reason| match reason {
        big_int::ShrError::NegativeShift => {
            PyValueError::new_err(reason.to_string())
        }
    })
}

#[inline]
fn try_big_int_from_py_any(value: &PyAny) -> PyResult<BigInt> {
    value
        .extract::<PyInt>()
        .map(|value| value.0)
        .or_else(|_| try_big_int_from_py_integral(value))
}

#[inline]
fn try_big_int_from_py_integral(value: &PyAny) -> PyResult<BigInt> {
    try_le_bytes_from_py_integral(value).map(|bytes| {
        if bytes.is_empty() {
            BigInt::zero()
        } else {
            BigInt::from_bytes(&bytes, Endianness::Little)
        }
    })
}

#[inline]
fn try_le_bytes_from_py_integral(value: &PyAny) -> PyResult<Vec<u8>> {
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
            Ordering::Equal => Ok(vec![0; 0]),
            Ordering::Greater => {
                let result_size = bits_count / (u8::BITS as usize) + 1;
                let mut result = vec![0u8; result_size];
                if ffi::_PyLong_AsByteArray(
                    Py::<PyLong>::from_owned_ptr(py, value).as_ptr()
                        as *mut ffi::PyLongObject,
                    result.as_mut_ptr(),
                    result.len(),
                    1,
                    1,
                ) < 0
                {
                    Err(PyErr::fetch(py))
                } else {
                    Ok(result)
                }
            }
        }
    }
}

#[inline]
fn try_truediv(dividend: BigInt, divisor: BigInt) -> PyResult<Fraction> {
    Fraction::new(dividend, divisor).ok_or_else(|| {
        PyZeroDivisionError::new_err(UNDEFINED_DIVISION_ERROR_MESSAGE)
    })
}

#[pymethods]
impl PyFraction {
    #[new]
    fn new(
        _numerator: Option<&PyAny>,
        _denominator: Option<&PyAny>,
    ) -> PyResult<Self> {
        match _denominator {
            Some(denominator) => match _numerator {
                Some(numerator) => {
                    match Fraction::new(
                        try_big_int_from_py_any(numerator)?,
                        try_big_int_from_py_any(denominator)?,
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
            None => try_py_fraction_from_value(_numerator),
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

    fn round(&self, tie_breaking: &PyTieBreaking) -> PyInt {
        PyInt((&self.0).round(tie_breaking.0))
    }

    fn __abs__(&self) -> PyFraction {
        PyFraction((&self.0).abs())
    }

    fn __add__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyFraction::type_object(py))? {
            Ok(Self(&self.0 + other.extract::<PyFraction>()?.0).into_py(py))
        } else {
            self.__radd__(other)
        }
    }

    fn __bool__(&self) -> bool {
        self.numerator().__bool__()
    }

    fn __ceil__(&self) -> PyInt {
        PyInt((&self.0).ceil())
    }

    fn __divmod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            try_divmod(&self.0, &divisor.0).map(|(quotient, remainder)| {
                (PyInt(quotient), Self(remainder)).into_py(py)
            })
        } else if let Ok(divisor) = to_py_ref::<PyInt>(divisor) {
            try_divmod(&self.0, &divisor.0).map(|(quotient, remainder)| {
                (PyInt(quotient), Self(remainder)).into_py(py)
            })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_divmod(&self.0, divisor).map(|(quotient, remainder)| {
                (PyInt(quotient), Self(remainder)).into_py(py)
            })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __float__(&self, py: Python) -> PyResult<PyObject> {
        match f64::try_from(&self.0) {
            Ok(float) => Ok(float.into_py(py)),
            Err(reason) => Err(PyOverflowError::new_err(reason.to_string())),
        }
    }

    fn __floor__(&self) -> PyInt {
        PyInt((&self.0).floor())
    }

    fn __floordiv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            (&self.0)
                .checked_div_euclid(&divisor.0)
                .map(|quotient| PyInt(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = to_py_ref::<PyInt>(divisor) {
            (&self.0)
                .checked_div_euclid(&divisor.0)
                .map(|quotient| PyInt(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            (&self.0)
                .checked_div_euclid(divisor)
                .map(|quotient| PyInt(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __getstate__(&self, py: Python) -> PyObject {
        (
            self.numerator().__getstate__(py),
            self.denominator().__getstate__(py),
        )
            .to_object(py)
    }

    fn __hash__(&self) -> ffi::Py_hash_t {
        let inverted_denominator = unsafe {
            self.0
                .denominator()
                .checked_pow_rem_euclid(
                    BigInt::from(HASH_MODULUS - 2),
                    BigInt::from(HASH_MODULUS),
                )
                .unwrap_unchecked()
        };
        let result = if inverted_denominator.is_zero() {
            HASH_INF
        } else {
            unsafe {
                ffi::Py_hash_t::try_from(
                    (self.0.numerator().abs() * inverted_denominator)
                        .checked_rem_euclid(BigInt::from(HASH_MODULUS))
                        .unwrap_unchecked(),
                )
                .unwrap_unchecked()
            }
        };
        if self.0.is_negative() {
            if result.is_one() {
                -2
            } else {
                -result
            }
        } else {
            result
        }
    }

    fn __mod__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            (&self.0)
                .checked_rem_euclid(&divisor.0)
                .map(|remainder| Self(remainder).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = to_py_ref::<PyInt>(divisor) {
            (&self.0)
                .checked_rem_euclid(&divisor.0)
                .map(|remainder| Self(remainder).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            (&self.0)
                .checked_rem_euclid(divisor)
                .map(|remainder| Self(remainder).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<Self>(other)
            .map(|other| Self(&self.0 * &other.0).into_py(py))
            .or_else(|_| self.__rmul__(other))
    }

    fn __neg__(&self) -> PyFraction {
        Self(-&self.0)
    }

    fn __pos__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __pow__(&self, exponent: &PyAny, modulo: &PyAny) -> PyResult<PyObject> {
        let py = exponent.py();
        if modulo.is_none() {
            if let Ok(exponent) = to_py_ref::<PyInt>(exponent) {
                (&self.0)
                    .checked_pow(&exponent.0)
                    .map(|power| Self(power).into_py(py))
                    .ok_or_else(|| {
                        PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )
                    })
            } else if let Ok(exponent) = try_big_int_from_py_integral(exponent)
            {
                (&self.0)
                    .checked_pow(exponent)
                    .map(|power| Self(power).into_py(py))
                    .ok_or_else(|| {
                        PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )
                    })
            } else {
                Ok(py.NotImplemented())
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<PyInt>(other)
            .map(|other| Self(&self.0 + &other.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(other)
                    .map(|other| Self(&self.0 + other).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __rdivmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = to_py_ref::<PyInt>(dividend) {
            try_divmod(&dividend.0, &self.0).map(|(quotient, remainder)| {
                (PyInt(quotient), Self(remainder)).into_py(py)
            })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            try_divmod(dividend, &self.0).map(|(quotient, remainder)| {
                (PyInt(quotient), Self(remainder)).into_py(py)
            })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Fraction({}, {})",
            self.numerator().__repr__(),
            self.denominator().__repr__()
        )
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<Self>(other)
            .map(|other| compare(&self.0, &other.0, op).into_py(py))
            .or_else(|_| {
                to_py_ref::<PyInt>(other)
                    .map(|other| compare(&self.0, &other.0, op).into_py(py))
                    .or_else(|_| {
                        try_big_int_from_py_integral(other)
                            .map(|other| {
                                compare(&self.0, &other, op).into_py(py)
                            })
                            .or_else(|_| Ok(py.NotImplemented()))
                    })
            })
    }

    fn __rfloordiv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = to_py_ref::<PyInt>(dividend) {
            (&dividend.0)
                .checked_div_euclid(&self.0)
                .map(|quotient| PyInt(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            dividend
                .checked_div_euclid(&self.0)
                .map(|quotient| PyInt(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmod__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = to_py_ref::<PyInt>(dividend) {
            (&dividend.0)
                .checked_rem_euclid(&self.0)
                .map(|remainder| Self(remainder).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            dividend
                .checked_rem_euclid(&self.0)
                .map(|remainder| Self(remainder).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmul__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        to_py_ref::<PyInt>(other)
            .map(|other| Self(&other.0 * &self.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(other)
                    .map(|other| Self(other * &self.0).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __round__(
        &self,
        digits: Option<&PyLong>,
        py: Python,
    ) -> PyResult<PyObject> {
        match digits {
            Some(digits) => {
                let digits = try_big_int_from_py_integral(digits)?;
                let is_digits_positive = digits.is_positive();
                let shift = unsafe {
                    BigInt::from(10)
                        .checked_pow(digits.abs())
                        .unwrap_unchecked()
                };
                if is_digits_positive {
                    Ok(Self(unsafe {
                        Fraction::new(
                            (&self.0 * &shift).round(TieBreaking::ToEven),
                            shift,
                        )
                        .unwrap_unchecked()
                    })
                    .into_py(py))
                } else {
                    Ok(Self(Fraction::from(
                        (&self.0 / &shift).round(TieBreaking::ToEven) * shift,
                    ))
                    .into_py(py))
                }
            }
            None => {
                Ok(PyInt((&self.0).round(TieBreaking::ToEven)).into_py(py))
            }
        }
    }

    fn __rsub__(&self, subtrahend: &PyAny) -> PyResult<PyObject> {
        let py = subtrahend.py();
        to_py_ref::<PyInt>(subtrahend)
            .map(|subtrahend| Self(&subtrahend.0 - &self.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(subtrahend)
                    .map(|subtrahend| Self(subtrahend - &self.0).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __rtruediv__(&self, dividend: &PyAny) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = to_py_ref::<PyInt>(dividend) {
            (&dividend.0)
                .checked_div(&self.0)
                .map(|quotient| Self(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            dividend
                .checked_div(&self.0)
                .map(|quotient| Self(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __setstate__(
        &mut self,
        state: (PyObject, PyObject),
        py: Python,
    ) -> PyResult<()> {
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
        to_py_ref::<Self>(minuend)
            .map(|minuend| Self(&self.0 - &minuend.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(minuend)
                    .map(|minuend| Self(&self.0 - minuend).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __truediv__(&self, divisor: &PyAny) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = to_py_ref::<Self>(divisor) {
            (&self.0)
                .checked_div(&divisor.0)
                .map(|quotient| Self(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            (&self.0)
                .checked_div(divisor)
                .map(|quotient| Self(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __trunc__(&self) -> PyInt {
        PyInt((&self.0).trunc())
    }
}

fn try_py_fraction_from_value(value: Option<&PyAny>) -> PyResult<PyFraction> {
    match value {
        Some(value) => {
            let py = value.py();
            if let Ok(value) = value.extract::<PyFraction>() {
                Ok(value)
            } else if let Ok(value) = try_big_int_from_py_any(value) {
                Ok(PyFraction(Fraction::from(value)))
            } else if let Ok((numerator, denominator)) = value
                .getattr(intern!(py, "numerator"))
                .and_then(try_big_int_from_py_any)
                .and_then(|numerator| {
                    value
                        .getattr(intern!(py, "denominator"))
                        .and_then(try_big_int_from_py_any)
                        .map(|denominator| (numerator, denominator))
                })
            {
                try_truediv(numerator, denominator).map(PyFraction)
            } else if let Ok(value) = value.downcast::<PyFloat>() {
                Fraction::try_from(value.value()).map(PyFraction).map_err(
                    |reason| match reason {
                        fraction::FromFloatConstructionError::NaN => {
                            PyValueError::new_err(reason.to_string())
                        }
                        _ => PyOverflowError::new_err(reason.to_string()),
                    },
                )
            } else {
                Err(PyTypeError::new_err(
                                format!("Value should be rational or floating point number, but found: {}",
                                        value.get_type().repr()?),
                            ))
            }
        }
        None => Ok(PyFraction(Fraction::zero())),
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

fn hash(value: &BigInt) -> usize {
    if value.digits().len() == 1 {
        return if value.is_negative() {
            usize::MAX
                - unsafe {
                    usize::try_from(
                        value.digits()[0]
                            + Digit::from(value.digits()[0].is_one()),
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
        result = ((result << DIGIT_BITNESS) & HASH_MODULUS)
            | (result >> (HASH_BITS - DIGIT_BITNESS));
        result += unsafe { usize::try_from(position).unwrap_unchecked() };
        if result >= HASH_MODULUS {
            result -= HASH_MODULUS;
        }
    }
    if value.is_negative() {
        result = usize::MAX - result + 1;
    };
    if result == usize::MAX {
        result - 1
    } else {
        result
    }
}

#[pymodule]
fn _crithm(py: Python, module: &PyModule) -> PyResult<()> {
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

fn to_py_ref<T: PyClass>(value: &PyAny) -> PyResult<PyRef<T>> {
    value.downcast::<PyCell<T>>()?.extract()
}

#[doc = include_str!("../README.md")]
type _DoctestReadme = ();
