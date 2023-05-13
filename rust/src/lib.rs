pub mod api;
pub mod aug;
pub mod doc;
pub mod model;
pub mod utils;

use pyo3::prelude::*;

#[pymodule]
fn rust_fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(api::augment_by_ocr, m)?)?;
    m.add_function(wrap_pyfunction!(api::augment_by_ocr_list, m)?)?;
    Ok(())
}
