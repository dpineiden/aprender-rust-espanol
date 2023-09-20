use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double, m)?)?;
    register_child_module(py, m)?;
    Ok(())
}


#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

use pyo3::prelude::*;


fn register_child_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "child_module")?;
    child_module.add_function(wrap_pyfunction!(func, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}

#[pyfunction]
fn func() -> String {
    "func".to_string()
}
