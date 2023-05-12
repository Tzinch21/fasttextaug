use std::path::Path;

use super::super::{BaseModel, Mapping};
use super::CharacterModel;
use crate::utils;

pub struct KeyboardModel {
    allow_special_char: bool,
    allow_numeric: bool,
    upper_case: bool,
    model_path: String,
    model: Option<Mapping>,
}

impl KeyboardModel {
    /// Check for conditions, if char met them, then include char to mapping model
    /// if spec_char not allowed and it's special char (not alphanumeric) -> return false
    /// if numeric not allowd and it's numeric -> return false
    /// if any condition is false -> return false
    fn check_conditions(&self, input: &str) -> bool {
        let character = input.chars().next().unwrap();
        let spec_char_cond_met = self.allow_special_char | character.is_alphanumeric();
        let num_cond_met = self.allow_numeric | !character.is_numeric();
        spec_char_cond_met & num_cond_met
    }

    pub fn get_allow_special_char(&self) -> bool {
        self.allow_special_char
    }

    pub fn new(
        allow_special_char: bool,
        allow_numeric: bool,
        upper_case: bool,
        model_path: String,
    ) -> Self {
        let model = Self {
            allow_special_char,
            allow_numeric,
            upper_case,
            model_path,
            model: None,
        };
        model
    }

    pub fn load_model(&mut self) {
        if let Some(_) = self.model {
            return;
        }

        let model_path = Path::new(&self.model_path);
        let mapping_from_file = utils::read_mapping(model_path, Some(100), Some(15)).unwrap();
        let mut keyboard_mapping = Mapping::with_capacity(mapping_from_file.capacity());

        for (key, arr) in mapping_from_file.into_iter() {
            if self.check_conditions(&key) {
                let mut arr_to_key = Vec::with_capacity(arr.capacity() * 2);
                let mut arr_to_caps_key = Vec::with_capacity(arr.capacity() * 2);
                let upper_diff = self.upper_case & (key != key.to_uppercase());
                for value in arr {
                    if self.check_conditions(&value) {
                        if upper_diff {
                            arr_to_caps_key.push(value.to_uppercase());
                            arr_to_caps_key.push(value.clone());
                        }
                        if self.upper_case {
                            arr_to_key.push(value.to_uppercase());
                        }
                        arr_to_key.push(value);
                    }
                }
                if arr_to_caps_key.len() > 0 {
                    keyboard_mapping.insert(key.to_uppercase(), arr_to_caps_key);
                }
                if arr_to_key.len() > 0 {
                    keyboard_mapping.insert(key, arr_to_key);
                }
            }
        }
        self.model = Some(Self::deduplicate(keyboard_mapping));
    }
}

impl BaseModel for KeyboardModel {
    fn get_mapping(&self) -> Option<&Mapping> {
        if let Some(model) = &self.model {
            return Some(model);
        }
        return None;
    }
}

impl CharacterModel for KeyboardModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::transform_to_set;
    use std::collections::HashMap;

    #[test]
    fn test_check_conditions() {
        let allow_all = KeyboardModel {
            allow_special_char: true,
            allow_numeric: true,
            upper_case: true,
            model_path: String::from(""),
            model: None,
        };
        assert!(allow_all.check_conditions("й"));
        assert!(allow_all.check_conditions("7"));
        assert!(allow_all.check_conditions("!"));

        let forbid_special_chars = KeyboardModel {
            allow_special_char: false,
            allow_numeric: true,
            upper_case: true,
            model_path: String::from(""),
            model: None,
        };
        assert!(forbid_special_chars.check_conditions("L"));
        assert!(forbid_special_chars.check_conditions("4"));
        assert!(!forbid_special_chars.check_conditions("%"));

        let forbid_digits = KeyboardModel {
            allow_special_char: true,
            allow_numeric: false,
            upper_case: true,
            model_path: String::from(""),
            model: None,
        };
        assert!(forbid_digits.check_conditions("f"));
        assert!(!forbid_digits.check_conditions("4"));
        assert!(forbid_digits.check_conditions("$"));

        let forbid_all = KeyboardModel {
            allow_special_char: false,
            allow_numeric: false,
            upper_case: true,
            model_path: String::from(""),
            model: None,
        };
        assert!(forbid_all.check_conditions("Б"));
        assert!(!forbid_all.check_conditions("9"));
        assert!(!forbid_all.check_conditions("@"));
    }

    #[test]
    fn test_load_model_caps_allow_all() {
        let mut allow_all_caps = KeyboardModel {
            allow_special_char: true,
            allow_numeric: true,
            upper_case: true,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        allow_all_caps.load_model();
        let allow_all_caps_set = transform_to_set(allow_all_caps.get_mapping().unwrap());
        let expected_allow_all_caps = HashMap::from([
            (
                String::from("а"),
                vec![
                    String::from("1"),
                    String::from("$"),
                    String::from("б"),
                    String::from("Б"),
                ],
            ),
            (
                String::from("А"),
                vec![
                    String::from("1"),
                    String::from("$"),
                    String::from("б"),
                    String::from("Б"),
                ],
            ),
            (
                String::from("2"),
                vec![
                    String::from("8"),
                    String::from("@"),
                    String::from("й"),
                    String::from("Й"),
                ],
            ),
            (
                String::from("%"),
                vec![
                    String::from("м"),
                    String::from("М"),
                    String::from("0"),
                    String::from("!"),
                ],
            ),
        ]);
        assert_eq!(
            allow_all_caps_set,
            transform_to_set(&expected_allow_all_caps)
        );
    }

    #[test]
    fn test_load_model_allow_all() {
        let mut allow_all = KeyboardModel {
            allow_special_char: true,
            allow_numeric: true,
            upper_case: false,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        allow_all.load_model();
        let allow_all_set = transform_to_set(allow_all.get_mapping().unwrap());
        let expected_allow_all = HashMap::from([
            (
                String::from("а"),
                vec![String::from("1"), String::from("$"), String::from("б")],
            ),
            (
                String::from("2"),
                vec![String::from("8"), String::from("@"), String::from("й")],
            ),
            (
                String::from("%"),
                vec![String::from("м"), String::from("0"), String::from("!")],
            ),
        ]);
        assert_eq!(allow_all_set, transform_to_set(&expected_allow_all));
    }

    #[test]
    fn test_load_model_caps_forbid_spec() {
        let mut forbid_spec_caps = KeyboardModel {
            allow_special_char: false,
            allow_numeric: true,
            upper_case: true,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        forbid_spec_caps.load_model();
        let forbid_spec_caps_set = transform_to_set(forbid_spec_caps.get_mapping().unwrap());
        let expected_forbid_spec_caps = HashMap::from([
            (
                String::from("а"),
                vec![String::from("1"), String::from("б"), String::from("Б")],
            ),
            (
                String::from("А"),
                vec![String::from("1"), String::from("б"), String::from("Б")],
            ),
            (
                String::from("2"),
                vec![String::from("8"), String::from("й"), String::from("Й")],
            ),
        ]);
        assert_eq!(
            forbid_spec_caps_set,
            transform_to_set(&expected_forbid_spec_caps)
        );
    }

    #[test]
    fn test_load_model_forbid_spec() {
        let mut forbid_spec = KeyboardModel {
            allow_special_char: false,
            allow_numeric: true,
            upper_case: false,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        forbid_spec.load_model();
        let forbid_spec_set = transform_to_set(forbid_spec.get_mapping().unwrap());
        let expected_forbid_spec = HashMap::from([
            (
                String::from("а"),
                vec![String::from("1"), String::from("б")],
            ),
            (
                String::from("2"),
                vec![String::from("8"), String::from("й")],
            ),
        ]);
        assert_eq!(forbid_spec_set, transform_to_set(&expected_forbid_spec));
    }

    #[test]
    fn test_load_model_caps_forbid_num() {
        let mut forbid_num_caps = KeyboardModel {
            allow_special_char: true,
            allow_numeric: false,
            upper_case: true,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        forbid_num_caps.load_model();
        let forbid_num_caps_set = transform_to_set(forbid_num_caps.get_mapping().unwrap());
        let expected_forbid_num_caps = HashMap::from([
            (
                String::from("а"),
                vec![String::from("$"), String::from("б"), String::from("Б")],
            ),
            (
                String::from("А"),
                vec![String::from("$"), String::from("б"), String::from("Б")],
            ),
            (
                String::from("%"),
                vec![String::from("м"), String::from("М"), String::from("!")],
            ),
        ]);
        assert_eq!(
            forbid_num_caps_set,
            transform_to_set(&expected_forbid_num_caps)
        );
    }

    #[test]
    fn test_load_model_forbid_num() {
        let mut forbid_num = KeyboardModel {
            allow_special_char: true,
            allow_numeric: false,
            upper_case: false,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        forbid_num.load_model();
        let forbid_num_set = transform_to_set(forbid_num.get_mapping().unwrap());
        let expected_forbid_num = HashMap::from([
            (
                String::from("а"),
                vec![String::from("$"), String::from("б")],
            ),
            (
                String::from("%"),
                vec![String::from("м"), String::from("!")],
            ),
        ]);
        assert_eq!(forbid_num_set, transform_to_set(&expected_forbid_num));
    }

    #[test]
    fn test_load_model_caps_forbid_all() {
        let mut forbid_all_caps = KeyboardModel {
            allow_special_char: false,
            allow_numeric: false,
            upper_case: true,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        forbid_all_caps.load_model();
        let forbid_all_caps_set = transform_to_set(forbid_all_caps.get_mapping().unwrap());
        let expected_forbid_all_caps = HashMap::from([
            (
                String::from("а"),
                vec![String::from("б"), String::from("Б")],
            ),
            (
                String::from("А"),
                vec![String::from("б"), String::from("Б")],
            ),
        ]);
        assert_eq!(
            forbid_all_caps_set,
            transform_to_set(&expected_forbid_all_caps)
        );
    }

    #[test]
    fn test_load_model_forbid_all() {
        let mut forbid_all = KeyboardModel {
            allow_special_char: false,
            allow_numeric: false,
            upper_case: false,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        forbid_all.load_model();
        let forbid_all_set = transform_to_set(forbid_all.get_mapping().unwrap());
        let expected_forbid_all = HashMap::from([(String::from("а"), vec![String::from("б")])]);
        assert_eq!(forbid_all_set, transform_to_set(&expected_forbid_all));
    }

    #[test]
    fn test_load_model() {
        let mut key_model = KeyboardModel {
            allow_special_char: false,
            allow_numeric: false,
            upper_case: false,
            model_path: String::from("test_res/small_keyboard.json"),
            model: None,
        };
        assert_eq!(key_model.get_mapping(), None);
        key_model.load_model();
        let expected_all = HashMap::from([(String::from("а"), vec![String::from("б")])]);
        assert_eq!(*key_model.get_mapping().unwrap(), expected_all);
        key_model.load_model();
        assert_eq!(*key_model.get_mapping().unwrap(), expected_all);
    }
}
