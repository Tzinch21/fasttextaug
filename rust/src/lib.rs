pub mod model;
pub mod utils;

use model::Model;
use pyo3::prelude::*;
use std::path::Path;

#[pyfunction]
fn get_predict_from_ocr_model(feature: String, filepath: String) -> String {
    let file_path = Path::new(&filepath);
    let ocr_model = model::OcrModel::from_json(&file_path);
    let predict = ocr_model.predict(&feature);
    if let Some(vec) = predict {
        let result = vec.iter().next().unwrap();
        return result.clone();
    }
    String::from("Not found")
}

#[pymodule]
fn rust_fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_predict_from_ocr_model, m)?)?;
    Ok(())
}
