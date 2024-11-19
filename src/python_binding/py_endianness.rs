use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyAnyMethods;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyTuple;
use pyo3::{
    pyclass, pymethods, Bound, Py, PyAny, PyResult, PyTypeInfo, Python,
};
use traiter::numbers::Endianness;

#[pyclass(name = "Endianness", module = "rithm.enums", frozen)]
#[derive(Clone)]
pub(super) struct PyEndianness(Endianness);

#[allow(non_snake_case)]
#[pymethods]
impl PyEndianness {
    #[classattr]
    fn BIG(py: Python<'_>) -> Py<PyEndianness> {
        to_py_endianness_values(py)[0].clone_ref(py)
    }

    #[classattr]
    fn LITTLE(py: Python<'_>) -> Py<PyEndianness> {
        to_py_endianness_values(py)[1].clone_ref(py)
    }

    #[new]
    #[pyo3(signature = (value, /))]
    fn new(value: &Bound<'_, PyAny>, py: Python<'_>) -> PyResult<Py<Self>> {
        let values = to_py_endianness_values(py);
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
            Endianness::Big => 0,
            Endianness::Little => 1,
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
                Endianness::Big => "BIG",
                Endianness::Little => "LITTLE",
            }
        )
    }
}

impl From<PyEndianness> for Endianness {
    fn from(value: PyEndianness) -> Self {
        value.0
    }
}

fn to_py_endianness_values(py: Python<'_>) -> &[Py<PyEndianness>; 2usize] {
    static VALUES: GILOnceCell<[Py<PyEndianness>; 2usize]> =
        GILOnceCell::new();
    VALUES.get_or_init(py, || {
        [
            Bound::new(py, PyEndianness(Endianness::Big))
                .unwrap()
                .into(),
            Bound::new(py, PyEndianness(Endianness::Little))
                .unwrap()
                .into(),
        ]
    })
}
