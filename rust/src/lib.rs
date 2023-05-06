pub mod model;
pub mod utils;

use model::{KeyboardModel, Mapping, Model, OcrModel};
use pyo3::prelude::*;

#[pyfunction]
fn create_ocr_from_mapping_and_get_predict(mapping: Mapping, feature: String) -> Vec<String> {
    let ocr_model = OcrModel::new_from_mapping(mapping);
    if let Some(vec) = ocr_model.predict(&feature) {
        return vec.clone();
    }
    Vec::new()
}

#[pyfunction]
fn create_ocr_and_get_stats(filepath: String) -> (usize, usize, Vec<(usize, usize)>) {
    let mut ocr_model = OcrModel::new(filepath);
    ocr_model.load_model();
    ocr_model.get_stats()
}

#[pyfunction]
fn create_ocr_and_get_predict(filepath: String, feature: String) -> Vec<String> {
    let mut ocr_model = OcrModel::new(filepath);
    ocr_model.load_model();
    let predict = ocr_model.predict(&feature);
    if let Some(vec) = predict {
        return vec.clone();
    }
    Vec::new()
}

#[pyfunction]
fn create_keyboard_and_get_predict(
    allow_special_char: bool,
    allow_numeric: bool,
    upper_case: bool,
    model_path: String,
    feature: String,
) -> Vec<String> {
    let mut key_model =
        KeyboardModel::new(allow_special_char, allow_numeric, upper_case, model_path);
    key_model.load_model();
    if let Some(vec) = key_model.predict(&feature) {
        return vec.clone();
    }
    Vec::new()
}

#[pyfunction]
fn create_keyboard_and_get_stats(
    allow_special_char: bool,
    allow_numeric: bool,
    upper_case: bool,
    model_path: String,
) -> (usize, usize, Vec<(usize, usize)>) {
    let mut key_model =
        KeyboardModel::new(allow_special_char, allow_numeric, upper_case, model_path);
    key_model.load_model();
    key_model.get_stats()
}

#[pymodule]
fn rust_fasttextaug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_ocr_and_get_predict, m)?)?;
    m.add_function(wrap_pyfunction!(
        create_ocr_from_mapping_and_get_predict,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(create_ocr_and_get_stats, m)?)?;
    m.add_function(wrap_pyfunction!(create_keyboard_and_get_predict, m)?)?;
    m.add_function(wrap_pyfunction!(create_keyboard_and_get_stats, m)?)?;
    Ok(())
}
