
use pyo3::prelude::*;

/// a simple function
#[pyfunction]
fn add(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

/// simple class
#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(value: i64) -> Self { Counter { value } }

    fn incr(&mut self, by: i64) {
        self.value += by;
    }

    fn value(&self) -> i64 {
        self.value
    }
}

/// module initializer
#[pymodule]
fn rust_lib(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}
