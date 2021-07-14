use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn _rithm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.setattr("__version__", "0.0.0")?;
    m.setattr("__doc__", "Arbitrary precision arithmetics.")?;
    Ok(())
}
