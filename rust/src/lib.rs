pub mod api;
pub mod aug;
pub mod doc;
pub mod model;
pub mod utils;

use pyo3::prelude::*;

#[pymodule]
fn rust_fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<api::ocr::RustOCRAugmentor>()?;
    m.add_class::<api::keyboard::RustKeyboardAugmentor>()?;
    Ok(())
}
