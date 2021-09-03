use pyo3::prelude::*;

#[pyfunction]
fn calculate(n_terms: usize) -> PyResult<f64> {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut operation = 1.0;
    let mut pi = 0.0;
    for _ in 0..n_terms {
        pi += operation * (numerator / denominator);
        denominator += 2.0;
        operation *= -1.0;
    }
    Ok(pi)
}

#[pymodule]
fn calculate_pi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate, m)?)?;
    Ok(())
}
