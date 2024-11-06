mod py_endianness;
mod py_fraction;
mod py_int;
mod py_tie_breaking;
mod utils;

use py_endianness::PyEndianness;
use py_fraction::PyFraction;
use py_int::PyInt;
use py_tie_breaking::PyTieBreaking;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use pyo3::types::{PyAnyMethods, PyModuleMethods};
use pyo3::{intern, Bound, PyTypeInfo};

#[pymodule]
fn _crithm(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.setattr(intern!(py, "__doc__"), env!("CARGO_PKG_DESCRIPTION"))?;
    module.setattr(intern!(py, "__version__"), env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyEndianness>()?;
    module.add_class::<PyFraction>()?;
    module.add_class::<PyInt>()?;
    module.add_class::<PyTieBreaking>()?;
    let numbers_module = py.import_bound("numbers")?;
    let integral_cls = numbers_module.getattr(intern!(py, "Integral"))?;
    let rational_cls = numbers_module.getattr(intern!(py, "Rational"))?;
    integral_cls.call_method1("register", (PyInt::type_object_bound(py),))?;
    rational_cls
        .call_method1("register", (PyFraction::type_object_bound(py),))?;
    Ok(())
}
