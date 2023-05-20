use super::super::{AugCountParams, BaseAugmentor};
use crate::doc::{Doc, Token, TokenType};
use crate::model::character::CharacterModel;
use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use std::collections::HashSet;

pub trait CharacterAugmentor<T>: BaseAugmentor<T>
where
    T: CharacterModel,
{
    fn get_aug_params_char(&self) -> &AugCountParams;

    fn sample_chars_to_aug(&self, token: &Token, rng: &mut StdRng) -> HashSet<usize> {
        let mut char_indexes: Vec<usize> = Vec::with_capacity(token.utf8_len());
        let aug_cnt = self
            .get_aug_params_char()
            .calculate_aug_cnt(token.utf8_len());
        for (idx, char) in token.token().chars().enumerate() {
            let key = char.to_string();
            let contains = self.get_model().key_exists(&key);
            if contains {
                char_indexes.push(idx);
            }
        }
        let sampled: Vec<usize>;
        if char_indexes.len() == 0 {
            sampled = Vec::new();
        } else if aug_cnt >= char_indexes.len() {
            sampled = char_indexes;
        } else {
            sampled = char_indexes.into_iter().choose_multiple(rng, aug_cnt);
        }
        HashSet::from_iter(sampled)
    }

    fn predict_char(
        &self,
        idx: usize,
        ch: char,
        char_idxs: &HashSet<usize>,
        rng: &mut StdRng,
    ) -> String {
        let ch_str = ch.to_string();
        if char_idxs.contains(&idx) {
            let predict = self.get_model().predict(&ch_str);
            if let Some(predicted) = predict {
                let replacer = predicted.into_iter().choose(rng);
                if let Some(value) = replacer {
                    return value.clone();
                }
            }
        }
        ch_str
    }

    fn substitute(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        let aug_tokens = self.sample_word_tokens_to_aug(doc, rng);
        let mut change_seq = 0;
        for a_token in aug_tokens {
            let original_token = a_token.get_original();
            let aug_chars_indexes = self.sample_chars_to_aug(original_token, rng);
            if aug_chars_indexes.len() == 0 {
                continue;
            }
            let mut result = String::with_capacity(original_token.byte_len() * 2);
            original_token
                .token()
                .chars()
                .enumerate()
                .map(|(idx, ch)| self.predict_char(idx, ch, &aug_chars_indexes, rng))
                .for_each(|x| result.push_str(&x));
            result.shrink_to_fit();
            a_token.change(TokenType::WordToken, result);
            change_seq += 1;
        }
        doc.set_change_count(change_seq);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BaseModel, Mapping};
    use crate::utils;
    use rand::SeedableRng;
    use std::collections::{HashMap, HashSet};
    struct MockModel {
        mapping: Option<Mapping>,
    }
    impl MockModel {
        fn new() -> Self {
            let model = HashMap::from([
                (
                    String::from("А"),
                    vec![String::from("Х"), String::from("Ш")],
                ),
                (
                    String::from("а"),
                    vec![String::from("О"), String::from("0")],
                ),
                (
                    String::from("г"),
                    vec![String::from("Х"), String::from("Ш")],
                ),
                (
                    String::from("с"),
                    vec![String::from("О"), String::from("0")],
                ),
                (
                    String::from("v"),
                    vec![String::from("s"), String::from("7")],
                ),
                (
                    String::from("Q"),
                    vec![String::from("O"), String::from("f")],
                ),
            ]);
            MockModel {
                mapping: Some(model),
            }
        }
    }
    impl BaseModel for MockModel {
        fn get_mapping(&self) -> Option<&Mapping> {
            self.mapping.as_ref()
        }
    }
    impl CharacterModel for MockModel {}

    struct MockAugmentor {
        aug_params_char: AugCountParams,
        aug_params_word: AugCountParams,
        model: MockModel,
    }
    impl BaseAugmentor<MockModel> for MockAugmentor {
        fn get_action(&self) -> () {}
        fn get_aug_params_word(&self) -> &AugCountParams {
            &self.aug_params_word
        }

        fn get_model(&self) -> &MockModel {
            &self.model
        }
        fn get_stopwords(&self) -> Option<&HashSet<String>> {
            None
        }
    }
    impl CharacterAugmentor<MockModel> for MockAugmentor {
        fn get_aug_params_char(&self) -> &AugCountParams {
            &self.aug_params_char
        }
    }

    #[test]
    fn sample_chars_to_aug() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(1), Some(5), Some(0.5)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("Qvqv"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn sample_chars_to_aug_cyrillic() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(3), Some(7), Some(0.4)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("агсагсагс"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        // every char presented at model mapping (9), but aug_params get 4
        assert_eq!(res.len(), 4);
    }

    #[test]
    fn sample_chars_to_aug_get_all_possible() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(1), Some(10), Some(0.8)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("vavava"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        // not every char presented at model mapping (only 3), but aug_params gets 5.
        // We still take all 3 possible variants
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn sample_chars_to_aug_get_all_possible_cyrillic() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(3), Some(7), Some(0.8)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("Авиастроение"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        assert_eq!(res.len(), 3)
    }

    #[test]
    fn sample_chars_to_aug_get_zero_by_params() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, Some(0.0)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("vavava"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn sample_chars_to_aug_get_zero_by_mapping() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(1), Some(3), Some(0.3)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("none"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn sample_chars_to_aug_get_zero_by_params_cyrillic() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, Some(0.0)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("агс"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn sample_chars_to_aug_get_zero_by_mapping_cyrillic() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(1), Some(3), Some(0.3)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let mut rng: StdRng = SeedableRng::from_entropy();
        let token = Token::new(TokenType::WordToken, String::from("ноль"));
        let res = mock_aug.sample_chars_to_aug(&token, &mut rng);
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn test_predict_char_not_idx_not_model() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let char_idxs: HashSet<usize> = HashSet::from([2, 3]);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = mock_aug.predict_char(0, 'м', &char_idxs, &mut rng);
        assert_eq!(result, String::from("м"));
    }

    #[test]
    fn test_predict_char_not_idx_in_model() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let char_idxs: HashSet<usize> = HashSet::from([2, 3]);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = mock_aug.predict_char(0, 'г', &char_idxs, &mut rng);
        assert_eq!(result, String::from("г"));
    }

    #[test]
    fn test_predict_char_in_idx_not_model() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let char_idxs: HashSet<usize> = HashSet::from([2, 3]);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = mock_aug.predict_char(3, 'к', &char_idxs, &mut rng);
        assert_eq!(result, String::from("к"));
    }

    #[test]
    fn test_predict_char_in_idx_in_model() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let char_idxs: HashSet<usize> = HashSet::from([2, 3]);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = mock_aug.predict_char(3, 'А', &char_idxs, &mut rng);
        assert!((result == String::from("Х")) | (result == String::from("Ш")));
    }

    #[test]
    fn test_substitute_word_non_sampled() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(None, None, Some(0.0)),
            model: model,
        };
        let input_string = String::from("Пример строки для аугментации");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        mock_aug.substitute(&mut doc, &mut rng);
        assert_eq!(doc.get_augmented_string(), input_string);
        assert_eq!(doc.get_changed_count(), 0)
    }

    #[test]
    fn test_substitute_word_non_sampled_char() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, Some(0.0)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let input_string = String::from("Пример строки для аугментации");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        mock_aug.substitute(&mut doc, &mut rng);
        assert_eq!(doc.get_augmented_string(), input_string);
        assert_eq!(doc.get_changed_count(), 0)
    }

    #[test]
    fn test_substitute_word_non_chars_in_model() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let input_string = String::from("Пример ещё один");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        mock_aug.substitute(&mut doc, &mut rng);
        assert_eq!(doc.get_augmented_string(), input_string);
        assert_eq!(doc.get_changed_count(), 0)
    }

    #[test]
    fn test_substitute_word() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(None, None, None),
            aug_params_word: AugCountParams::new(Some(2), None, None),
            model: model,
        };
        let input_string = String::from("Апельсин гора стакан");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        mock_aug.substitute(&mut doc, &mut rng);
        assert_ne!(
            doc.get_augmented_string(),
            String::from("Апельсин гора стакан")
        );
        assert_eq!(
            utils::get_chars_len(&doc.get_augmented_string()),
            utils::get_chars_len(&input_string)
        );
        assert_eq!(doc.get_changed_count(), 2)
    }
}
