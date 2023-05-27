use super::super::{BaseModel, Mapping};
use super::CharacterModel;

/// Supported chars sequences
enum SupportedLanguage {
    EN,
    RU,
    Unknown,
}

/// Random char augmentations model
pub struct RandomCharModel {
    /// Allow uppercase chars to be in augments
    include_upper_case: bool,
    /// Allow lowercase chars to be in augments
    include_lower_case: bool,
    /// Allow special char to be in augments
    include_special_char: bool,
    /// Allow numeric char to be in augments
    include_numeric: bool,
    /// Supported language
    lang: SupportedLanguage,
    /// Your own String of special_chars to include
    spec_char: Option<String>,
    /// You own Vector of chars to use in model
    candidates: Option<Vec<String>>,
}

impl RandomCharModel {
    pub fn new(
        include_upper_case: bool,
        include_lower_case: bool,
        include_special_char: bool,
        include_numeric: bool,
        lang: &str,
        spec_char: Option<String>,
    ) -> Self {
        let lang = match lang {
            "en" => SupportedLanguage::EN,
            "ru" => SupportedLanguage::RU,
            _ => SupportedLanguage::Unknown,
        };

        let model = Self {
            include_upper_case,
            include_lower_case,
            include_special_char,
            include_numeric,
            lang,
            spec_char,
            candidates: None,
        };
        model
    }

    /// Instead using flag and lang, it's possible to directly pass vec of chars  to use
    ///
    /// String instead of char, because utf-16 symbols need more space to store
    pub fn from_candidates(candidates: Vec<String>) -> Self {
        Self {
            include_upper_case: false,
            include_lower_case: false,
            include_special_char: false,
            include_numeric: false,
            lang: SupportedLanguage::Unknown,
            spec_char: None,
            candidates: Some(candidates),
        }
    }

    fn get_special_chars(&self) -> String {
        if let Some(data) = &self.spec_char {
            return data.clone();
        }
        String::from("!@#$%^&*()_+")
    }

    /// Lazy-file read, before this method executed -> Model = None
    pub fn load_model(&mut self) {
        if let Some(_) = self.candidates {
            return;
        }
        let spec_val_str = self.get_special_chars();
        let mut candidates = Vec::with_capacity(100);
        let (upper_lang, lower_lang) = match self.lang {
            SupportedLanguage::EN => ("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz"),
            SupportedLanguage::RU => (
                "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ",
                "абвгдеёжзийклмнопрстуфхцчшщъыьэюя",
            ),
            SupportedLanguage::Unknown => ("", ""),
        };
        if self.include_upper_case {
            candidates.extend(upper_lang.chars().map(|x| x.to_string()))
        }
        if self.include_lower_case {
            candidates.extend(lower_lang.chars().map(|x| x.to_string()))
        }
        if self.include_numeric {
            candidates.extend("0123456789".chars().map(|x| x.to_string()))
        }
        if self.include_special_char {
            candidates.extend(spec_val_str.chars().map(|x| x.to_string()))
        }
        candidates.shrink_to_fit();
        self.candidates = Some(candidates)
    }
}

impl BaseModel for RandomCharModel {
    fn get_mapping(&self) -> Option<&Mapping> {
        return None;
    }

    fn get_stats(&self) -> (usize, usize, Vec<(usize, usize)>) {
        if let Some(data) = &self.candidates {
            return (data.len(), data.capacity(), Vec::new());
        }
        (0, 0, Vec::new())
    }

    /// Every key exists, because we may replace it with every our char from candidates
    fn key_exists(&self, _: &str) -> bool {
        true
    }

    fn predict(&self, _: &str) -> Option<&Vec<String>> {
        self.candidates.as_ref()
    }
}

impl CharacterModel for RandomCharModel {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_not_load_model() {
        let model = RandomCharModel::new(true, true, true, true, "en", None);
        assert_eq!(model.predict(""), None);
    }

    #[test]
    fn test_load_full_en_model() {
        let mut model = RandomCharModel::new(true, true, true, true, "en", None);
        model.load_model();
        assert_ne!(model.predict(""), None);
        assert_eq!(model.predict("").unwrap().len(), 74);
    }

    #[test]
    fn test_load_chars_digits_ru_model() {
        let mut model = RandomCharModel::new(true, true, false, true, "ru", None);
        model.load_model();
        assert_ne!(model.predict(""), None);
        assert_eq!(model.predict("").unwrap().len(), 76);
    }

    #[test]
    fn test_load_chars_digits_only_model() {
        let mut model = RandomCharModel::new(true, true, false, true, "br-br", None);
        model.load_model();
        assert_ne!(model.predict(""), None);
        assert_eq!(model.predict("").unwrap().len(), 10);
    }

    #[test]
    fn test_custom_spec_chars() {
        let mut model =
            RandomCharModel::new(true, true, true, false, "br-br", Some(String::from("$%!")));
        model.load_model();
        assert_ne!(model.predict(""), None);
        assert_eq!(model.predict("").unwrap().len(), 3);
    }

    #[test]
    fn test_custom_candidates() {
        let mut model =
            RandomCharModel::from_candidates(vec![String::from("a"), String::from("q")]);
        model.load_model();
        assert_ne!(model.predict(""), None);
        assert_eq!(model.predict("").unwrap().len(), 2);
    }
}
