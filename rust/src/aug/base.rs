use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use std::collections::HashSet;

use super::AugCountParams;
use crate::doc::{Doc, TokenHandler};
use crate::model::BaseModel;

/// Actions enum - what we can do with a data to create augmentions
#[derive(Clone, Copy)]
pub enum Action {
    Insert,
    Substitute,
    Delete,
    Swap,
}

/// Base augmentors functionality
pub trait BaseAugmentor<T>
where
    T: BaseModel,
{
    fn augment(&self, doc: &mut Doc, rng: &mut StdRng) -> ();
    fn get_action(&self) -> Action;
    fn get_aug_params_word(&self) -> &AugCountParams;
    fn get_flag_use_model_in_sampling_words(&self) -> bool {
        false
    }
    fn get_min_chars(&self) -> Option<usize> {
        None
    }
    fn get_model(&self) -> &T;
    fn get_stopwords(&self) -> Option<&HashSet<String>>;
    fn get_use_special_chars(&self) -> bool {
        false
    }

    /// Apply stopwords, min_char and model filtration to the input data
    /// Returns filtered vector of tuples (original_index, element)
    fn get_filtered_word_tokens<'a>(&self, doc: &'a mut Doc) -> Vec<(usize, &'a mut TokenHandler)> {
        let word_tokens = doc.get_word_tokens_with_indexes(self.get_use_special_chars());
        let model = self.get_model();
        let min_chars = self.get_min_chars();
        let stopwords = self.get_stopwords();
        let use_model_to_filtration = self.get_flag_use_model_in_sampling_words();

        let mut filtered = Vec::with_capacity(word_tokens.len());

        for (idx, handler) in word_tokens {
            let orig_token = handler.get_original().token();
            if use_model_to_filtration {
                if !model.key_exists(orig_token) {
                    continue;
                }
            }
            if let Some(min_char_len) = min_chars {
                let token_len = handler.get_original().utf8_len();
                if token_len < min_char_len {
                    continue;
                }
            }
            if let Some(stopw) = stopwords {
                if stopw.contains(orig_token) {
                    continue;
                }
            }
            filtered.push((idx, handler));
        }
        filtered
    }

    /// Sample words to augmentation after filtration
    fn sample_word_tokens_to_aug<'a>(
        &self,
        doc: &'a mut Doc,
        rng: &mut StdRng,
    ) -> Vec<(usize, &'a mut TokenHandler)> {
        let origin_word_count = doc.get_word_tokens_count(self.get_use_special_chars());
        let filtered_word_tokens = self.get_filtered_word_tokens(doc);
        let aug_cnt = self
            .get_aug_params_word()
            .calculate_aug_cnt(origin_word_count);

        if filtered_word_tokens.len() == 0 {
            return Vec::new();
        } else if aug_cnt >= filtered_word_tokens.len() {
            return filtered_word_tokens;
        }
        let sampled = filtered_word_tokens
            .into_iter()
            .choose_multiple(rng, aug_cnt);
        sampled
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use crate::doc::*;

    struct MockModel {
        data: Vec<String>,
    }

    impl MockModel {
        fn new(data: Vec<String>) -> Self {
            Self { data }
        }
    }

    impl BaseModel for MockModel {
        fn get_mapping(&self) -> Option<&crate::model::Mapping> {
            None
        }
        fn key_exists(&self, data: &str) -> bool {
            self.data.contains(&String::from(data))
        }
    }

    struct MockAugmentor<'a> {
        aug_params_word: AugCountParams,
        min_chars: Option<usize>,
        model: &'a MockModel,
        stopwords: Option<&'a HashSet<String>>,
        use_special_chars: bool,
        use_model_to_filtration: bool,
    }

    impl<'a> BaseAugmentor<MockModel> for MockAugmentor<'a> {
        fn augment(&self, _: &mut Doc, _: &mut StdRng) -> () {}
        fn get_action(&self) -> Action {
            Action::Substitute
        }
        fn get_aug_params_word(&self) -> &AugCountParams {
            &self.aug_params_word
        }
        fn get_flag_use_model_in_sampling_words(&self) -> bool {
            self.use_model_to_filtration
        }
        fn get_min_chars(&self) -> Option<usize> {
            self.min_chars
        }
        fn get_model(&self) -> &MockModel {
            self.model
        }
        fn get_stopwords(&self) -> Option<&HashSet<String>> {
            self.stopwords
        }
        fn get_use_special_chars(&self) -> bool {
            self.use_special_chars
        }
    }

    #[test]
    fn test_filter_with_special_char() {
        // Test only special char paramenter set on true, other on default
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &model,
            stopwords: None,
            use_special_chars: true,
            use_model_to_filtration: false,
        };
        let result = mock_aug.get_filtered_word_tokens(&mut doc);
        let indexes: Vec<usize> = result.iter().map(|x| x.0).collect();
        assert_eq!(indexes, vec![0, 2, 3, 5, 7, 8]);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_filter_without_special_char() {
        // Test only special char paramenter on false, other on default
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &model,
            stopwords: None,
            use_special_chars: false,
            use_model_to_filtration: false,
        };
        let result = mock_aug.get_filtered_word_tokens(&mut doc);
        let indexes: Vec<usize> = result.iter().map(|x| x.0).collect();
        assert_eq!(indexes, vec![0, 3, 7]);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_filter_by_min_char_and_special_char() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, None),
            min_chars: Some(3),
            model: &model,
            stopwords: None,
            use_special_chars: true,
            use_model_to_filtration: false,
        };
        let result = mock_aug.get_filtered_word_tokens(&mut doc);
        let indexes: Vec<usize> = result.iter().map(|x| x.0).collect();
        assert_eq!(indexes, vec![3, 7]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_filter_by_min_char_and_stopwords() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let stopwords = HashSet::from([String::from("example")]);
        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, None),
            min_chars: Some(2),
            model: &model,
            stopwords: Some(&stopwords),
            use_special_chars: false,
            use_model_to_filtration: false,
        };
        let result = mock_aug.get_filtered_word_tokens(&mut doc);
        let indexes: Vec<usize> = result.iter().map(|x| x.0).collect();
        assert_eq!(indexes, vec![0, 7]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_filter_by_min_char_and_stopwords_and_empty_model() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let stopwords = HashSet::from([String::from("example")]);
        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, None),
            min_chars: Some(2),
            model: &model,
            stopwords: Some(&stopwords),
            use_special_chars: false,
            use_model_to_filtration: true,
        };
        let result = mock_aug.get_filtered_word_tokens(&mut doc);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_filter_by_min_char_and_stopwords_and_non_empty_model() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![String::from("string")]);
        let stopwords = HashSet::from([String::from("example")]);
        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, None),
            min_chars: Some(2),
            model: &model,
            stopwords: Some(&stopwords),
            use_special_chars: false,
            use_model_to_filtration: true,
        };
        let result = mock_aug.get_filtered_word_tokens(&mut doc);
        let indexes: Vec<usize> = result.iter().map(|x| x.0).collect();
        assert_eq!(indexes, vec![7]);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_sample_some_data() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let stopwords = HashSet::from([String::from("example")]);
        let mut rng: StdRng = SeedableRng::from_entropy();

        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, Some(0.1)),
            min_chars: Some(2),
            model: &model,
            stopwords: Some(&stopwords),
            use_special_chars: false,
            use_model_to_filtration: false,
        };
        let result = mock_aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 1);
        assert!((result[0].0 == 0) | (result[0].0 == 7));
    }

    #[test]
    fn test_sample_full_data() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let stopwords = HashSet::from([String::from("example")]);
        let mut rng: StdRng = SeedableRng::from_entropy();

        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, Some(1.0)),
            min_chars: Some(2),
            model: &model,
            stopwords: Some(&stopwords),
            use_special_chars: false,
            use_model_to_filtration: false,
        };
        let result = mock_aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 2);
        assert!((result[0].0 == 0) & (result[1].0 == 7));
    }

    #[test]
    fn test_sample_zero_data() {
        let mut doc = Doc::new(&String::from("My !example ! string!"));
        let model = MockModel::new(vec![]);
        let stopwords = HashSet::from([String::from("example")]);
        let mut rng: StdRng = SeedableRng::from_entropy();

        let mock_aug = MockAugmentor {
            aug_params_word: AugCountParams::new(None, None, Some(0.0)),
            min_chars: Some(2),
            model: &model,
            stopwords: Some(&stopwords),
            use_special_chars: false,
            use_model_to_filtration: false,
        };
        let result = mock_aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 0);
    }
}
