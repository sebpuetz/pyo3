use pyo3::prelude::*;
use pyo3::types::{PyLong, PyString};
use pyo3::{wrap_pyfunction, FromPyObject};

// #[pyfunction]
// pub fn foo<'a>(inp: Union<'a>) {
//     match inp {
//         Union::Str(s) => println!("{}", s.to_string_lossy()),
//         Union::Int(i) => println!("{}", i.repr().unwrap()),
//         Union::StringList(s_list) => println!("{:?}", s_list),
//     }
// }

#[derive(FromPyObject)]
pub enum Union<'a> {
    #[rename="str"]
    Str(&'a PyString),
    Int(&'a PyLong),
    StringList(Vec<String>),
}

#[derive(FromPyObject)]
pub struct A<'a> {
    #[rename="str"]
    s: &'a PyString,
}
//
// #[pymodule]
// pub fn union(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(foo))?;
//     Ok(())
// }
