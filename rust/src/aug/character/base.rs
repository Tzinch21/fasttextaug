use super::super::{AugCountParams, BaseAugmentor};
use crate::model::character::CharacterModel;
use crate::utils;
use rand::{prelude::IteratorRandom, thread_rng};
use std::collections::HashSet;

pub trait CharacterAugmentor<T>: BaseAugmentor<T>
where
    T: CharacterModel,
{
    fn get_aug_params_char(&self) -> &AugCountParams;

    fn sample_chars_to_aug(&self, token: &String) -> HashSet<usize> {
        let chars_len = utils::get_chars_len(token);
        let mut char_indexes: Vec<usize> = Vec::with_capacity(chars_len);
        let aug_cnt = self.get_aug_params_char().calculate_aug_cnt(chars_len);
        for (idx, char) in token.chars().enumerate() {
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
            let mut rng = thread_rng();
            sampled = char_indexes.into_iter().choose_multiple(&mut rng, aug_cnt);
        }
        HashSet::from_iter(sampled)
    }

    fn predict_char(&self, idx: usize, ch: char, char_idxs: &HashSet<usize>) -> String {
        let mut rng = thread_rng();
        let ch_str = ch.to_string();
        if char_idxs.contains(&idx) {
            let predict = self.get_model().predict(&ch_str);
            if let Some(predicted) = predict {
                let replacer = predicted.into_iter().choose(&mut rng);
                if let Some(value) = replacer {
                    return value.clone();
                }
            }
        }
        ch_str
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::model::{BaseModel, Mapping};
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
        let res = mock_aug.sample_chars_to_aug(&String::from("Qvqv"));
        assert_eq!(res.len(), 2)
    }

    #[test]
    fn sample_chars_to_aug_cyrillic() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(3), Some(7), Some(0.4)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let res = mock_aug.sample_chars_to_aug(&String::from("агсагсагс"));
        // every char presented at model mapping (9), but aug_params get 4
        assert_eq!(res.len(), 4)
    }

    #[test]
    fn sample_chars_to_aug_get_all_possible() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(1), Some(10), Some(0.8)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let res = mock_aug.sample_chars_to_aug(&String::from("vavava"));
        // not every char presented at model mapping (only 3), but aug_params gets 5.
        // We still take all 3 possible variants
        assert_eq!(res.len(), 3)
    }

    #[test]
    fn sample_chars_to_aug_get_all_possible_cyrillic() {
        let model = MockModel::new();
        let mock_aug = MockAugmentor {
            aug_params_char: AugCountParams::new(Some(3), Some(7), Some(0.8)),
            aug_params_word: AugCountParams::new(None, None, None),
            model: model,
        };
        let res = mock_aug.sample_chars_to_aug(&String::from("Авиастроение"));
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
        let res = mock_aug.sample_chars_to_aug(&String::from("vavava"));
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
        let res = mock_aug.sample_chars_to_aug(&String::from("none"));
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
        let res = mock_aug.sample_chars_to_aug(&String::from("агс"));
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
        let res = mock_aug.sample_chars_to_aug(&String::from("ноль"));
        assert_eq!(res.len(), 0)
    }
}
