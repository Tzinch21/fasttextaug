pub mod api;
pub mod aug;
pub mod doc;
pub mod model;
pub mod utils;

use pyo3::prelude::*;

#[pymodule]
fn rust_fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<api::RustOCRApiClass>()?;
    m.add_class::<api::RustKeyboardApiClass>()?;
    m.add_class::<api::RustRandomCharApiClass>()?;
    m.add_class::<api::RustRandomWordApiClass>()?;
    Ok(())
}
