use super::RustBaseApiClass;
use crate::aug::character::RandomCharAugmentor;
use crate::aug::{Action, AugCountParams, BaseAugmentor};
use crate::doc::Doc;
use crate::model::character::RandomCharModel;
use pyo3::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

/// Api Class to perform RandomChar model augmentations on input
#[pyclass]
pub struct RustRandomCharApiClass {
    /// Action to augmentation, set of values {'substitute', 'insert', 'swap', 'delete'}
    action: Action,
    /// Parameteres to calculate number of chars that will be augmented in single word
    aug_char_params: AugCountParams,
    /// Parameteres to calculate number of words that will be augmented
    aug_word_params: AugCountParams,
    /// RandomCharModel
    model: Arc<RandomCharModel>,
    /// Filter, Set of words that cannot be augmented
    stopwords: Arc<Option<HashSet<String>>>,
    /// Filter, do not augment word, if it's lenght less than this value
    min_char: Option<usize>,
    /// Choosen swap strategy
    swapmode: String,
}

impl RustRandomCharApiClass {
    fn get_aug_char_params(&self) -> AugCountParams {
        self.aug_char_params.clone()
    }

    fn get_aug_word_params(&self) -> AugCountParams {
        self.aug_word_params.clone()
    }

    fn get_min_chars(&self) -> Option<usize> {
        self.min_char
    }

    fn get_swapmode(&self) -> String {
        self.swapmode.clone()
    }
}

#[pymethods]
impl RustRandomCharApiClass {
    #[new]
    #[pyo3(signature = (
        action, aug_min_char, aug_max_char, aug_p_char,
        aug_min_word, aug_max_word, aug_p_word,
        include_upper_case, include_lower_case,
        include_numeric, include_special_char, lang,
        stopwords, min_char, swap_mode, spec_char,
        candidates)
    )]
    fn new(
        action: String,
        aug_min_char: Option<usize>,
        aug_max_char: Option<usize>,
        aug_p_char: Option<f32>,
        aug_min_word: Option<usize>,
        aug_max_word: Option<usize>,
        aug_p_word: Option<f32>,
        include_upper_case: bool,
        include_lower_case: bool,
        include_numeric: bool,
        include_special_char: bool,
        lang: String,
        stopwords: Option<HashSet<String>>,
        min_char: Option<usize>,
        swap_mode: String,
        spec_char: Option<String>,
        candidates: Option<Vec<String>>,
    ) -> Self {
        let mut model = match candidates {
            Some(values) => RandomCharModel::from_candidates(values),
            None => RandomCharModel::new(
                include_upper_case,
                include_lower_case,
                include_special_char,
                include_numeric,
                &lang,
                spec_char,
            ),
        };
        model.load_model();
        let action = match &action[..] {
            "insert" => Action::Insert,
            "substitute" => Action::Substitute,
            "delete" => Action::Delete,
            "swap" => Action::Swap,
            _ => Action::Substitute,
        };
        RustRandomCharApiClass {
            action,
            aug_char_params: AugCountParams::new(aug_min_char, aug_max_char, aug_p_char),
            aug_word_params: AugCountParams::new(aug_min_word, aug_max_word, aug_p_word),
            model: Arc::new(model),
            stopwords: Arc::new(stopwords),
            min_char: min_char,
            swapmode: swap_mode,
        }
    }

    fn augment_string_single_thread(&self, input_string: String, n: usize) -> Vec<String> {
        RustBaseApiClass::augment_string_single_thread(self, input_string, n)
    }

    fn augment_string_multi_thread(
        &self,
        input_string: String,
        n: usize,
        n_threads: usize,
    ) -> Vec<String> {
        RustBaseApiClass::augment_string_multi_thread(self, input_string, n, n_threads)
    }

    fn augment_list_single_thread(&self, input_list: Vec<String>) -> Vec<String> {
        RustBaseApiClass::augment_list_single_thread(self, input_list)
    }

    fn augment_list_multi_thread(&self, input_list: Vec<String>, n_threads: usize) -> Vec<String> {
        RustBaseApiClass::augment_list_multi_thread(self, input_list, n_threads)
    }
}

impl RustBaseApiClass<RandomCharAugmentor, RandomCharModel> for RustRandomCharApiClass {
    fn create_augmentor_instance(&self) -> RandomCharAugmentor {
        RandomCharAugmentor::new(
            self.action,
            self.get_aug_char_params(),
            self.get_aug_word_params(),
            self.get_min_chars(),
            Arc::clone(&self.model),
            Arc::clone(&self.stopwords),
            self.get_swapmode(),
        )
    }

    fn create_thread_handle_string(
        &self,
        input_string_ref: Arc<String>,
        n_on_thread: usize,
    ) -> thread::JoinHandle<Vec<String>> {
        let action_cloned = self.action.clone();
        let aug_params_char_cloned = self.get_aug_char_params();
        let aug_params_word_cloned = self.get_aug_word_params();
        let min_chars_cloned = self.get_min_chars();
        let arc_model_ref = Arc::clone(&self.model);
        let arc_stopword_ref = Arc::clone(&self.stopwords);
        let swapmode_cloned = self.get_swapmode();

        let thread_handle = thread::spawn(move || {
            let mut rng: StdRng = SeedableRng::from_entropy();
            let mut thread_res = Vec::with_capacity(n_on_thread);
            let mut doc = Doc::from_arc(input_string_ref);
            let augmentor = RandomCharAugmentor::new(
                action_cloned,
                aug_params_char_cloned,
                aug_params_word_cloned,
                min_chars_cloned,
                arc_model_ref,
                arc_stopword_ref,
                swapmode_cloned,
            );
            for _ in 0..n_on_thread {
                augmentor.augment(&mut doc, &mut rng);
                thread_res.push(doc.get_augmented_string());
                doc.set_to_original();
            }
            thread_res
        });
        thread_handle
    }

    fn create_thread_handle_list(
        &self,
        input_list_ref: Arc<Vec<String>>,
        left_idx: usize,
        right_idx: usize,
    ) -> thread::JoinHandle<Vec<String>> {
        let action_cloned = self.action.clone();
        let aug_params_char_cloned = self.get_aug_char_params();
        let aug_params_word_cloned = self.get_aug_word_params();
        let min_chars_cloned = self.get_min_chars();
        let arc_model_ref = Arc::clone(&self.model);
        let arc_stopword_ref = Arc::clone(&self.stopwords);
        let swapmode_cloned = self.get_swapmode();

        let thread_handle = thread::spawn(move || {
            let mut rng: StdRng = SeedableRng::from_entropy();
            let mut thread_res = Vec::with_capacity(right_idx - left_idx);
            let augmentor = RandomCharAugmentor::new(
                action_cloned,
                aug_params_char_cloned,
                aug_params_word_cloned,
                min_chars_cloned,
                arc_model_ref,
                arc_stopword_ref,
                swapmode_cloned,
            );
            // [left_idx..right_idx]
            for input in &input_list_ref.as_ref()[left_idx..right_idx] {
                let mut doc = Doc::new(input);
                augmentor.augment(&mut doc, &mut rng);
                thread_res.push(doc.get_augmented_string());
            }

            thread_res
        });
        thread_handle
    }
}
