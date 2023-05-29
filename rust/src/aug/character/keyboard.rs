use super::super::{Action, AugCountParams, BaseAugmentor};
use super::CharacterAugmentor;
use crate::model::character::KeyboardModel;
use std::collections::HashSet;
use std::sync::Arc;

/// Augmentor, which performs KeyboardModel on text
pub struct KeyboardAugmentor {
    /// Parameteres to calculate number of chars that will be augmented in single word
    aug_params_char: AugCountParams,
    /// Parameteres to calculate number of words that will be augmented
    aug_params_word: AugCountParams,
    /// Filter, do not augment word, if it's lenght less than this value
    min_chars: Option<usize>,
    /// KeyboardModel
    model: Arc<KeyboardModel>,
    /// Filter, Set of words that cannot be augmented
    stopwords: Arc<Option<HashSet<String>>>,
    /// Flag, if it's true then we can augment special_chars
    use_special_chars: bool,
}

impl KeyboardAugmentor {
    pub fn new(
        aug_params_char: AugCountParams,
        aug_params_word: AugCountParams,
        min_chars: Option<usize>,
        model: Arc<KeyboardModel>,
        stopwords: Arc<Option<HashSet<String>>>,
    ) -> Self {
        let use_special_chars = model.get_allow_special_char();
        KeyboardAugmentor {
            aug_params_char,
            aug_params_word,
            min_chars,
            model,
            stopwords,
            use_special_chars,
        }
    }
}

impl BaseAugmentor<KeyboardModel> for KeyboardAugmentor {
    fn augment(&self, doc: &mut crate::doc::Doc, rng: &mut rand::rngs::StdRng) -> () {
        self.substitute(doc, rng)
    }
    fn get_action(&self) -> Action {
        Action::Substitute
    }
    fn get_aug_params_word(&self) -> &AugCountParams {
        &self.aug_params_word
    }
    fn get_min_chars(&self) -> Option<usize> {
        self.min_chars
    }
    fn get_model(&self) -> &KeyboardModel {
        self.model.as_ref()
    }
    fn get_stopwords(&self) -> Option<&HashSet<String>> {
        self.stopwords.as_ref().as_ref()
    }
    fn get_use_special_chars(&self) -> bool {
        self.use_special_chars
    }
}

impl CharacterAugmentor<KeyboardModel> for KeyboardAugmentor {
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
    fn test_substitute_all_false() {
        let mut model = KeyboardModel::new(
            false,
            false,
            false,
            String::from("test_res/keyboard_en.json"),
        );
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("The"),
        ])));
        // KeyboardAugmentor::new(aug_params_char, aug_params_word, min_chars, &model, stopwords)
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
        );
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        augmentor.substitute(&mut doc, &mut rng);
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
    fn test_substitute_all_false_cyrillic() {
        let mut model = KeyboardModel::new(
            false,
            false,
            false,
            String::from("test_res/keyboard_ru.json"),
        );
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("для"),
            String::from("Юнит"),
        ])));
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
        );
        let input_string = String::from("Юнит-тест для тестов, тестов");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::from_entropy();
        augmentor.substitute(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("Юнит"));
        assert!(result.contains("для"));
    }

    #[test]
    fn test_substitute_with_upper() {
        let mut model = KeyboardModel::new(
            false,
            false,
            true,
            String::from("test_res/keyboard_en.json"),
        );
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("the"),
        ])));
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
        );
        let input_string = String::from("the quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        augmentor.substitute(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("the"));
        assert!(result.contains("fox"));

        let mut upper_case_count: usize = 0;
        for ch in result.chars() {
            if ch.is_uppercase() {
                upper_case_count += 1
            }
        }
        assert!(upper_case_count > 0)
    }

    #[test]
    fn test_substitute_with_upper_cyrillic() {
        let mut model = KeyboardModel::new(
            false,
            false,
            true,
            String::from("test_res/keyboard_ru.json"),
        );
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([String::from("Пример")])));
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
        );
        let input_string = String::from("Пример строки для аугментации");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        augmentor.substitute(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("Пример"));

        let mut upper_case_count: usize = 0;
        for ch in result.chars() {
            if ch.is_uppercase() {
                upper_case_count += 1
            }
        }
        assert!(upper_case_count > 0)
    }

    #[test]
    fn test_substitute_special_chars() {
        let mut model = KeyboardModel::new(
            true,
            false,
            false,
            String::from("test_res/keyboard_en.json"),
        );
        model.load_model();
        let arc_model = Arc::new(model);
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(3), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            None,
            Arc::clone(&arc_model),
            Arc::new(None),
        );
        let input_string = String::from("$$$$$$$!$@@$@$$@!!!!");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        augmentor.substitute(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );

        let mut special_char_count: usize = 0;
        for ch in result.chars() {
            if !ch.is_alphanumeric() {
                special_char_count += 1
            }
        }
        assert!(special_char_count < 20)
    }

    #[test]
    fn test_substitute_numeric() {
        let mut model = KeyboardModel::new(
            false,
            true,
            false,
            String::from("test_res/keyboard_en.json"),
        );
        model.load_model();
        let arc_model = Arc::new(model);
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            None,
            Arc::clone(&arc_model),
            Arc::new(None),
        );
        let input_string = String::from("0351368213471238123512409");
        let mut doc = Doc::new(&input_string);
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);
        augmentor.substitute(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );

        let mut numeric_count: usize = 0;
        for ch in result.chars() {
            if !ch.is_numeric() {
                numeric_count += 1
            }
        }
        assert!(numeric_count < 25)
    }
}
