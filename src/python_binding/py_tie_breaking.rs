use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyAnyMethods;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyTuple;
use pyo3::{
    pyclass, pymethods, Bound, Py, PyAny, PyResult, PyTypeInfo, Python,
};
use traiter::numbers::TieBreaking;

#[pyclass(name = "TieBreaking", module = "rithm.enums", frozen)]
#[derive(Clone)]
pub(super) struct PyTieBreaking(TieBreaking);

#[allow(non_snake_case)]
#[pymethods]
impl PyTieBreaking {
    #[classattr]
    fn AWAY_FROM_ZERO(py: Python<'_>) -> Py<PyTieBreaking> {
        to_py_tie_breaking_values(py)[0].clone_ref(py)
    }

    #[classattr]
    fn TOWARD_ZERO(py: Python<'_>) -> Py<PyTieBreaking> {
        to_py_tie_breaking_values(py)[1].clone_ref(py)
    }

    #[classattr]
    fn TO_EVEN(py: Python<'_>) -> Py<PyTieBreaking> {
        to_py_tie_breaking_values(py)[2].clone_ref(py)
    }

    #[classattr]
    fn TO_ODD(py: Python<'_>) -> Py<PyTieBreaking> {
        to_py_tie_breaking_values(py)[3].clone_ref(py)
    }

    #[new]
    #[pyo3(signature = (value, /))]
    fn new(value: &Bound<'_, PyAny>, py: Python<'_>) -> PyResult<Py<Self>> {
        let values = to_py_tie_breaking_values(py);
        match value.extract::<usize>() {
            Ok(value) if value < values.len() => {
                Ok(values[value].clone_ref(py))
            }
            _ => Err(PyValueError::new_err(format!(
                "{} is not a valid {}",
                value.repr()?,
                Self::NAME
            ))),
        }
    }

    #[getter]
    fn value(&self) -> u8 {
        match self.0 {
            TieBreaking::AwayFromZero => 0,
            TieBreaking::ToEven => 2,
            TieBreaking::ToOdd => 3,
            TieBreaking::TowardZero => 1,
        }
    }

    fn __getnewargs__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, [self.value()])
    }

    fn __repr__(&self) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                TieBreaking::AwayFromZero => "AWAY_FROM_ZERO",
                TieBreaking::ToEven => "TO_EVEN",
                TieBreaking::ToOdd => "TO_ODD",
                TieBreaking::TowardZero => "TOWARD_ZERO",
            }
        )
    }
}

impl From<PyTieBreaking> for TieBreaking {
    fn from(value: PyTieBreaking) -> Self {
        value.0
    }
}

fn to_py_tie_breaking_values(py: Python<'_>) -> &[Py<PyTieBreaking>; 4usize] {
    static VALUES: GILOnceCell<[Py<PyTieBreaking>; 4usize]> =
        GILOnceCell::new();
    VALUES.get_or_init(py, || {
        [
            Bound::new(py, PyTieBreaking(TieBreaking::AwayFromZero))
                .unwrap()
                .into(),
            Bound::new(py, PyTieBreaking(TieBreaking::TowardZero))
                .unwrap()
                .into(),
            Bound::new(py, PyTieBreaking(TieBreaking::ToEven))
                .unwrap()
                .into(),
            Bound::new(py, PyTieBreaking(TieBreaking::ToOdd))
                .unwrap()
                .into(),
        ]
    })
}
