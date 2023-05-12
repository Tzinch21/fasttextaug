use rand::rngs::StdRng;

use super::super::{AugCountParams, BaseAugmentor};
use super::CharacterAugmentor;
use crate::doc::Doc;
use crate::model::character::OcrModel;
use std::collections::HashSet;

pub struct OcrAugmentor<'a> {
    aug_params_char: AugCountParams,
    aug_params_word: AugCountParams,
    min_chars: Option<usize>,
    model: &'a OcrModel,
    stopwords: Option<&'a HashSet<String>>,
}

impl<'a> OcrAugmentor<'a> {
    fn new(
        aug_params_char: AugCountParams,
        aug_params_word: AugCountParams,
        min_chars: Option<usize>,
        model: &'a OcrModel,
        stopwords: Option<&'a HashSet<String>>,
    ) -> Self {
        OcrAugmentor {
            aug_params_char,
            aug_params_word,
            min_chars,
            model,
            stopwords,
        }
    }
}

impl<'a> BaseAugmentor<OcrModel> for OcrAugmentor<'a> {
    fn augment(&self, doc: &mut Doc, rng: &mut StdRng) -> () {
        self.substitute(doc, rng)
    }
    fn get_action(&self) -> () {}
    fn get_aug_params_word(&self) -> &AugCountParams {
        &self.aug_params_word
    }
    fn get_min_chars(&self) -> Option<usize> {
        self.min_chars
    }
    fn get_model(&self) -> &OcrModel {
        self.model
    }
    fn get_stopwords(&self) -> Option<&HashSet<String>> {
        self.stopwords
    }
}

impl<'a> CharacterAugmentor<OcrModel> for OcrAugmentor<'a> {
    fn get_aug_params_char(&self) -> &AugCountParams {
        &self.aug_params_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use rand::SeedableRng;

    #[test]
    fn test_substitute() {
        let mut model = OcrModel::new(String::from("test_res/ocr_en.json"));
        model.load_model();
        let stopwords = HashSet::from([String::from("fox"), String::from("The")]);
        let augmentor = OcrAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(3),
            &model,
            Some(&stopwords),
        );
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(input_string.clone());
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
    fn test_substitute_cyrillic() {
        let mut model = OcrModel::new(String::from("test_res/ocr_ru.json"));
        model.load_model();
        let stopwords = HashSet::from([String::from("пример"), String::from("Я")]);
        let augmentor = OcrAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(4),
            &model,
            Some(&stopwords),
        );
        let input_string = String::from("Очень важный пример для аугментации");
        let mut doc = Doc::new(input_string.clone());
        let mut rng: StdRng = SeedableRng::from_entropy();
        augmentor.substitute(&mut doc, &mut rng);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(
            utils::get_chars_len(&result),
            utils::get_chars_len(&input_string)
        );
        assert!(result.contains("пример"));
    }
}
