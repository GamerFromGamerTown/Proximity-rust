use pyo3::{pymodule};

/// A Python module implemented in Rust.
#[pymodule]
mod pyoxide {
    use pyo3::{pyfunction, PyResult};

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
