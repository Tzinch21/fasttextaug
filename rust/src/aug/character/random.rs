use super::super::{Action, AugCountParams, BaseAugmentor};
use super::CharacterAugmentor;
use crate::doc::{Doc, TokenType};
use crate::model::character::RandomCharModel;
use crate::model::BaseModel;
use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use std::collections::HashSet;
use std::sync::Arc;

enum SwapMode {
    Adjacent,
    Middle,
    Random,
}

pub struct RandomCharAugmentor {
    action: Action,
    aug_params_char: AugCountParams,
    aug_params_word: AugCountParams,
    min_chars: Option<usize>,
    model: Arc<RandomCharModel>,
    stopwords: Arc<Option<HashSet<String>>>,
    swapmode: SwapMode,
}

impl RandomCharAugmentor {
    pub fn new(
        action: Action,
        aug_params_char: AugCountParams,
        aug_params_word: AugCountParams,
        min_chars: Option<usize>,
        model: Arc<RandomCharModel>,
        stopwords: Arc<Option<HashSet<String>>>,
        swapmode: String,
    ) -> Self {
        let swapmode = match &swapmode[..] {
            "adjacent" => SwapMode::Adjacent,
            "middle" => SwapMode::Middle,
            "random" => SwapMode::Random,
            _ => SwapMode::Adjacent,
        };
        Self {
            action,
            aug_params_char,
            aug_params_word,
            min_chars,
            model,
            stopwords,
            swapmode,
        }
    }

    fn insert_predicted_char(
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
                    let mut value = value.clone();
                    value.push_str(&ch_str);
                    return value;
                }
            }
        }
        ch_str
    }

    fn insert(&self, doc: &mut Doc, rng: &mut StdRng) {
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
                .map(|(idx, ch)| self.insert_predicted_char(idx, ch, &aug_chars_indexes, rng))
                .for_each(|x| result.push_str(&x));
            result.shrink_to_fit();
            a_token.change(TokenType::WordToken, result);
            change_seq += 1;
        }
        doc.set_change_count(change_seq);
    }

    fn delete(&self, doc: &mut Doc, rng: &mut StdRng) {
        let aug_tokens = self.sample_word_tokens_to_aug(doc, rng);
        let mut change_seq = 0;
        for a_token in aug_tokens {
            let original_token = a_token.get_original();
            let aug_chars_indexes = self.sample_chars_to_aug(original_token, rng);
            if aug_chars_indexes.len() == 0 {
                continue;
            }
            let mut result = String::with_capacity(original_token.byte_len());
            original_token
                .token()
                .chars()
                .enumerate()
                .map(|(idx, ch)| {
                    if aug_chars_indexes.contains(&idx) {
                        return String::new();
                    }
                    return ch.to_string();
                })
                .for_each(|x| result.push_str(&x));
            result.shrink_to_fit();
            a_token.change(TokenType::WordToken, result);
            change_seq += 1;
        }
        doc.set_change_count(change_seq);
    }

    fn get_swap_position(&self, pos: usize, token_length: usize, rng: &mut StdRng) -> usize {
        let new_pos: usize;
        match self.swapmode {
            SwapMode::Adjacent => {
                if pos == 0 {
                    new_pos = 1;
                } else if pos == token_length {
                    new_pos = pos - 1;
                } else {
                    if rand::random() {
                        new_pos = pos + 1
                    } else {
                        new_pos = pos - 1
                    }
                }
                new_pos
            }
            SwapMode::Middle => {
                let candidates: Vec<usize> = (1..token_length).filter(|x| *x != pos).collect();
                if candidates.len() == 0 {
                    new_pos = pos
                } else {
                    if let Some(choosed) = candidates.iter().choose(rng) {
                        new_pos = *choosed
                    } else {
                        new_pos = pos
                    }
                }
                new_pos
            }
            SwapMode::Random => {
                let candidates: Vec<usize> = (0..token_length + 1).filter(|x| *x != pos).collect();
                if candidates.len() == 0 {
                    new_pos = pos
                } else {
                    if let Some(choosed) = candidates.iter().choose(rng) {
                        new_pos = *choosed
                    } else {
                        new_pos = pos
                    }
                }
                new_pos
            }
        }
    }

    fn swap(&self, doc: &mut Doc, rng: &mut StdRng) {
        let aug_tokens = self.sample_word_tokens_to_aug(doc, rng);
        let mut change_seq = 0;
        for a_token in aug_tokens {
            let original_token = a_token.get_original();
            if a_token.get_original().utf8_len() < 2 {
                continue;
            }
            let aug_chars_indexes = self.sample_chars_to_aug(original_token, rng);
            if aug_chars_indexes.len() == 0 {
                continue;
            }
            let mut result = original_token.token().clone();
            for aug_char_idx in aug_chars_indexes {
                let swap_position =
                    self.get_swap_position(aug_char_idx, original_token.utf8_len() - 1, rng);
                if swap_position != aug_char_idx {
                    let mut origin_char_op: Option<char> = None;
                    let mut swap_char_op: Option<char> = None;
                    for (i, ch) in result.chars().enumerate() {
                        if i == aug_char_idx {
                            origin_char_op = Some(ch)
                        }
                        if i == swap_position {
                            swap_char_op = Some(ch)
                        }
                    }
                    if origin_char_op.is_some() & swap_char_op.is_some() {
                        let origin_is_alphabetic = origin_char_op.unwrap().is_alphabetic();
                        let swap_is_alphabetic = swap_char_op.unwrap().is_alphabetic();
                        let origin_char: String;
                        let swap_char: String;
                        if origin_is_alphabetic & swap_is_alphabetic {
                            let origin_upper = origin_char_op.unwrap().to_uppercase().to_string()
                                == origin_char_op.unwrap().to_string();
                            let swap_upper = swap_char_op.unwrap().to_uppercase().to_string()
                                == swap_char_op.unwrap().to_string();
                            (origin_char, swap_char) = match (origin_upper, swap_upper) {
                                (true, false) => {
                                    let origin_char_val =
                                        origin_char_op.unwrap().to_lowercase().to_string();
                                    let swap_char_val =
                                        swap_char_op.unwrap().to_uppercase().to_string();
                                    (origin_char_val, swap_char_val)
                                }
                                (false, true) => {
                                    let origin_char_val =
                                        origin_char_op.unwrap().to_uppercase().to_string();
                                    let swap_char_val =
                                        swap_char_op.unwrap().to_lowercase().to_string();
                                    (origin_char_val, swap_char_val)
                                }
                                (_, _) => (
                                    origin_char_op.unwrap().to_string(),
                                    swap_char_op.unwrap().to_string(),
                                ),
                            };
                        } else {
                            origin_char = origin_char_op.unwrap().to_string();
                            swap_char = swap_char_op.unwrap().to_string();
                        }
                        result = result
                            .chars()
                            .enumerate()
                            .map(|(i, ch)| {
                                if i == aug_char_idx {
                                    return swap_char.clone();
                                } else if i == swap_position {
                                    return origin_char.clone();
                                } else {
                                    return ch.to_string();
                                }
                            })
                            .collect();
                    }
                }
            }
            result.shrink_to_fit();
            a_token.change(TokenType::WordToken, result);
            change_seq += 1;
        }
        doc.set_change_count(change_seq);
    }
}

impl BaseAugmentor<RandomCharModel> for RandomCharAugmentor {
    fn augment(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        match self.action {
            Action::Insert => self.insert(doc, rng),
            Action::Substitute => self.substitute(doc, rng),
            Action::Delete => self.delete(doc, rng),
            Action::Swap => self.swap(doc, rng),
        }
    }
    fn get_action(&self) -> Action {
        self.action
    }
    fn get_aug_params_word(&self) -> &AugCountParams {
        &self.aug_params_word
    }
    fn get_min_chars(&self) -> Option<usize> {
        self.min_chars
    }
    fn get_model(&self) -> &RandomCharModel {
        self.model.as_ref()
    }
    fn get_stopwords(&self) -> Option<&HashSet<String>> {
        self.stopwords.as_ref().as_ref()
    }
}

impl CharacterAugmentor<RandomCharModel> for RandomCharAugmentor {
    fn get_aug_params_char(&self) -> &AugCountParams {
        &self.aug_params_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::Doc;
    use crate::utils;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_substitute_some_data() {
        let mut model = RandomCharModel::new(true, true, true, true, "en", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("The"),
        ])));
        let augmentor = RandomCharAugmentor::new(
            Action::Substitute,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("The"));
        assert!(result.contains("fox"));
    }

    #[test]
    fn test_substitute_some_cyrillic() {
        let mut model = RandomCharModel::new(true, true, true, true, "ru", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([String::from("Привет")])));
        let augmentor = RandomCharAugmentor::new(
            Action::Substitute,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("Привет, попробуем аугментировать эту строку");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("Привет"));
    }

    #[test]
    fn test_insert_some_data() {
        let mut model = RandomCharModel::new(true, true, true, true, "en", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("The"),
        ])));
        let augmentor = RandomCharAugmentor::new(
            Action::Insert,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert!(utils::get_chars_len(&result) > utils::get_chars_len(&input_string));
        assert!(result.contains("The"));
        assert!(result.contains("fox"));
    }

    #[test]
    fn test_insert_some_cyrillic() {
        let mut model = RandomCharModel::new(true, true, true, true, "ru", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([String::from("Привет")])));
        let augmentor = RandomCharAugmentor::new(
            Action::Insert,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("Привет, попробуем аугментировать эту строку");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert!(utils::get_chars_len(&result) > utils::get_chars_len(&input_string));
        assert!(result.contains("Привет"));
    }

    #[test]
    fn get_swap_position_adjacent() {
        let aug = RandomCharAugmentor::new(
            Action::Swap,
            AugCountParams::new(None, None, None),
            AugCountParams::new(None, None, None),
            Some(2),
            Arc::new(RandomCharModel::new(false, false, false, false, "en", None)),
            Arc::new(None),
            String::from("adjacent"),
        );
        let mut rng: StdRng = SeedableRng::from_entropy();
        let swap_position = aug.get_swap_position(0, 5, &mut rng);
        assert_eq!(swap_position, 1);
        let swap_position = aug.get_swap_position(5, 5, &mut rng);
        assert_eq!(swap_position, 4);
        let swap_position = aug.get_swap_position(2, 5, &mut rng);
        assert!((swap_position == 1) | (swap_position == 3));
    }

    #[test]
    fn get_swap_position_middle() {
        let aug = RandomCharAugmentor::new(
            Action::Swap,
            AugCountParams::new(None, None, None),
            AugCountParams::new(None, None, None),
            Some(2),
            Arc::new(RandomCharModel::new(false, false, false, false, "en", None)),
            Arc::new(None),
            String::from("middle"),
        );
        let mut rng: StdRng = SeedableRng::from_entropy();
        for _ in 0..20 {
            let swap_position = aug.get_swap_position(2, 5, &mut rng);
            assert!(swap_position != 0);
            assert!(swap_position != 2);
            assert!(swap_position != 5);
        }
        for _ in 0..20 {
            let swap_position = aug.get_swap_position(0, 2, &mut rng);
            assert_eq!(swap_position, 1);
        }
        for _ in 0..20 {
            let swap_position = aug.get_swap_position(1, 2, &mut rng);
            assert_eq!(swap_position, 1);
        }
    }

    #[test]
    fn get_swap_position_random() {
        let aug = RandomCharAugmentor::new(
            Action::Swap,
            AugCountParams::new(None, None, None),
            AugCountParams::new(None, None, None),
            Some(2),
            Arc::new(RandomCharModel::new(false, false, false, false, "en", None)),
            Arc::new(None),
            String::from("random"),
        );
        let mut rng: StdRng = SeedableRng::from_entropy();
        for _ in 0..20 {
            let swap_position = aug.get_swap_position(1, 1, &mut rng);
            assert_eq!(swap_position, 0);
        }
        for _ in 0..20 {
            let swap_position = aug.get_swap_position(1, 2, &mut rng);
            assert!((swap_position == 0) | (swap_position == 2));
        }
    }

    #[test]
    fn test_swap_some_data() {
        let mut model = RandomCharModel::new(true, true, true, true, "ru", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("The"),
        ])));
        let augmentor = RandomCharAugmentor::new(
            Action::Swap,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("The"));
        assert!(result.contains("fox"));
    }

    #[test]
    fn test_swap_some_cyrillic() {
        let mut model = RandomCharModel::new(true, true, true, true, "ru", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([String::from("Привет")])));
        let augmentor = RandomCharAugmentor::new(
            Action::Swap,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("Привет, попробуем аугментировать эту строку");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("Привет"));
    }

    #[test]
    fn test_delete_some_data() {
        let mut model = RandomCharModel::new(true, true, true, true, "en", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("The"),
        ])));
        let augmentor = RandomCharAugmentor::new(
            Action::Delete,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert!(utils::get_chars_len(&result) < utils::get_chars_len(&input_string));
        assert!(result.contains("The"));
        assert!(result.contains("fox"));
    }

    #[test]
    fn test_delete_some_cyrillic() {
        let mut model = RandomCharModel::new(true, true, true, true, "ru", None);
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([String::from("Привет")])));
        let augmentor = RandomCharAugmentor::new(
            Action::Delete,
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
            String::new(),
        );
        let input_string = String::from("Привет, попробуем аугментировать эту строку");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();

        augmentor.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert!(utils::get_chars_len(&result) < utils::get_chars_len(&input_string));
        assert!(result.contains("Привет"));
    }
}
