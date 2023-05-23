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

#[pyclass]
pub struct RustOCRAugmentor {
    aug_char_params: AugCountParams,
    aug_word_params: AugCountParams,
    model: Arc<OcrModel>,
    stopwords: Arc<Option<HashSet<String>>>,
    min_char: Option<usize>,
}

impl RustOCRAugmentor {
    fn get_aug_char_params(&self) -> AugCountParams {
        self.aug_char_params.clone()
    }

    fn get_aug_word_params(&self) -> AugCountParams {
        self.aug_word_params.clone()
    }

    fn get_min_chars(&self) -> Option<usize> {
        self.min_char
    }
}

#[pymethods]
impl RustOCRAugmentor {
    #[new]
    #[pyo3(signature = (
        aug_min_char, aug_max_char, aug_p_char,
        aug_min_word, aug_max_word, aug_p_word,
        stopwords, min_char, dict_of_path)
    )]
    fn new(
        aug_min_char: Option<usize>,
        aug_max_char: Option<usize>,
        aug_p_char: Option<f32>,
        aug_min_word: Option<usize>,
        aug_max_word: Option<usize>,
        aug_p_word: Option<f32>,
        stopwords: Option<HashSet<String>>,
        min_char: Option<usize>,
        dict_of_path: String,
    ) -> Self {
        let mut model = OcrModel::new(dict_of_path);
        model.load_model();
        RustOCRAugmentor {
            aug_char_params: AugCountParams::new(aug_min_char, aug_max_char, aug_p_char),
            aug_word_params: AugCountParams::new(aug_min_word, aug_max_word, aug_p_word),
            model: Arc::new(model),
            stopwords: Arc::new(stopwords),
            min_char: min_char,
        }
    }

    fn augment_string_single_thread(&self, input_string: String, n: usize) -> Vec<String> {
        let mut rng: StdRng = SeedableRng::from_entropy();
        let mut result = Vec::with_capacity(n);
        let mut doc = Doc::new(&input_string);
        let augmentor = OcrAugmentor::new(
            self.get_aug_char_params(),
            self.get_aug_word_params(),
            self.get_min_chars(),
            Arc::clone(&self.model),
            Arc::clone(&self.stopwords),
        );

        for _ in 0..n {
            augmentor.augment(&mut doc, &mut rng);
            result.push(doc.get_augmented_string());
            doc.set_to_original();
        }
        result
    }

    fn augment_string_multi_thread(
        &self,
        input_string: String,
        n: usize,
        n_threads: usize,
    ) -> Vec<String> {
        let mut result = Vec::with_capacity(n);
        let arc_input_string = Arc::new(input_string);
        let mut thread_handles = Vec::with_capacity(n_threads);
        let n_on_threads = utils::split_n_to_chunks(n, n_threads);

        for idx in 0..n_threads {
            let n_on_thread = n_on_threads[idx];
            let aug_params_char_cloned = self.get_aug_char_params();
            let aug_params_word_cloned = self.get_aug_word_params();
            let min_chars_cloned = self.get_min_chars();
            let arc_input_string_ref = Arc::clone(&arc_input_string);
            let arc_model_ref = Arc::clone(&self.model);
            let arc_stopword_ref = Arc::clone(&self.stopwords);

            let thread_handle = thread::spawn(move || {
                let mut rng: StdRng = SeedableRng::from_entropy();
                let mut thread_res = Vec::with_capacity(n_on_thread);
                let mut doc = Doc::from_arc(arc_input_string_ref);
                let augmentor = OcrAugmentor::new(
                    aug_params_char_cloned,
                    aug_params_word_cloned,
                    min_chars_cloned,
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

    fn augment_list_single_thread(&self, input_list: Vec<String>) -> Vec<String> {
        let mut rng: StdRng = SeedableRng::from_entropy();
        let mut result = Vec::with_capacity(input_list.len());
        let augmentor = OcrAugmentor::new(
            self.get_aug_char_params(),
            self.get_aug_word_params(),
            self.get_min_chars(),
            Arc::clone(&self.model),
            Arc::clone(&self.stopwords),
        );
        for input_str in input_list {
            let mut doc = Doc::new(&input_str);
            augmentor.augment(&mut doc, &mut rng);
            result.push(doc.get_augmented_string());
        }
        result
    }

    fn augment_list_multi_thread(&self, input_list: Vec<String>, n_threads: usize) -> Vec<String> {
        let mut result = Vec::with_capacity(input_list.len());
        let arc_input_list = Arc::new(input_list);
        let chunk_indexes = utils::split_to_chunks_indexes(arc_input_list.len(), n_threads);
        let mut thread_handles = Vec::with_capacity(n_threads);

        for idx in 0..n_threads {
            let (left_idx, right_idx) = chunk_indexes[idx];
            if left_idx != right_idx {
                let aug_params_char_cloned = self.get_aug_char_params();
                let aug_params_word_cloned = self.get_aug_word_params();
                let min_chars_cloned = self.get_min_chars();
                let arc_model_ref = Arc::clone(&self.model);
                let arc_stopword_ref = Arc::clone(&self.stopwords);
                let arc_input_list_ref = Arc::clone(&arc_input_list);

                let thread_handle = thread::spawn(move || {
                    let mut rng: StdRng = SeedableRng::from_entropy();
                    let mut thread_res = Vec::with_capacity(right_idx - left_idx);
                    let augmentor = OcrAugmentor::new(
                        aug_params_char_cloned,
                        aug_params_word_cloned,
                        min_chars_cloned,
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
}
