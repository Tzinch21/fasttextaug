pub mod api;
pub mod aug;
pub mod doc;
pub mod model;
pub mod utils;

use pyo3::prelude::*;

#[pymodule]
fn rust_fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(api::ocr::augment_by_ocr_single_thread, m)?)?;
    m.add_function(wrap_pyfunction!(api::ocr::augment_by_ocr_multi_thread, m)?)?;
    m.add_function(wrap_pyfunction!(
        api::ocr::augment_by_ocr_list_single_thread,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        api::ocr::augment_by_ocr_list_multi_thread,
        m
    )?)?;
    Ok(())
}
