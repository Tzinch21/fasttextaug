use super::super::{Action, AugCountParams, BaseAugmentor};
use super::CharacterAugmentor;
use crate::model::character::OcrModel;
use std::collections::HashSet;
use std::sync::Arc;

/// Augmentor, which performs OcrModel on text
pub struct OcrAugmentor {
    /// Parameteres to calculate number of chars that will be augmented in single word
    aug_params_char: AugCountParams,
    /// Parameteres to calculate number of words that will be augmented
    aug_params_word: AugCountParams,
    /// Filter, do not augment word, if it's lenght less than this value
    min_chars: Option<usize>,
    /// OcrModel
    model: Arc<OcrModel>,
    /// Filter, Set of words that cannot be augmented
    stopwords: Arc<Option<HashSet<String>>>,
}

impl OcrAugmentor {
    pub fn new(
        aug_params_char: AugCountParams,
        aug_params_word: AugCountParams,
        min_chars: Option<usize>,
        model: Arc<OcrModel>,
        stopwords: Arc<Option<HashSet<String>>>,
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

impl BaseAugmentor<OcrModel> for OcrAugmentor {
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
    fn get_model(&self) -> &OcrModel {
        self.model.as_ref()
    }
    fn get_stopwords(&self) -> Option<&HashSet<String>> {
        self.stopwords.as_ref().as_ref()
    }
}

impl CharacterAugmentor<OcrModel> for OcrAugmentor {
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
    fn test_substitute() {
        let mut model = OcrModel::new(String::from("test_res/ocr_en.json"));
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("fox"),
            String::from("The"),
        ])));
        let augmentor = OcrAugmentor::new(
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
    fn test_substitute_cyrillic() {
        let mut model = OcrModel::new(String::from("test_res/ocr_ru.json"));
        model.load_model();
        let arc_model = Arc::new(model);
        let stopwords = Arc::new(Some(HashSet::from([
            String::from("пример"),
            String::from("Я"),
        ])));
        let augmentor = OcrAugmentor::new(
            AugCountParams::new(Some(1), Some(5), None),
            AugCountParams::new(Some(2), Some(6), None),
            Some(4),
            Arc::clone(&arc_model),
            Arc::clone(&stopwords),
        );
        let input_string = String::from("Очень важный пример для аугментации");
        let mut doc = Doc::new(&input_string);
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
