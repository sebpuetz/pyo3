use pyo3::prelude::*;
use pyo3::types::{PyLong, PyString};
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn foo<'a>(inp: Union<'a>) {
    match inp {
        Union::Str(s) => println!("{}", s.to_string_lossy()),
        Union::Int(i) => println!("{}", i.repr().unwrap()),
        Union::StringList(s_list) => println!("{:?}", s_list),
    }
}

#[union]
pub enum Union<'a> {
    Str(&'a PyString),
    Int(&'a PyLong),
    StringList(Vec<String>),
}

#[pymodule]
pub fn union(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(foo))?;
    Ok(())
}
