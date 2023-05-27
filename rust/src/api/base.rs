use crate::aug::BaseAugmentor;
use crate::doc::Doc;
use crate::model::BaseModel;
use crate::utils;
use rand::{rngs::StdRng, SeedableRng};
use std::sync::Arc;
use std::thread::JoinHandle;

/// Base RustApiClass functionality
pub trait RustBaseApiClass<A, M>
where
    A: BaseAugmentor<M>,
    M: BaseModel,
{
    /// create specific augmentor instanse
    fn create_augmentor_instance(&self) -> A;
    fn create_thread_handle_string(
        &self,
        input_string_ref: Arc<String>,
        n_on_thread: usize,
    ) -> JoinHandle<Vec<String>>;
    fn create_thread_handle_list(
        &self,
        input_list_ref: Arc<Vec<String>>,
        left_idx: usize,
        right_idx: usize,
    ) -> JoinHandle<Vec<String>>;

    /// Augment `input_string` `n` times in single thread mode
    fn augment_string_single_thread(&self, input_string: String, n: usize) -> Vec<String> {
        let mut rng: StdRng = SeedableRng::from_entropy();
        let mut result = Vec::with_capacity(n);
        let mut doc = Doc::new(&input_string);
        let augmentor = self.create_augmentor_instance();
        for _ in 0..n {
            augmentor.augment(&mut doc, &mut rng);
            result.push(doc.get_augmented_string());
            doc.set_to_original();
        }
        result
    }

    /// Augment `input_string` `n` times in multi thread mode (`n_threads`)
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
            let arc_input_string_ref = Arc::clone(&arc_input_string);
            let handle = self.create_thread_handle_string(arc_input_string_ref, n_on_thread);
            thread_handles.push(handle);
        }
        for thread_handle in thread_handles {
            let thread_res = thread_handle.join().unwrap();
            result.extend(thread_res)
        }
        result
    }

    /// Augment list of values in single thread mode
    fn augment_list_single_thread(&self, input_list: Vec<String>) -> Vec<String> {
        let mut rng: StdRng = SeedableRng::from_entropy();
        let mut result = Vec::with_capacity(input_list.len());
        let augmentor = self.create_augmentor_instance();
        for input_str in input_list {
            let mut doc = Doc::new(&input_str);
            augmentor.augment(&mut doc, &mut rng);
            result.push(doc.get_augmented_string());
        }
        result
    }

    /// Augment list of values in multi thread mode (`n_threads`)
    fn augment_list_multi_thread(&self, input_list: Vec<String>, n_threads: usize) -> Vec<String> {
        let mut result = Vec::with_capacity(input_list.len());
        let arc_input_list = Arc::new(input_list);
        let chunk_indexes = utils::split_to_chunks_indexes(arc_input_list.len(), n_threads);
        let mut thread_handles = Vec::with_capacity(n_threads);

        for idx in 0..n_threads {
            let (left_idx, right_idx) = chunk_indexes[idx];
            if left_idx != right_idx {
                let arc_input_list_ref = Arc::clone(&arc_input_list);
                let handle =
                    self.create_thread_handle_list(arc_input_list_ref, left_idx, right_idx);
                thread_handles.push(handle);
            }
        }

        for thread_handle in thread_handles {
            let thread_res = thread_handle.join().unwrap();
            result.extend(thread_res)
        }
        result
    }
}
