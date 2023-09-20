use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyDict,PyTuple};
use primos::primeros_primos;
use std::num::ParseIntError;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn double(x:usize)-> usize { 
	x * 2 
}


#[pyfunction]
#[pyo3(signature=(**kwds))]
fn num_kwds(kwds: Option<&PyDict>)->usize {
	kwds.map_or(0, |dict| dict.len())
}


#[pyfunction]
#[pyo3(signature=(*py_args))]
fn es_par(py_args: &PyTuple)->bool {
	py_args.len() % 2  == 0
}

#[pyfunction]
fn pyprimos(numero: u64)->Vec<u64> {
	primeros_primos(numero)
}

#[pyfunction]
fn check_positive(x: i32)->PyResult<()> {
	if x <0 {
		Err(PyValueError::new_err("x is negative"))
	} else {
		Ok(())
	}
}

#[pyfunction]
fn parse_int(x: &str)->Result<usize, ParseIntError> {
	x.parse()
}

#[pyclass]
struct Integer {
	inner: i32
}


#[pymethods]
impl Integer {
	#[new]
	fn new(value:i32)-> Self {
		Integer {inner: value}
	}
}

#[pyclass]
struct Number(i32);

#[pymethods]
impl Number {
	#[new]
	fn py_new(value:i32)->PyResult<Self> {
		if value==0{
			Err(PyValueError::new_err("cannot be zero"))
		} else {
			Ok(Number(value))
		}
	}
}

// #[pyclass]
// enum HttpResponse {
// 	Ok = 200,
// 	NotFound = 404,
// 	Teapo = 418
// }

// #[pyclass]
// enum MyEnum {
// 	Variant,
// 	OtherVariant = 30
// }


/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double, m)?)?;
	m.add_function(wrap_pyfunction!(num_kwds, m)?)?;
	m.add_function(wrap_pyfunction!(es_par, m)?)?;
	m.add_function(wrap_pyfunction!(pyprimos, m)?)?;
	m.add_function(wrap_pyfunction!(check_positive, m)?)?;
	m.add_function(wrap_pyfunction!(parse_int, m)?)?;
	m.add_class::<Number>()?;
	m.add_class::<Integer>()?;
    Ok(())
}
