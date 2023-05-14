use crate::aug::character::OcrAugmentor;
use crate::aug::{AugCountParams, BaseAugmentor};
use crate::doc::Doc;
use crate::model::character::OcrModel;
use crate::utils;
use pyo3::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

#[pyfunction]
#[pyo3(signature = (
    input_string, n, aug_min_char, aug_max_char,
    aug_p_char, aug_min_word, aug_max_word, aug_p_word,
    min_chars, model_path, stopwords)
)]
pub fn augment_by_ocr_single_thread(
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
    let mut model = OcrModel::new(model_path);
    model.load_model();
    let arc_model = Arc::new(model);
    let arc_stopwords = Arc::new(stopwords);
    let mut rng: StdRng = SeedableRng::from_entropy();

    let mut result = Vec::with_capacity(n);
    let mut doc = Doc::new(&input_string);
    let augmentor = OcrAugmentor::new(
        aug_params_char,
        aug_params_word,
        min_chars,
        arc_model,
        arc_stopwords,
    );

    for _ in 0..n {
        augmentor.augment(&mut doc, &mut rng);
        result.push(doc.get_augmented_string());
        doc.set_to_original();
    }
    result
}

#[pyfunction]
#[pyo3(signature = (
    input_string, n, num_threads, aug_min_char, aug_max_char,
    aug_p_char, aug_min_word, aug_max_word, aug_p_word,
    min_chars, model_path, stopwords)
)]
pub fn augment_by_ocr_multi_thread(
    input_string: String,
    n: usize,
    mut num_threads: usize,
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
    let mut model = OcrModel::new(model_path);
    model.load_model();
    let arc_input_string = Arc::new(input_string);
    let arc_model = Arc::new(model);
    let arc_stopwords = Arc::new(stopwords);
    if num_threads > 8 {
        num_threads = 8
    }
    if n < num_threads {
        num_threads = 1
    }

    let mut result = Vec::with_capacity(n);
    let mut thread_handles = Vec::with_capacity(num_threads);
    let n_on_threads = utils::split_n_to_chunks(n, num_threads);

    for idx in 0..num_threads {
        let n_on_thread = n_on_threads[idx];
        let aug_params_char_cloned = aug_params_char.clone();
        let aug_params_word_cloned = aug_params_word.clone();
        let arc_input_string_ref = Arc::clone(&arc_input_string);
        let arc_model_ref = Arc::clone(&arc_model);
        let arc_stopword_ref = Arc::clone(&arc_stopwords);

        let thread_handle = thread::spawn(move || {
            let mut rng: StdRng = SeedableRng::from_entropy();
            let mut thread_res = Vec::with_capacity(n_on_thread);
            let mut doc = Doc::from_arc(arc_input_string_ref);
            let augmentor = OcrAugmentor::new(
                aug_params_char_cloned,
                aug_params_word_cloned,
                min_chars,
                arc_model_ref,
                arc_stopword_ref,
            );
            for _ in 0..n_on_thread {
                augmentor.augment(&mut doc, &mut rng);
                thread_res.push(doc.get_augmented_string());
                doc.set_to_original();
            }
            thread_res
        });
        thread_handles.push(thread_handle);
    }

    for thread_handle in thread_handles {
        let thread_res = thread_handle.join().unwrap();
        result.extend(thread_res)
    }
    result
}

#[pyfunction]
#[pyo3(signature = (
    input_list, aug_min_char, aug_max_char,
    aug_p_char, aug_min_word, aug_max_word, aug_p_word,
    min_chars, model_path, stopwords)
)]
pub fn augment_by_ocr_list_single_thread(
    input_list: Vec<String>,
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
    let arc_model = Arc::new(model);
    let arc_stopwords = Arc::new(stopwords);

    let augmentor = OcrAugmentor::new(
        aug_params_char,
        aug_params_word,
        min_chars,
        Arc::clone(&arc_model),
        Arc::clone(&arc_stopwords),
    );
    let mut result = Vec::with_capacity(input_list.len());
    for input in input_list {
        let mut doc = Doc::new(&input);
        augmentor.augment(&mut doc, &mut rng);
        result.push(doc.get_augmented_string());
    }
    result
}

#[pyfunction]
#[pyo3(signature = (
    input_list, num_threads, aug_min_char, aug_max_char,
    aug_p_char, aug_min_word, aug_max_word, aug_p_word,
    min_chars, model_path, stopwords)
)]
pub fn augment_by_ocr_list_multi_thread(
    input_list: Vec<String>,
    num_threads: usize,
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
    let mut model = OcrModel::new(model_path);
    model.load_model();
    let arc_input_list = Arc::new(input_list);
    let arc_model = Arc::new(model);
    let arc_stopwords = Arc::new(stopwords);

    let chunk_indexes = utils::split_to_chunks_indexes(arc_input_list.len(), num_threads);
    let mut result = Vec::with_capacity(arc_input_list.len());
    let mut thread_handles = Vec::with_capacity(num_threads);

    for idx in 0..num_threads {
        let (left_idx, right_idx) = chunk_indexes[idx];
        if left_idx != right_idx {
            let aug_params_char_cloned = aug_params_char.clone();
            let aug_params_word_cloned = aug_params_word.clone();
            let arc_model_ref = Arc::clone(&arc_model);
            let arc_stopword_ref = Arc::clone(&arc_stopwords);
            let arc_input_list_ref = Arc::clone(&arc_input_list);

            let thread_handle = thread::spawn(move || {
                let mut rng: StdRng = SeedableRng::from_entropy();
                let mut thread_res = Vec::with_capacity(right_idx - left_idx);
                let augmentor = OcrAugmentor::new(
                    aug_params_char_cloned,
                    aug_params_word_cloned,
                    min_chars,
                    arc_model_ref,
                    arc_stopword_ref,
                );
                // [left_idx..right_idx]
                for input in &arc_input_list_ref.as_ref()[left_idx..right_idx] {
                    let mut doc = Doc::new(input);
                    augmentor.augment(&mut doc, &mut rng);
                    thread_res.push(doc.get_augmented_string());
                }

                thread_res
            });

            thread_handles.push(thread_handle);
        }
    }

    for thread_handle in thread_handles {
        let thread_res = thread_handle.join().unwrap();
        result.extend(thread_res)
    }
    result
}
