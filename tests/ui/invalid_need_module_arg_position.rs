use pyo3::prelude::*;

#[pymodule]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "with_module", need_module)]
    fn fail(string: &str, module: &PyModule) -> PyResult<&str> {
        module.name()
    }
    Ok(())
}

fn main(){}