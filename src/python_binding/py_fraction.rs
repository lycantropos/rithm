use super::py_big_int::{
    try_big_int_from_py_any, try_big_int_from_py_any_ref,
    try_big_int_from_py_integral, try_truediv, BigInt, PyBigInt,
};
use super::py_tie_breaking::PyTieBreaking;
use super::utils::{compare, try_divmod, HASH_INF, HASH_MODULUS};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyOverflowError, PyTypeError, PyValueError, PyZeroDivisionError,
};
use pyo3::prelude::{PyAnyMethods, PyFloatMethods};
use pyo3::types::{PyFloat, PyInt, PyTuple};
use pyo3::{
    intern, pyclass, pymethods, Bound, BoundObject, IntoPyObject, PyAny,
    PyObject, PyRef, PyResult, PyTypeInfo, Python,
};
use std::convert::TryFrom;
use traiter::numbers::{
    Abs, Ceil, CheckedDiv, CheckedDivEuclid, CheckedPow, CheckedPowRemEuclid,
    CheckedRemEuclid, Floor, Round, Signed, TieBreaking, Trunc, Unitary, Zero,
    Zeroable,
};

pub(super) type Fraction = crate::fraction::Fraction<BigInt>;

#[pyclass(name = "Fraction", module = "rithm.fraction", frozen)]
#[derive(Clone)]
pub(super) struct PyFraction(Fraction);

#[pymethods]
impl PyFraction {
    #[new]
    #[pyo3(signature = (numerator=None, denominator=None, /))]
    fn new(
        numerator: Option<&Bound<'_, PyAny>>,
        denominator: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        match denominator {
            Some(denominator) => match numerator {
                Some(numerator) => {
                    match Fraction::new(
                        try_big_int_from_py_any_ref(numerator)?,
                        try_big_int_from_py_any_ref(denominator)?,
                    ) {
                        Some(value) => Ok(PyFraction(value)),
                        None => Err(PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )),
                    }
                }
                None => Err(PyTypeError::new_err(
                    "Numerator should be of type `Int` or `int`, but found `None`",
                )),
            },
            None => try_py_fraction_from_value(numerator),
        }
    }

    #[getter]
    fn denominator(&self) -> PyBigInt {
        PyBigInt(self.0.denominator().clone())
    }

    #[getter]
    fn numerator(&self) -> PyBigInt {
        PyBigInt(self.0.numerator().clone())
    }

    #[pyo3(signature = (tie_breaking, /))]
    fn round(&self, tie_breaking: &PyTieBreaking) -> PyBigInt {
        PyBigInt((&self.0).round(tie_breaking.clone().into()))
    }

    fn __abs__(&self) -> PyFraction {
        PyFraction((&self.0).abs())
    }

    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(&PyFraction::type_object(py))? {
            Ok(Self(&self.0 + other.extract::<PyFraction>()?.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else {
            self.__radd__(other)
        }
    }

    fn __bool__(&self) -> bool {
        self.numerator().__bool__()
    }

    fn __ceil__(&self) -> PyBigInt {
        PyBigInt((&self.0).ceil())
    }

    fn __divmod__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            try_divmod(&self.0, &divisor.0).and_then(
                |(quotient, remainder)| {
                    Ok((PyBigInt(quotient), Self(remainder))
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                },
            )
        } else if let Ok(divisor) = divisor.extract::<PyRef<'_, PyBigInt>>() {
            try_divmod(&self.0, &divisor.0).and_then(
                |(quotient, remainder)| {
                    Ok((PyBigInt(quotient), Self(remainder))
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                },
            )
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            try_divmod(&self.0, divisor).and_then(|(quotient, remainder)| {
                Ok((PyBigInt(quotient), Self(remainder))
                    .into_pyobject(py)?
                    .into_any()
                    .unbind())
            })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __float__(&self, py: Python<'_>) -> PyResult<PyObject> {
        match f64::try_from(&self.0) {
            Ok(float) => Ok(float.into_pyobject(py)?.into_any().unbind()),
            Err(error) => Err(PyOverflowError::new_err(error.to_string())),
        }
    }

    fn __floor__(&self) -> PyBigInt {
        PyBigInt((&self.0).floor())
    }

    fn __floordiv__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            (&self.0)
                .checked_div_euclid(&divisor.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(PyBigInt(quotient)
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                })
        } else if let Ok(divisor) = divisor.extract::<PyRef<'_, PyBigInt>>() {
            (&self.0)
                .checked_div_euclid(&divisor.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(PyBigInt(quotient)
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            (&self.0)
                .checked_div_euclid(divisor)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(PyBigInt(quotient)
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __hash__(&self) -> isize {
        let inverted_denominator = unsafe {
            self.0
                .denominator()
                .checked_pow_rem_euclid(
                    BigInt::from(HASH_MODULUS - 2usize),
                    BigInt::from(HASH_MODULUS),
                )
                .unwrap_unchecked()
        };
        let result = if (&inverted_denominator).is_zero() {
            HASH_INF
        } else {
            unsafe {
                isize::try_from(
                    (self.0.numerator().abs() * inverted_denominator)
                        .checked_rem_euclid(BigInt::from(HASH_MODULUS))
                        .unwrap_unchecked(),
                )
                .unwrap_unchecked()
            }
        };
        if (&self.0).is_negative() {
            if result.is_one() {
                -2isize
            } else {
                -result
            }
        } else {
            result
        }
    }

    fn __mod__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            (&self.0)
                .checked_rem_euclid(&divisor.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|remainder| {
                    Ok(Self(remainder).into_pyobject(py)?.into_any().unbind())
                })
        } else if let Ok(divisor) = divisor.extract::<PyRef<'_, PyBigInt>>() {
            (&self.0)
                .checked_rem_euclid(&divisor.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|remainder| {
                    Ok(Self(remainder).into_pyobject(py)?.into_any().unbind())
                })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            (&self.0)
                .checked_rem_euclid(divisor)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|remainder| {
                    Ok(Self(remainder).into_pyobject(py)?.into_any().unbind())
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(Self(&self.0 * &other.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else {
            self.__rmul__(other)
        }
    }

    fn __neg__(&self) -> PyFraction {
        Self(-&self.0)
    }

    fn __pos__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __pow__(
        &self,
        exponent: &Bound<'_, PyAny>,
        modulo: &Bound<'_, PyAny>,
    ) -> PyResult<PyObject> {
        let py = exponent.py();
        if modulo.is_none() {
            if let Ok(exponent) = exponent.extract::<PyRef<'_, PyBigInt>>() {
                (&self.0)
                    .checked_pow(&exponent.0)
                    .ok_or_else(|| {
                        PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )
                    })
                    .and_then(|power| {
                        Ok(Self(power).into_pyobject(py)?.into_any().unbind())
                    })
            } else if let Ok(exponent) = try_big_int_from_py_integral(exponent)
            {
                (&self.0)
                    .checked_pow(exponent)
                    .ok_or_else(|| {
                        PyZeroDivisionError::new_err(
                            UNDEFINED_DIVISION_ERROR_MESSAGE,
                        )
                    })
                    .and_then(|power| {
                        Ok(Self(power).into_pyobject(py)?.into_any().unbind())
                    })
            } else {
                Ok(py.NotImplemented())
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, PyBigInt>>() {
            Ok(Self(&self.0 + &other.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(other) = try_big_int_from_py_integral(other) {
            Ok(Self(&self.0 + other).into_pyobject(py)?.into_any().unbind())
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rdivmod__(&self, dividend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyBigInt>>() {
            try_divmod(&dividend.0, &self.0).and_then(
                |(quotient, remainder)| {
                    Ok((PyBigInt(quotient), Self(remainder))
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                },
            )
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            try_divmod(dividend, &self.0).and_then(|(quotient, remainder)| {
                Ok((PyBigInt(quotient), Self(remainder))
                    .into_pyobject(py)?
                    .into_any()
                    .unbind())
            })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __getnewargs__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, [self.numerator(), self.denominator()])
    }

    fn __repr__(&self) -> String {
        format!(
            "{}({}, {})",
            Self::NAME,
            self.numerator().__repr__(),
            self.denominator().__repr__()
        )
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
    ) -> PyResult<PyObject> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(compare(&self.0, &other.0, op)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(other) = other.extract::<PyRef<'_, Self>>() {
            Ok(compare(&self.0, &other.0, op)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(other) = try_big_int_from_py_integral(other) {
            Ok(compare(&self.0, &other, op)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rfloordiv__(
        &self,
        dividend: &Bound<'_, PyAny>,
    ) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyBigInt>>() {
            (&dividend.0)
                .checked_div_euclid(&self.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(PyBigInt(quotient)
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            dividend
                .checked_div_euclid(&self.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(PyBigInt(quotient)
                        .into_pyobject(py)?
                        .into_any()
                        .unbind())
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmod__(&self, dividend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyBigInt>>() {
            (&dividend.0)
                .checked_rem_euclid(&self.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|remainder| {
                    Ok(Self(remainder).into_pyobject(py)?.into_any().unbind())
                })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            dividend
                .checked_rem_euclid(&self.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|remainder| {
                    Ok(Self(remainder).into_pyobject(py)?.into_any().unbind())
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        if let Ok(other) = other.extract::<PyRef<'_, PyBigInt>>() {
            Ok(Self(&other.0 * &self.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(other) = try_big_int_from_py_integral(other) {
            Ok(Self(other * &self.0).into_pyobject(py)?.into_any().unbind())
        } else {
            Ok(py.NotImplemented())
        }
    }

    #[pyo3(signature = (digits=None))]
    fn __round__(
        &self,
        digits: Option<&Bound<'_, PyInt>>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        match digits {
            Some(digits) => {
                let digits = try_big_int_from_py_integral(digits)?;
                let is_digits_positive = (&digits).is_positive();
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
                    .into_pyobject(py)?
                    .into_any()
                    .unbind())
                } else {
                    Ok(Self(Fraction::from(
                        (&self.0 / &shift).round(TieBreaking::ToEven) * shift,
                    ))
                    .into_pyobject(py)?
                    .into_any()
                    .unbind())
                }
            }
            None => Ok(PyBigInt((&self.0).round(TieBreaking::ToEven))
                .into_pyobject(py)?
                .into_any()
                .unbind()),
        }
    }

    fn __rsub__(&self, subtrahend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = subtrahend.py();
        if let Ok(subtrahend) = subtrahend.extract::<PyRef<'_, PyBigInt>>() {
            Ok(Self(&subtrahend.0 - &self.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(subtrahend) = try_big_int_from_py_integral(subtrahend)
        {
            Ok(Self(subtrahend - &self.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __rtruediv__(&self, dividend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyBigInt>>() {
            (&dividend.0)
                .checked_div(&self.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(Self(quotient).into_pyobject(py)?.into_any().unbind())
                })
        } else if let Ok(dividend) = try_big_int_from_py_integral(dividend) {
            dividend
                .checked_div(&self.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(Self(quotient).into_pyobject(py)?.into_any().unbind())
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __sub__(&self, minuend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = minuend.py();
        if let Ok(minuend) = minuend.extract::<PyRef<'_, Self>>() {
            Ok(Self(&self.0 - &minuend.0)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else if let Ok(minuend) = try_big_int_from_py_integral(minuend) {
            Ok(Self(&self.0 - minuend)
                .into_pyobject(py)?
                .into_any()
                .unbind())
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __truediv__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            (&self.0)
                .checked_div(&divisor.0)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(Self(quotient).into_pyobject(py)?.into_any().unbind())
                })
        } else if let Ok(divisor) = try_big_int_from_py_integral(divisor) {
            (&self.0)
                .checked_div(divisor)
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
                .and_then(|quotient| {
                    Ok(Self(quotient).into_pyobject(py)?.into_any().unbind())
                })
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __trunc__(&self) -> PyBigInt {
        PyBigInt((&self.0).trunc())
    }
}

impl From<Fraction> for PyFraction {
    fn from(value: Fraction) -> Self {
        PyFraction(value)
    }
}

fn try_py_fraction_from_value(
    value: Option<&Bound<'_, PyAny>>,
) -> PyResult<PyFraction> {
    match value {
        Some(value) => {
            let py = value.py();
            if let Ok(value) = value.extract::<PyFraction>() {
                Ok(value)
            } else if let Ok(value) = try_big_int_from_py_any_ref(value) {
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
                    |error| match error {
                        crate::fraction::FromFloatConstructionError::NaN => {
                            PyValueError::new_err(error.to_string())
                        }
                        _ => PyOverflowError::new_err(error.to_string()),
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
