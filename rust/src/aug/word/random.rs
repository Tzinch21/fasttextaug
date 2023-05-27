use std::collections::HashSet;
use std::sync::Arc;

use super::super::BaseAugmentor;
use crate::aug::{Action, AugCountParams};
use crate::doc::Doc;
use crate::doc::TokenType;
use crate::model::word::RandomWordModel;
use crate::model::BaseModel;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

/// Augmentor, which performs RandomWordModel on text
pub struct RandomWordAugmentor {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: Action,
    /// Parameteres to calculate number of words that will be augmented
    aug_params_word: AugCountParams,
    /// RandomWordModel
    model: Arc<RandomWordModel>,
    /// Filter, Set of words that cannot be augmented
    stopwords: Arc<Option<HashSet<String>>>,
    /// Flag, use model to filter words to augmentation
    use_model_in_sampler_words: bool,
}

impl RandomWordAugmentor {
    pub fn new(
        action: Action,
        aug_params_word: AugCountParams,
        model: Arc<RandomWordModel>,
        stopwords: Arc<Option<HashSet<String>>>,
    ) -> Self {
        let use_model_in_sampler_words = match action {
            Action::Substitute => true,
            _ => false,
        };
        Self {
            action,
            aug_params_word,
            model,
            stopwords,
            use_model_in_sampler_words,
        }
    }

    /// Action::Substitute augmentation
    fn substitute(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        let aug_tokens = self.sample_word_tokens_to_aug(doc, rng);
        let mut change_seq = 0;
        for (_, a_token) in aug_tokens {
            let original_token = a_token.get_original();
            let predict = self.get_model().predict(&original_token.token());
            if let Some(predicted) = predict {
                let replacer = predicted.into_iter().choose(rng);
                if let Some(value) = replacer {
                    a_token.change(TokenType::WordToken, value.to_owned());
                    change_seq += 1
                }
            }
        }
        doc.set_change_count(change_seq);
    }

    /// Action::Delete augmentation
    fn delete(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        let aug_tokens = self.sample_word_tokens_to_aug(doc, rng);
        let mut change_seq = 0;
        for (_, a_token) in aug_tokens {
            a_token.change(TokenType::SpaceToken, String::new());
            change_seq += 1;
        }
        doc.set_change_count(change_seq);
    }

    /// Swap strategy for words
    fn get_swap_position(&self, pos: usize, possible_indexes: &Vec<usize>) -> Option<usize> {
        if possible_indexes.len() < 2 {
            return None;
        }
        let curr_pos = possible_indexes.binary_search(&pos);
        if let Ok(pos_idx) = curr_pos {
            if pos_idx == 0 {
                return Some(possible_indexes[pos_idx + 1]);
            } else if pos_idx == possible_indexes.len() - 1 {
                return Some(possible_indexes[pos_idx - 1]);
            } else {
                if rand::random() {
                    return Some(possible_indexes[pos_idx + 1]);
                } else {
                    return Some(possible_indexes[pos_idx - 1]);
                }
            }
        }
        None
    }

    /// Action::Swap augmentation
    fn swap(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        let word_token_indexes = doc.get_word_indexes(self.get_use_special_chars());
        let aug_tokens = self.sample_word_tokens_to_aug(doc, rng);
        let mut change_seq = 0;
        let mut swap_pairs = Vec::with_capacity(aug_tokens.len());
        for (idx, _) in aug_tokens {
            let swap_position = self.get_swap_position(idx, &word_token_indexes);
            if let Some(swap_pos) = swap_position {
                swap_pairs.push((idx, swap_pos));
            }
        }
        for (idx_a, idx_b) in swap_pairs {
            doc.perform_swap_by_idx(idx_a, idx_b);
            change_seq += 1;
        }
        doc.set_change_count(change_seq);
    }
}

impl BaseAugmentor<RandomWordModel> for RandomWordAugmentor {
    fn augment(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        match self.action {
            Action::Insert => (),
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

    fn get_flag_use_model_in_sampling_words(&self) -> bool {
        self.use_model_in_sampler_words
    }

    fn get_model(&self) -> &RandomWordModel {
        self.model.as_ref()
    }

    fn get_stopwords(&self) -> Option<&HashSet<String>> {
        self.stopwords.as_ref().as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_sampled_vec_model() {
        let model = RandomWordModel::from_vec(vec![String::from("word")]);
        let aug = RandomWordAugmentor::new(
            Action::Substitute,
            AugCountParams::new(Some(10), Some(100), Some(1.0)),
            Arc::new(model),
            Arc::new(None),
        );
        let input_string = String::from("My new input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 4);

        aug.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(input_string, result);
        assert_eq!(result, String::from("word word word word!"));
    }

    #[test]
    fn test_sampled_vec_model_with_stopwords() {
        let model = RandomWordModel::from_vec(vec![String::from("word")]);
        let stopwords = HashSet::from([String::from("My"), String::from("new")]);
        let aug = RandomWordAugmentor::new(
            Action::Substitute,
            AugCountParams::new(Some(10), Some(100), Some(1.0)),
            Arc::new(model),
            Arc::new(Some(stopwords)),
        );
        let input_string = String::from("My new input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 2);

        aug.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(input_string, result);
        assert_eq!(result, String::from("My new word word!"));
    }

    #[test]
    fn test_sampled_dict_model() {
        let model = RandomWordModel::from_map(HashMap::from([
            (
                String::from("My"),
                vec![String::from("Their"), String::from("Our")],
            ),
            (String::from("new"), vec![String::from("fresh")]),
        ]));
        let aug = RandomWordAugmentor::new(
            Action::Substitute,
            AugCountParams::new(Some(10), Some(100), Some(1.0)),
            Arc::new(model),
            Arc::new(None),
        );
        let input_string = String::from("My new input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 2);

        aug.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(input_string, result);
        assert!(
            (result == String::from("Their fresh input string!"))
                | (result == String::from("Our fresh input string!"))
        );
    }

    #[test]
    fn test_sampled_dict_model_with_stopwords() {
        let model = RandomWordModel::from_map(HashMap::from([
            (
                String::from("My"),
                vec![String::from("Their"), String::from("Our")],
            ),
            (String::from("new"), vec![String::from("fresh")]),
        ]));
        let stopwords = HashSet::from([String::from("My"), String::from("new")]);
        let aug = RandomWordAugmentor::new(
            Action::Substitute,
            AugCountParams::new(Some(10), Some(100), Some(1.0)),
            Arc::new(model),
            Arc::new(Some(stopwords)),
        );
        let input_string = String::from("My new input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        let result = aug.sample_word_tokens_to_aug(&mut doc, &mut rng);
        assert_eq!(result.len(), 0);

        aug.augment(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_eq!(input_string, result);
    }

    #[test]
    fn test_swap_tokens() {
        let model = RandomWordModel::from_vec(vec![String::from("word")]);
        let aug = RandomWordAugmentor::new(
            Action::Swap,
            AugCountParams::new(None, None, Some(0.2)),
            Arc::new(model),
            Arc::new(None),
        );
        let input_string = String::from("My new!! input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        aug.augment(&mut doc, &mut rng);
        assert_ne!(input_string, doc.get_augmented_string());
        assert_eq!(doc.get_changed_count(), 1)
    }

    #[test]
    fn test_swap_two_times() {
        let model = RandomWordModel::from_vec(vec![]);
        let aug = RandomWordAugmentor::new(
            Action::Swap,
            AugCountParams::new(None, None, Some(1.0)),
            Arc::new(model),
            Arc::new(None),
        );
        let input_string = String::from("Test string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        aug.augment(&mut doc, &mut rng);
        assert_eq!(input_string, doc.get_augmented_string());
        assert_eq!(doc.get_changed_count(), 2)
    }

    #[test]
    fn test_delete_some_tokens() {
        let model = RandomWordModel::from_vec(vec![]);
        let aug = RandomWordAugmentor::new(
            Action::Delete,
            AugCountParams::new(None, None, Some(0.5)),
            Arc::new(model),
            Arc::new(None),
        );
        let input_string = String::from("My new!! input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        aug.augment(&mut doc, &mut rng);
        assert_ne!(input_string, doc.get_augmented_string());
        assert!(input_string.len() > doc.get_augmented_string().len())
    }

    #[test]
    fn test_delete_all_tokens() {
        let model = RandomWordModel::from_vec(vec![]);
        let aug = RandomWordAugmentor::new(
            Action::Delete,
            AugCountParams::new(None, None, Some(1.0)),
            Arc::new(model),
            Arc::new(None),
        );
        let input_string = String::from("My new!! input string!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        aug.augment(&mut doc, &mut rng);
        assert_eq!(doc.get_augmented_string(), String::from(" !!  !"));
    }
}
