use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use std::collections::HashSet;

use super::AugCountParams;
use crate::doc::{Doc, TokenHandler};
use crate::model::BaseModel;

pub trait BaseAugmentor<T>
where
    T: BaseModel,
{
    fn get_action(&self) -> ();
    fn get_aug_params_word(&self) -> &AugCountParams;
    fn get_min_chars(&self) -> Option<usize> {
        None
    }
    fn get_model(&self) -> &T;
    fn get_stopwords(&self) -> Option<&HashSet<String>>;
    fn get_use_special_chars(&self) -> bool {
        false
    }

    fn get_filtered_word_tokens<'a>(&self, doc: &'a mut Doc) -> Vec<&'a mut TokenHandler> {
        let word_tokens = doc.get_word_tokens(self.get_use_special_chars());
        let mut filtered = Vec::with_capacity(word_tokens.len());
        let min_chars = self.get_min_chars();
        let stopwords = self.get_stopwords();
        for handler in word_tokens {
            let orig_token = handler.get_original().token();
            let token_len = handler.get_original().utf8_len();
            match (min_chars, stopwords) {
                (Some(min_c), Some(stop_set)) => {
                    if (!stop_set.contains(orig_token)) & (token_len >= min_c) {
                        filtered.push(handler)
                    }
                }
                (Some(min_c), None) => {
                    if token_len >= min_c {
                        filtered.push(handler)
                    }
                }
                (None, Some(stop_set)) => {
                    if !stop_set.contains(orig_token) {
                        filtered.push(handler)
                    }
                }
                (None, None) => filtered.push(handler),
            }
        }
        filtered
    }

    fn sample_word_tokens_to_aug<'a>(
        &self,
        doc: &'a mut Doc,
        rng: &mut StdRng,
    ) -> Vec<&'a mut TokenHandler> {
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

    struct MockModel {}
    impl BaseModel for MockModel {
        fn get_mapping(&self) -> Option<&crate::model::Mapping> {
            None
        }
    }
    struct MockAugmentor<'a> {
        aug_params: AugCountParams,
        min_chars: Option<usize>,
        model: &'a MockModel,
        stopwords: Option<&'a HashSet<String>>,
        use_special_chars: bool,
    }
    impl<'a> BaseAugmentor<MockModel> for MockAugmentor<'a> {
        fn get_action(&self) -> () {}
        fn get_min_chars(&self) -> Option<usize> {
            self.min_chars
        }
        fn get_model(&self) -> &MockModel {
            self.model
        }
        fn get_aug_params_word(&self) -> &AugCountParams {
            &self.aug_params
        }
        fn get_stopwords(&self) -> Option<&HashSet<String>> {
            self.stopwords
        }
        fn get_use_special_chars(&self) -> bool {
            self.use_special_chars
        }
    }

    #[test]
    fn test_filter_words_by_stopwords_with_spec() {
        let mut doc = Doc::new(&String::from("My example string!"));
        let stopwords = HashSet::from([String::from("example"), String::from("My")]);
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &MockModel {},
            stopwords: Some(&stopwords),
            use_special_chars: true,
        };
        let result = mock_object.get_filtered_word_tokens(&mut doc);
        let mut expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("string")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
        ];
        let expected: Vec<&mut TokenHandler> = expected_handlers.iter_mut().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_words_by_stopwords_without_spec() {
        let mut doc = Doc::new(&String::from("My example string!!!"));
        let stopwords = HashSet::from([String::from("string")]);
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &MockModel {},
            stopwords: Some(&stopwords),
            use_special_chars: false,
        };
        let result = mock_object.get_filtered_word_tokens(&mut doc);
        let mut expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("My")),
            TokenHandler::new(TokenType::WordToken, String::from("example")),
        ];
        let expected: Vec<&mut TokenHandler> = expected_handlers.iter_mut().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_words_by_minchar() {
        let mut doc = Doc::new(&String::from("Мой пример строки!!!"));
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, None),
            min_chars: Some(4),
            model: &MockModel {},
            stopwords: None,
            use_special_chars: true,
        };
        let result = mock_object.get_filtered_word_tokens(&mut doc);
        let mut expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("пример")),
            TokenHandler::new(TokenType::WordToken, String::from("строки")),
        ];
        let expected: Vec<&mut TokenHandler> = expected_handlers.iter_mut().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dont_filter_words_by_minchar() {
        let mut doc = Doc::new(&String::from("My example string!"));
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &MockModel {},
            stopwords: None,
            use_special_chars: true,
        };
        let result = mock_object.get_filtered_word_tokens(&mut doc);
        let mut expected_handlers = vec![
            TokenHandler::new(TokenType::WordToken, String::from("My")),
            TokenHandler::new(TokenType::WordToken, String::from("example")),
            TokenHandler::new(TokenType::WordToken, String::from("string")),
            TokenHandler::new(TokenType::SpecSymbolToken, String::from("!")),
        ];
        let expected: Vec<&mut TokenHandler> = expected_handlers.iter_mut().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sample_words_without_filtering_without_spec() {
        let mut doc = Doc::new(&String::from("My very useful example string!"));
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &MockModel {},
            stopwords: None,
            use_special_chars: false,
        };
        let mut rng = StdRng::from_entropy();
        let result = mock_object.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_sample_zero() {
        let mut doc = Doc::new(&String::from("My very useful example string!"));
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, Some(0.0)),
            min_chars: None,
            model: &MockModel {},
            stopwords: None,
            use_special_chars: false,
        };
        let mut rng = StdRng::from_entropy();
        let result = mock_object.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_sample_words_without_filtering_with_spec() {
        let mut doc = Doc::new(&String::from("My very useful example string!!!!!"));
        let mock_object = MockAugmentor {
            aug_params: AugCountParams::new(None, None, None),
            min_chars: None,
            model: &MockModel {},
            stopwords: None,
            use_special_chars: true,
        };
        let mut rng = StdRng::from_entropy();
        let result = mock_object.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 3);
    }
}
