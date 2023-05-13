use crate::aug::character::OcrAugmentor;
use crate::aug::{AugCountParams, BaseAugmentor};
use crate::doc::Doc;
use crate::model::character::OcrModel;
use pyo3::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashSet;

#[pyfunction]
#[pyo3(signature = (
    input_string, n, aug_min_char, aug_max_char,
    aug_p_char, aug_min_word, aug_max_word, aug_p_word,
    min_chars, model_path, stopwords)
)]
pub fn augment_by_ocr(
    input_string: String,
    n: usize,
    aug_min_char: Option<usize>,
    aug_max_char: Option<usize>,
    aug_p_char: Option<f32>,
    aug_min_word: Option<usize>,
    aug_max_word: Option<usize>,
    aug_p_word: Option<f32>,
    min_chars: Option<usize>,
    model_path: String,
    stopwords: Option<HashSet<String>>,
) -> Vec<String> {
    let aug_params_char = AugCountParams::new(aug_min_char, aug_max_char, aug_p_char);
    let aug_params_word = AugCountParams::new(aug_min_word, aug_max_word, aug_p_word);
    let mut rng: StdRng = SeedableRng::from_entropy();
    let mut model = OcrModel::new(model_path);
    model.load_model();
    let mut doc = Doc::new(input_string);
    let augmentor = OcrAugmentor::new(
        aug_params_char,
        aug_params_word,
        min_chars,
        &model,
        stopwords.as_ref(),
    );

    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
        augmentor.augment(&mut doc, &mut rng);
        result.push(doc.get_augmented_string());
    }
    result
}
