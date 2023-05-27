use super::RustBaseApiClass;
use crate::aug::word::RandomWordAugmentor;
use crate::aug::{Action, AugCountParams, BaseAugmentor};
use crate::doc::Doc;
use crate::model::word::RandomWordModel;
use pyo3::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::thread;

/// Api Class to perform RandomWord model augmentations on input
#[pyclass]
pub struct RustRandomWordApiClass {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: Action,
    /// Parameteres to calculate number of words that will be augmented
    aug_word_params: AugCountParams,
    /// RandomWordModel
    model: Arc<RandomWordModel>,
    /// Filter, Set of words that cannot be augmented
    stopwords: Arc<Option<HashSet<String>>>,
}

impl RustRandomWordApiClass {
    fn get_aug_word_params(&self) -> AugCountParams {
        self.aug_word_params.clone()
    }
}

#[pymethods]
impl RustRandomWordApiClass {
    #[new]
    #[pyo3(signature = (
        action, aug_min_word, aug_max_word, aug_p_word,
        stopwords, target_vec_words, target_map_words)
    )]
    fn new(
        action: String,
        aug_min_word: Option<usize>,
        aug_max_word: Option<usize>,
        aug_p_word: Option<f32>,
        stopwords: Option<HashSet<String>>,
        target_vec_words: Option<Vec<String>>,
        target_map_words: Option<HashMap<String, Vec<String>>>,
    ) -> Self {
        let model = match (target_vec_words, target_map_words) {
            (Some(target), _) => RandomWordModel::from_vec(target),
            (None, Some(target)) => RandomWordModel::from_map(target),
            (None, None) => RandomWordModel::empty_model(),
        };
        model.load_model();
        let action = match &action[..] {
            "substitute" => Action::Substitute,
            "delete" => Action::Delete,
            "swap" => Action::Swap,
            _ => Action::Substitute,
        };
        RustRandomWordApiClass {
            action: action,
            aug_word_params: AugCountParams::new(aug_min_word, aug_max_word, aug_p_word),
            model: Arc::new(model),
            stopwords: Arc::new(stopwords),
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

impl RustBaseApiClass<RandomWordAugmentor, RandomWordModel> for RustRandomWordApiClass {
    fn create_augmentor_instance(&self) -> RandomWordAugmentor {
        RandomWordAugmentor::new(
            self.action,
            self.get_aug_word_params(),
            Arc::clone(&self.model),
            Arc::clone(&self.stopwords),
        )
    }

    fn create_thread_handle_string(
        &self,
        input_string_ref: Arc<String>,
        n_on_thread: usize,
    ) -> thread::JoinHandle<Vec<String>> {
        let action_cloned = self.action.clone();
        let aug_params_word_cloned = self.get_aug_word_params();
        let arc_model_ref = Arc::clone(&self.model);
        let arc_stopword_ref = Arc::clone(&self.stopwords);

        let thread_handle = thread::spawn(move || {
            let mut rng: StdRng = SeedableRng::from_entropy();
            let mut thread_res = Vec::with_capacity(n_on_thread);
            let mut doc = Doc::from_arc(input_string_ref);
            let augmentor = RandomWordAugmentor::new(
                action_cloned,
                aug_params_word_cloned,
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
        thread_handle
    }

    fn create_thread_handle_list(
        &self,
        input_list_ref: Arc<Vec<String>>,
        left_idx: usize,
        right_idx: usize,
    ) -> thread::JoinHandle<Vec<String>> {
        let action_cloned = self.action.clone();
        let aug_params_word_cloned = self.get_aug_word_params();
        let arc_model_ref = Arc::clone(&self.model);
        let arc_stopword_ref = Arc::clone(&self.stopwords);

        let thread_handle = thread::spawn(move || {
            let mut rng: StdRng = SeedableRng::from_entropy();
            let mut thread_res = Vec::with_capacity(right_idx - left_idx);
            let augmentor = RandomWordAugmentor::new(
                action_cloned,
                aug_params_word_cloned,
                arc_model_ref,
                arc_stopword_ref,
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
