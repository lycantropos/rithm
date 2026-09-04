use super::py_endianness::PyEndianness;
use super::py_fraction::{Fraction, PyFraction};
use super::utils::{
    compare, try_divmod, try_floordiv, try_lshift, try_mod, try_rshift,
    HASH_BITS, HASH_MODULUS,
};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use pyo3::exceptions::{
    PyMemoryError, PyOverflowError, PyValueError, PyZeroDivisionError,
};
use pyo3::prelude::{PyAnyMethods, PyFloatMethods};
use pyo3::types::{
    PyBytes, PyFloat, PyInt, PyString, PyTuple, PyType, PyTypeMethods,
};
use pyo3::{
    pyclass, pymethods, Bound, BoundObject, IntoPyObject, PyAny, PyRef,
    PyResult, PyTypeInfo, Python,
};
use pyo3_ffi as ffi;
use std::convert::TryFrom;
use traiter::numbers::{
    Abs, BitLength, CheckedDivRemEuclid, CheckedPow, CheckedPowRemEuclid,
    CheckedShl, Endianness, FromBytes, FromStrRadix, Gcd, IsPowerOfTwo, One,
    Parity, Signed, ToBytes, Unitary, Zero, Zeroable,
};

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const DIGIT_BITNESS: usize = (Digit::BITS - 1u32) as usize;
const _: () =
    assert!(crate::big_int::is_valid_digit_bitness::<Digit, DIGIT_BITNESS>());

pub(super) type BigInt = crate::big_int::BigInt<Digit, DIGIT_BITNESS>;

#[pyclass(name = "Int", module = "rithm.integer", from_py_object, frozen)]
#[derive(Clone)]
pub(super) struct PyBigInt(pub(super) BigInt);

#[pymethods]
impl PyBigInt {
    #[new]
    #[pyo3(signature = (value=None, base=None, /))]
    fn new(
        value: Option<&Bound<'_, PyAny>>,
        base: Option<&Bound<'_, PyInt>>,
    ) -> PyResult<Self> {
        match value {
            None => Ok(Self(BigInt::zero())),
            Some(value) => {
                let py = value.py();
                if base.is_some()
                    || value.is_instance(&PyString::type_object(py))?
                {
                    let base = match base {
                        Some(base) => {
                            base.extract::<u32>().or(Err(PyValueError::new_err(format!(
                                "Base should be zero or in range from {} to {}, but found: {}.",
                                crate::big_int::MIN_REPRESENTABLE_BASE,
                                crate::big_int::MAX_REPRESENTABLE_BASE,
                                base.repr()?
                            ))))?
                        }
                        None => 10,
                    };
                    match BigInt::from_str_radix(
                        value.extract::<&str>()?,
                        base,
                    ) {
                        Ok(value) => Ok(Self(value)),
                        Err(error) => {
                            Err(PyValueError::new_err(error.to_string()))
                        }
                    }
                } else if value.is_instance(&PyFloat::type_object(py))? {
                    Ok(Self(
                        BigInt::try_from(
                            value.extract::<Bound<'_, PyFloat>>()?.value(),
                        )
                            .map_err(
                                |error| match error {
                                    crate::big_int::TryFromFloatError::Infinity => {
                                        PyOverflowError::new_err(error.to_string())
                                    }
                                    crate::big_int::TryFromFloatError::NaN => {
                                        PyValueError::new_err(error.to_string())
                                    }
                                },
                            )?,
                    ))
                } else {
                    try_big_int_from_py_any_ref(value).map(Self)
                }
            }
        }
    }

    #[classmethod]
    #[pyo3(signature = (bytes, endianness, /))]
    fn from_bytes(
        _cls: &Bound<'_, PyType>,
        bytes: Vec<u8>,
        endianness: &PyEndianness,
    ) -> Self {
        Self(BigInt::from_bytes(
            bytes.as_slice(),
            endianness.clone().into(),
        ))
    }

    #[getter]
    fn denominator(_slf: PyRef<'_, Self>) -> Self {
        Self(BigInt::one())
    }

    #[getter]
    fn numerator(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn bit_length(&self) -> Self {
        Self(self.0.bit_length())
    }

    fn is_power_of_two(&self) -> bool {
        (&self.0).is_power_of_two()
    }

    #[pyo3(text_signature = "($self, other, /)")]
    fn gcd(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self((&self.0).gcd(&try_big_int_from_py_integral(other)?)))
    }

    #[pyo3(signature = (endianness, /))]
    fn to_bytes<'py>(
        &self,
        endianness: &PyEndianness,
        py: Python<'py>,
    ) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.to_bytes(endianness.clone().into()))
    }

    fn __abs__(&self) -> Self {
        Self((&self.0).abs())
    }

    fn __add__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'py, Self>>() {
            Ok(Self(&self.0 + &other.0).into_pyobject(py)?.into_any())
        } else {
            self.__radd__(other)
        }
    }

    fn __and__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(Self(&self.0 & &other.0).into_pyobject(py)?.into_any())
        } else {
            self.__rand__(other)
        }
    }

    pub(super) fn __bool__(&self) -> bool {
        !(&self.0).is_zero()
    }

    fn __ceil__<'py>(slf: PyRef<'py, Self>) -> PyRef<'py, Self> {
        slf
    }

    fn __divmod__<'py>(
        &self,
        divisor: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            try_divmod(&self.0, &divisor.0).and_then(
                |(quotient, remainder)| {
                    Ok((Self(quotient), Self(remainder))
                        .into_pyobject(py)?
                        .into_any())
                },
            )
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_divmod(&self.0, divisor).and_then(|(quotient, remainder)| {
                Ok((Self(quotient), Self(remainder))
                    .into_pyobject(py)?
                    .into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __float__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        f64::try_from(&self.0)
            .map_err(|error| PyOverflowError::new_err(error.to_string()))
            .and_then(|result| Ok(result.into_pyobject(py)?.into_any()))
    }

    fn __floor__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __floordiv__<'py>(
        &self,
        divisor: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            try_floordiv(&self.0, &divisor.0).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_floordiv(&self.0, divisor).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __hash__(&self) -> isize {
        hash(&self.0) as isize
    }

    fn __index__<'py>(&self, py: Python<'py>) -> Bound<'py, PyAny> {
        to_py_long::<BigInt>(&self.0, py)
    }

    fn __int__<'py>(&self, py: Python<'py>) -> Bound<'py, PyAny> {
        to_py_long::<BigInt>(&self.0, py)
    }

    fn __invert__(&self) -> Self {
        Self(!&self.0)
    }

    fn __lshift__<'py>(
        &self,
        shift: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = shift.py();
        if let Ok(shift) = shift.extract::<PyRef<'_, Self>>() {
            try_lshift(&self.0, &shift.0).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else if let Ok(shift) = try_big_int_from_py_integral(shift) {
            try_lshift(&self.0, shift).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __mod__<'py>(
        &self,
        divisor: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            try_mod(&self.0, &divisor.0).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_mod(&self.0, divisor).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __mul__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(Self(&self.0 * &other.0).into_pyobject(py)?.into_any())
        } else {
            self.__rmul__(other)
        }
    }

    fn __neg__(&self) -> Self {
        Self(-&self.0)
    }

    fn __or__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(Self(&self.0 | &other.0).into_pyobject(py)?.into_any())
        } else {
            self.__ror__(other)
        }
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __pow__<'py>(
        &self,
        exponent: &Bound<'py, PyAny>,
        divisor: Option<&Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = exponent.py();
        if let Ok(exponent) = try_big_int_from_py_any_ref(exponent) {
            match divisor {
                Some(divisor) => {
                    if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
                        (&self.0)
                            .checked_pow_rem_euclid(exponent, &divisor.0)
                            .map_err(|error| {
                                PyValueError::new_err(error.to_string())
                            })
                            .and_then(|remainder| {
                                Ok(Self(remainder)
                                    .into_pyobject(py)?
                                    .into_any())
                            })
                    } else if let Ok(divisor) =
                        try_big_int_from_py_integral(divisor)
                    {
                        (&self.0)
                            .checked_pow_rem_euclid(exponent, divisor)
                            .map_err(|error| {
                                PyValueError::new_err(error.to_string())
                            })
                            .and_then(|remainder| {
                                Ok(Self(remainder)
                                    .into_pyobject(py)?
                                    .into_any())
                            })
                    } else {
                        Ok(pyo3::types::PyNotImplemented::get(py)
                            .to_owned()
                            .into_any())
                    }
                }
                None => {
                    if (&exponent).is_negative() {
                        try_pow_negative_exponent(self.0.clone(), exponent, py)
                    } else {
                        Ok(Self(pow_non_negative_exponent(&self.0, &exponent))
                            .into_pyobject(py)?
                            .into_any())
                    }
                }
            }
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __radd__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = try_big_int_from_py_integral(other) {
            Ok(Self(other + &self.0).into_pyobject(py)?.into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rand__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = try_big_int_from_py_integral(other) {
            Ok(Self(other & &self.0).into_pyobject(py)?.into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rdivmod__<'py>(
        &self,
        dividend: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = dividend.py();
        if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            try_divmod(dividend, &self.0).and_then(|(quotient, remainder)| {
                Ok((Self(quotient), Self(remainder))
                    .into_pyobject(py)?
                    .into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __getnewargs__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, [self.__int__(py)])
    }

    pub(super) fn __repr__<'py>(&self, py: Python<'py>) -> PyResult<String> {
        Ok(format!("{}({})", Self::type_object(py).name()?, self.0))
    }

    fn __rfloordiv__<'py>(
        &self,
        dividend: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = dividend.py();
        if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            try_floordiv(dividend, &self.0).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __richcmp__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
        op: pyo3::basic::CompareOp,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(compare(&self.0, &other.0, op)
                .into_pyobject(py)?
                .into_bound()
                .into_any())
        } else if let Ok(other) = try_big_int_from_py_integral(other) {
            Ok(compare(&self.0, &other, op)
                .into_pyobject(py)?
                .into_bound()
                .into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rlshift__<'py>(
        &self,
        base: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = base.py();
        if let Ok(base) = try_big_int_from_py_integral(base) {
            try_lshift(base, &self.0).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rmod__<'py>(
        &self,
        dividend: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = dividend.py();
        if dividend.is_instance(&PyInt::type_object(py))? {
            try_mod(try_big_int_from_py_integral(dividend)?, &self.0).and_then(
                |result| Ok(Self(result).into_pyobject(py)?.into_any()),
            )
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rmul__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if other.is_instance(&PyInt::type_object(py))? {
            Ok(Self(try_big_int_from_py_integral(other)? * &self.0)
                .into_pyobject(py)?
                .into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __ror__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if other.is_instance(&PyInt::type_object(py))? {
            Ok(Self(&self.0 | try_big_int_from_py_integral(other)?)
                .into_pyobject(py)?
                .into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    #[pyo3(signature = (digits=None))]
    fn __round__(
        &self,
        digits: Option<&Bound<'_, PyInt>>,
        py: Python<'_>,
    ) -> PyResult<Self> {
        Ok(match digits {
            Some(digits) => {
                if digits.lt(0.into_pyobject(py).unwrap())? {
                    let ten_to_digits_power = unsafe {
                        BigInt::from(10u8)
                            .checked_pow(-try_big_int_from_py_integral(
                                digits,
                            )?)
                            .unwrap_unchecked()
                    };
                    Self(
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

    fn __rpow__<'py>(
        &self,
        base: &Bound<'py, PyAny>,
        divisor: Option<&Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = base.py();
        let base = if base.is_instance(&PyInt::type_object(py))? {
            try_big_int_from_py_integral(base)?
        } else {
            return Ok(pyo3::types::PyNotImplemented::get(py)
                .to_owned()
                .into_any());
        };
        match divisor {
            Some(divisor) => {
                if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
                    base.checked_pow_rem_euclid(&self.0, &divisor.0)
                        .map_err(|error| {
                            PyValueError::new_err(error.to_string())
                        })
                        .and_then(|remainder| {
                            Ok(Self(remainder).into_pyobject(py)?.into_any())
                        })
                } else if let Ok(divisor) =
                    try_big_int_from_py_integral(divisor)
                {
                    base.checked_pow_rem_euclid(&self.0, divisor)
                        .map_err(|error| {
                            PyValueError::new_err(error.to_string())
                        })
                        .and_then(|remainder| {
                            Ok(Self(remainder).into_pyobject(py)?.into_any())
                        })
                } else {
                    Ok(pyo3::types::PyNotImplemented::get(py)
                        .to_owned()
                        .into_any())
                }
            }
            None => {
                if (&self.0).is_negative() {
                    try_pow_negative_exponent(base, self.0.clone(), py)
                } else {
                    Ok(Self(pow_non_negative_exponent(&base, &self.0))
                        .into_pyobject(py)?
                        .into_any())
                }
            }
        }
    }

    fn __rrshift__<'py>(
        &self,
        base: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = base.py();
        if base.is_instance(&PyInt::type_object(py))? {
            try_rshift(try_big_int_from_py_integral(base)?, &self.0).and_then(
                |result| Ok(Self(result).into_pyobject(py)?.into_any()),
            )
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rshift__<'py>(
        &self,
        shift: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = shift.py();
        if let Ok(shift) = shift.extract::<PyRef<'_, Self>>() {
            try_rshift(&self.0, &shift.0).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else if let Ok(shift) = try_big_int_from_py_integral(shift) {
            try_rshift(&self.0, shift).and_then(|result| {
                Ok(Self(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rsub__<'py>(
        &self,
        minuend: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = minuend.py();
        if let Ok(minuend) = try_big_int_from_py_integral(minuend) {
            Ok(Self(minuend - &self.0).into_pyobject(py)?.into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rtruediv__<'py>(
        &self,
        dividend: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = dividend.py();
        if dividend.is_instance(&PyInt::type_object(py))? {
            let dividend = try_big_int_from_py_integral(dividend)?;
            try_truediv(dividend, self.0.clone()).and_then(|result| {
                Ok(PyFraction::from(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __rxor__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if other.is_instance(&PyInt::type_object(py))? {
            Ok(Self(&self.0 ^ try_big_int_from_py_integral(other)?)
                .into_pyobject(py)?
                .into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __sub__<'py>(
        &self,
        subtrahend: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = subtrahend.py();
        if let Ok(subtrahend) = subtrahend.extract::<PyRef<'_, Self>>() {
            Ok(Self(&self.0 - &subtrahend.0).into_pyobject(py)?.into_any())
        } else if let Ok(subtrahend) = try_big_int_from_py_integral(subtrahend)
        {
            Ok(Self(&self.0 - subtrahend).into_pyobject(py)?.into_any())
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __truediv__<'py>(
        &self,
        divisor: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = divisor.py();
        if let Ok(divisor) = try_big_int_from_py_any_ref(divisor) {
            try_truediv(self.0.clone(), divisor).and_then(|result| {
                Ok(PyFraction::from(result).into_pyobject(py)?.into_any())
            })
        } else {
            Ok(pyo3::types::PyNotImplemented::get(py).to_owned().into_any())
        }
    }

    fn __trunc__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __xor__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = other.py();
        if other.is_instance(&Self::type_object(py))? {
            Ok(Self(&self.0 ^ other.extract::<Self>()?.0)
                .into_pyobject(py)?
                .into_any())
        } else {
            self.__rxor__(other)
        }
    }
}

pub(super) fn try_big_int_from_py_integral(
    value: &pyo3::Bound<'_, pyo3::PyAny>,
) -> pyo3::PyResult<BigInt> {
    use pyo3::types::PyAnyMethods;

    let ptr = value.as_ptr();
    let py = value.py();
    let py_long = unsafe {
        let ptr = pyo3::ffi::PyNumber_Index(ptr);
        if ptr.is_null() {
            return Err(pyo3::PyErr::fetch(py));
        }
        pyo3::Bound::from_owned_ptr(py, ptr)
    };
    let bits_count = py_long
        .call_method0(pyo3::intern!(py, "bit_length"))
        .and_then(|any| any.extract::<usize>())?;
    if bits_count == 0 {
        Ok(BigInt::zero())
    } else {
        let bytes_count = bits_count / (u8::BITS as usize) + 1;
        let mut buffer = vec![0u8; bytes_count];
        if unsafe {
            cfg_select! {
                any(Py_3_14, all(Py_3_13, not(Py_LIMITED_API))) => {
                    pyo3::ffi::PyLong_AsNativeBytes(
                        py_long.as_ptr().cast(),
                        buffer.as_mut_ptr().cast(),
                        bytes_count as isize,
                        pyo3::ffi::Py_ASNATIVEBYTES_LITTLE_ENDIAN,
                    )
                }
                _ => {
                    pyo3::ffi::_PyLong_AsByteArray(
                        py_long.as_ptr().cast::<pyo3::ffi::PyLongObject>(),
                        buffer.as_mut_ptr(),
                        buffer.len(),
                        1,
                        1,
                    )
                }
            }
        } < 0
        {
            Err(pyo3::PyErr::fetch(py_long.py()))
        } else {
            Ok(BigInt::from_bytes(&buffer, Endianness::Little))
        }
    }
}

pub(super) fn try_py_any_to_big_int(
    value: Bound<'_, PyAny>,
) -> PyResult<BigInt> {
    try_big_int_from_py_any_ref(&value)
}

pub(super) fn try_big_int_from_py_any_ref(
    value: &Bound<'_, PyAny>,
) -> PyResult<BigInt> {
    value
        .extract::<PyBigInt>()
        .map(|value| value.0)
        .or_else(|_| try_big_int_from_py_integral(value))
}

pub(super) fn try_truediv(
    dividend: BigInt,
    divisor: BigInt,
) -> PyResult<Fraction> {
    Fraction::new(dividend, divisor).ok_or_else(|| {
        PyZeroDivisionError::new_err(UNDEFINED_DIVISION_ERROR_MESSAGE)
    })
}

fn hash(value: &BigInt) -> usize {
    if value.digits().len() == 1usize {
        return if value.is_negative() {
            usize::MAX
                - unsafe {
                    usize::try_from(
                        value.digits()[0]
                            + Digit::from(value.digits()[0].is_one()),
                    )
                    .unwrap_unchecked()
                }
                + 1usize
        } else {
            unsafe { usize::try_from(value.digits()[0]).unwrap_unchecked() }
        };
    };
    let mut result = 0usize;
    for &position in value.digits().iter().rev() {
        result = ((result << DIGIT_BITNESS) & HASH_MODULUS)
            | (result >> (HASH_BITS - DIGIT_BITNESS));
        result += unsafe { usize::try_from(position).unwrap_unchecked() };
        if result >= HASH_MODULUS {
            result -= HASH_MODULUS;
        }
    }
    if value.is_negative() {
        result = usize::MAX - result + 1usize;
    };
    if result == usize::MAX {
        result - 1usize
    } else {
        result
    }
}

fn pow_non_negative_exponent(base: &BigInt, exponent: &BigInt) -> BigInt {
    debug_assert!(!exponent.is_negative());
    unsafe { base.checked_pow(exponent).unwrap_unchecked() }
}

fn to_py_long<'a, 'py, T>(value: &'a T, py: Python<'py>) -> Bound<'py, PyAny>
where
    &'a T: ToBytes<Output = Vec<u8>>,
{
    let buffer = value.to_bytes(Endianness::Little);
    unsafe {
        Bound::<'py, PyAny>::from_owned_ptr(
            py,
            cfg_select! {
                any(Py_3_14, all(Py_3_13, not(Py_LIMITED_API))) => {
                    ffi::PyLong_FromNativeBytes(buffer.as_ptr().cast(), buffer.len(), pyo3::ffi::Py_ASNATIVEBYTES_LITTLE_ENDIAN)
                }
                _ => {
                    ffi::_PyLong_FromByteArray(buffer.as_ptr(), buffer.len(), 1, 1)
                }
            },
        )
    }
}

fn try_mod_to_near(dividend: &BigInt, divisor: &BigInt) -> PyResult<BigInt> {
    let (quotient, remainder) = match dividend.checked_div_rem_euclid(divisor)
    {
        Some((quotient, remainder)) => Ok((quotient, remainder)),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }?;
    let double_remainder = (&remainder).checked_shl(BigInt::one()).map_err(
        |error| match error {
            crate::big_int::ShlError::NegativeShift => {
                PyValueError::new_err(error.to_string())
            }
            crate::big_int::ShlError::OutOfMemory => {
                PyMemoryError::new_err(error.to_string())
            }
            crate::big_int::ShlError::TooLarge => {
                PyOverflowError::new_err(error.to_string())
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

fn try_pow_negative_exponent(
    base: BigInt,
    exponent: BigInt,
    py: Python<'_>,
) -> PyResult<Bound<'_, PyAny>> {
    debug_assert!((&exponent).is_negative());
    match Fraction::from(base).checked_pow(exponent) {
        Some(power) => {
            Ok(PyFraction::from(power).into_pyobject(py)?.into_any())
        }
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}
