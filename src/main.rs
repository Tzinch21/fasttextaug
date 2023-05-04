use std::path::Path;

use fasttextaug::model::{Model, OcrModel};

fn main() {
    let file_path = Path::new("res/ru.json");
    let ocr_model = OcrModel::from_json(&file_path);
    println!("{:?}", ocr_model.model);
    println!("{:?}", ocr_model.model.len());
    println!("{:?}", ocr_model.model.iter().map(|(_, x)| x.len()).max());
}
