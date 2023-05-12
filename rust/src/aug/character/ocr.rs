use super::super::{AugCountParams, BaseAugmentor};
use super::CharacterAugmentor;
use crate::doc::{Doc, TokenType};
use crate::model::character::OcrModel;
use std::collections::HashSet;

struct OcrAugmentor<'a> {
    aug_params_char: AugCountParams,
    aug_params_word: AugCountParams,
    min_chars: Option<usize>,
    model: &'a OcrModel,
    stopwords: Option<&'a HashSet<String>>,
}

impl<'a> BaseAugmentor<OcrModel> for OcrAugmentor<'a> {
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

impl<'a> OcrAugmentor<'a> {
    fn substitute(&self, doc: &mut Doc) -> () {
        let aug_tokens = self.sample_word_tokens_to_aug(doc);
        let mut change_seq = 0;
        for a_token in aug_tokens {
            let raw_token = a_token.get_original().token();
            let aug_chars_indexes = self.sample_chars_to_aug(raw_token);
            let mut result = String::with_capacity(raw_token.len() * 2);
            raw_token
                .chars()
                .enumerate()
                .map(|(idx, ch)| self.predict_char(idx, ch, &aug_chars_indexes))
                .for_each(|x| result.push_str(&x));
            a_token.add_change(TokenType::WordToken, result, change_seq);
            change_seq += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute() {
        let mut model = OcrModel::new(String::from("test_res/ocr_en.json"));
        model.load_model();
        let stopwords = HashSet::from([String::from("fox"), String::from("The")]);
        let augmentor = OcrAugmentor {
            aug_params_char: AugCountParams::new(Some(1), Some(5), None),
            aug_params_word: AugCountParams::new(Some(2), Some(6), None),
            min_chars: Some(3),
            model: &model,
            stopwords: Some(&stopwords),
        };
        let input_string = String::from("The quick brown fox jumps over the lazy dog .");
        let mut doc = Doc::new(input_string.clone());
        augmentor.substitute(&mut doc);
        let result = doc.get_augmented_string();
        assert_ne!(result, input_string);
        assert_eq!(result.len(), input_string.len());
        assert!(result.contains("The"));
        assert!(result.contains("fox"));
    }

    // #[test]
    // fn test_substitute_cyrillic() {
    //     let mut model = OcrModel::new(String::from("test_res/ocr_ru.json"));
    //     model.load_model();
    //     let stopwords = HashSet::from([
    //         String::from("пример"),
    //         String::from("Я")
    //     ]);
    //     let augmentor = OcrAugmentor{
    //         aug_params_char: AugCountParams::new(Some(1), Some(5), None),
    //         aug_params_word: AugCountParams::new(Some(2), Some(6), None),
    //         min_chars: Some(4),
    //         model:&model,
    //         stopwords: Some(&stopwords)
    //     };
    //     let input_string = String::from("Очень важный пример для аугментации");
    //     let mut doc = Doc::new(input_string.clone());
    //     augmentor.substitute(&mut doc);
    //     let result = doc.get_augmented_string();
    //     assert_eq!(result, input_string);
    //     assert_eq!(result.len(), input_string.len())
    // }
}
