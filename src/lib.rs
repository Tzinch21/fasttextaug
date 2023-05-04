pub mod model;
pub mod utils;

use std::path::Path;
use model::Model;
use pyo3::prelude::*;

#[pyfunction]
fn get_predict_from_model(feature: String) -> String {
    let file_path = Path::new("res/ru.json");
    let ocr_model = model::OcrModel::from_json(&file_path);
    let predict = ocr_model.predict(&feature);
    if let Some(vec) = predict {
        let result = vec.iter().next().unwrap();
        return result.clone()
    }
    String::from("Not found")
}

#[pymodule]
fn fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_predict_from_model, m)?)?;
    Ok(())
}