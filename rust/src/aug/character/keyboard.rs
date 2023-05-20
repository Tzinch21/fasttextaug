use super::super::{AugCountParams, BaseAugmentor};
use super::CharacterAugmentor;
use crate::model::character::KeyboardModel;
use std::collections::HashSet;

pub struct KeyboardAugmentor<'a> {
    aug_params_char: AugCountParams,
    aug_params_word: AugCountParams,
    min_chars: Option<usize>,
    model: &'a KeyboardModel,
    stopwords: Option<&'a HashSet<String>>,
    use_special_chars: bool,
}

impl<'a> KeyboardAugmentor<'a> {
    pub fn new(
        aug_params_char: AugCountParams,
        aug_params_word: AugCountParams,
        min_chars: Option<usize>,
        model: &'a KeyboardModel,
        stopwords: Option<&'a HashSet<String>>,
    ) -> Self {
        KeyboardAugmentor {
            aug_params_char,
            aug_params_word,
            min_chars,
            model,
            stopwords,
            use_special_chars: model.get_allow_special_char(),
        }
    }
}

impl<'a> BaseAugmentor<KeyboardModel> for KeyboardAugmentor<'a> {
    fn get_action(&self) -> () {}
    fn get_aug_params_word(&self) -> &AugCountParams {
        &self.aug_params_word
    }
    fn get_min_chars(&self) -> Option<usize> {
        self.min_chars
    }
    fn get_model(&self) -> &KeyboardModel {
        self.model
    }
    fn get_stopwords(&self) -> Option<&HashSet<String>> {
        self.stopwords
    }
    fn get_use_special_chars(&self) -> bool {
        self.use_special_chars
    }
}

impl<'a> CharacterAugmentor<KeyboardModel> for KeyboardAugmentor<'a> {
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
        let stopwords = HashSet::from([String::from("fox"), String::from("The")]);
        // KeyboardAugmentor::new(aug_params_char, aug_params_word, min_chars, &model, stopwords)
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            &model,
            Some(&stopwords),
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
        let stopwords = HashSet::from([String::from("для"), String::from("Юнит")]);
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            &model,
            Some(&stopwords),
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
        let stopwords = HashSet::from([String::from("fox"), String::from("the")]);
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            &model,
            Some(&stopwords),
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
        let stopwords = HashSet::from([String::from("Пример")]);
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            &model,
            Some(&stopwords),
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
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(3), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            None,
            &model,
            None,
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
        let augmentor = KeyboardAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            None,
            &model,
            None,
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
