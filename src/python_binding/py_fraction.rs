use super::py_int::{
    try_big_int_from_bound_py_any, try_big_int_from_bound_py_any_ref,
    try_big_int_from_py_integral, try_truediv, BigInt, PyInt,
};
use super::py_tie_breaking::PyTieBreaking;
use super::utils::{compare, try_divmod, HASH_INF, HASH_MODULUS};
use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyOverflowError, PyTypeError, PyValueError, PyZeroDivisionError,
};
use pyo3::prelude::{PyAnyMethods, PyFloatMethods};
use pyo3::types::{PyFloat, PyLong, PyTuple};
use pyo3::{
    intern, pyclass, pymethods, Bound, IntoPy, PyAny, PyObject, PyRef,
    PyResult, PyTypeInfo, Python,
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
                        try_big_int_from_bound_py_any_ref(numerator)?,
                        try_big_int_from_bound_py_any_ref(denominator)?,
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
    fn denominator(&self) -> PyInt {
        PyInt(self.0.denominator().clone())
    }

    #[getter]
    fn numerator(&self) -> PyInt {
        PyInt(self.0.numerator().clone())
    }

    #[pyo3(signature = (tie_breaking, /))]
    fn round(&self, tie_breaking: &PyTieBreaking) -> PyInt {
        PyInt((&self.0).round(tie_breaking.clone().into()))
    }

    fn __abs__(&self) -> PyFraction {
        PyFraction((&self.0).abs())
    }

    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(&PyFraction::type_object_bound(py))? {
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

    fn __divmod__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            try_divmod(&self.0, &divisor.0).map(|(quotient, remainder)| {
                (PyInt(quotient), Self(remainder)).into_py(py)
            })
        } else if let Ok(divisor) = divisor.extract::<PyRef<'_, PyInt>>() {
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

    fn __float__(&self, py: Python<'_>) -> PyResult<PyObject> {
        match f64::try_from(&self.0) {
            Ok(float) => Ok(float.into_py(py)),
            Err(error) => Err(PyOverflowError::new_err(error.to_string())),
        }
    }

    fn __floor__(&self) -> PyInt {
        PyInt((&self.0).floor())
    }

    fn __floordiv__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
            (&self.0)
                .checked_div_euclid(&divisor.0)
                .map(|quotient| PyInt(quotient).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = divisor.extract::<PyRef<'_, PyInt>>() {
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
                .map(|remainder| Self(remainder).into_py(py))
                .ok_or_else(|| {
                    PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )
                })
        } else if let Ok(divisor) = divisor.extract::<PyRef<'_, PyInt>>() {
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

    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        other
            .extract::<PyRef<'_, Self>>()
            .map(|other| Self(&self.0 * &other.0).into_py(py))
            .or_else(|_| self.__rmul__(other))
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
            if let Ok(exponent) = exponent.extract::<PyRef<'_, PyInt>>() {
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

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        other
            .extract::<PyRef<'_, PyInt>>()
            .map(|other| Self(&self.0 + &other.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(other)
                    .map(|other| Self(&self.0 + other).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __rdivmod__(&self, dividend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyInt>>() {
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

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> Bound<'py, PyTuple> {
        PyTuple::new_bound(
            py,
            [self.numerator().into_py(py), self.denominator().into_py(py)],
        )
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
        other
            .extract::<PyRef<'_, Self>>()
            .map(|other| compare(&self.0, &other.0, op).into_py(py))
            .or_else(|_| {
                other
                    .extract::<PyRef<'_, PyInt>>()
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

    fn __rfloordiv__(
        &self,
        dividend: &Bound<'_, PyAny>,
    ) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyInt>>() {
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

    fn __rmod__(&self, dividend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyInt>>() {
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

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = other.py();
        other
            .extract::<PyRef<'_, PyInt>>()
            .map(|other| Self(&other.0 * &self.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(other)
                    .map(|other| Self(other * &self.0).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    #[pyo3(signature = (digits=None))]
    fn __round__(
        &self,
        digits: Option<&Bound<'_, PyLong>>,
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

    fn __rsub__(&self, subtrahend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = subtrahend.py();
        subtrahend
            .extract::<PyRef<'_, PyInt>>()
            .map(|subtrahend| Self(&subtrahend.0 - &self.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(subtrahend)
                    .map(|subtrahend| Self(subtrahend - &self.0).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __rtruediv__(&self, dividend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = dividend.py();
        if let Ok(dividend) = dividend.extract::<PyRef<'_, PyInt>>() {
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

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __sub__(&self, minuend: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = minuend.py();
        minuend
            .extract::<PyRef<'_, Self>>()
            .map(|minuend| Self(&self.0 - &minuend.0).into_py(py))
            .or_else(|_| {
                try_big_int_from_py_integral(minuend)
                    .map(|minuend| Self(&self.0 - minuend).into_py(py))
                    .or_else(|_| Ok(py.NotImplemented()))
            })
    }

    fn __truediv__(&self, divisor: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = divisor.py();
        if let Ok(divisor) = divisor.extract::<PyRef<'_, Self>>() {
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

fn try_py_fraction_from_value(
    value: Option<&Bound<'_, PyAny>>,
) -> PyResult<PyFraction> {
    match value {
        Some(value) => {
            let py = value.py();
            if let Ok(value) = value.extract::<PyFraction>() {
                Ok(value)
            } else if let Ok(value) = try_big_int_from_bound_py_any_ref(value)
            {
                Ok(PyFraction(Fraction::from(value)))
            } else if let Ok((numerator, denominator)) = value
                .getattr(intern!(py, "numerator"))
                .and_then(try_big_int_from_bound_py_any)
                .and_then(|numerator| {
                    value
                        .getattr(intern!(py, "denominator"))
                        .and_then(try_big_int_from_bound_py_any)
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

impl From<Fraction> for PyFraction {
    fn from(value: Fraction) -> Self {
        PyFraction(value)
    }
}
