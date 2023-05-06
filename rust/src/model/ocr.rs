use super::super::utils;
use super::base::{Mapping, Model};
use std::path::Path;

pub struct OcrModel {
    model_path: String,
    model: Option<Mapping>,
}

impl OcrModel {
    /// This function useful because if 'l' looks like '1', so '1' looks like 'l'
    fn reverse_mapping(mapping: &mut Mapping) {
        let mut pairs_to_insert: Vec<(String, String)> = Vec::with_capacity(mapping.capacity() * 2);
        for (key, vec_value) in mapping.iter() {
            for value in vec_value {
                let is_in_hash = mapping.get(value);
                match is_in_hash {
                    None => {
                        pairs_to_insert.push((value.clone(), key.clone()));
                    }
                    Some(vec_not_key) => {
                        if !vec_not_key.contains(key) {
                            pairs_to_insert.push((value.clone(), key.clone()));
                        }
                    }
                }
            }
        }

        for (new_key, new_value) in pairs_to_insert {
            if let Some(vec_of_val) = mapping.get_mut(&new_key) {
                vec_of_val.push(new_value)
            } else {
                let mut new_vec_of_val: Vec<String> = Vec::with_capacity(10);
                new_vec_of_val.push(new_value);
                mapping.insert(new_key.clone(), new_vec_of_val);
            }
        }
    }

    pub fn new(model_path: String) -> Self {
        let model = Self {
            model_path,
            model: None,
        };
        model
    }

    pub fn new_from_mapping(mut mapping: Mapping) -> Self {
        Self::reverse_mapping(&mut mapping);
        let deduplicated_mapping = Self::deduplicate(mapping);
        OcrModel {
            model_path: String::from("internal mapping"),
            model: Some(deduplicated_mapping),
        }
    }

    pub fn load_model(&mut self) {
        if let Some(_) = self.model {
            return;
        }
        let model_path = Path::new(&self.model_path);
        let mut mapping_from_file = utils::read_mapping(model_path, Some(100), Some(10)).unwrap();
        Self::reverse_mapping(&mut mapping_from_file);
        self.model = Some(Self::deduplicate(mapping_from_file));
    }
}

impl Model for OcrModel {
    fn get_model(&self) -> Option<&Mapping> {
        if let Some(model) = &self.model {
            return Some(model);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::utils::{transform_to_set, MappingSet};
    use super::*;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_empty_transform_to_set() {
        let input_mapping: HashMap<String, Vec<String>> = HashMap::new();
        let mapping_set: MappingSet = transform_to_set(&input_mapping);
        let expected_result: MappingSet = HashMap::new();
        assert_eq!(mapping_set, expected_result);
    }

    #[test]
    fn test_reverse() {
        let mut hash: Mapping = HashMap::from([
            (
                String::from("A"),
                vec![String::from("x"), String::from("y")],
            ),
            (String::from("B"), vec![String::from("b")]),
            (
                String::from("C"),
                vec![String::from("x"), String::from("x")],
            ),
            (String::from("D"), vec![String::from("d")]),
            (String::from("d"), vec![String::from("D")]),
        ]);
        let expected_result = HashMap::from([
            (
                String::from("A"),
                HashSet::from([String::from("x"), String::from("y")]),
            ),
            (String::from("B"), HashSet::from([String::from("b")])),
            (String::from("C"), HashSet::from([String::from("x")])),
            (
                String::from("x"),
                HashSet::from([String::from("C"), String::from("A")]),
            ),
            (String::from("y"), HashSet::from([String::from("A")])),
            (String::from("b"), HashSet::from([String::from("B")])),
            (String::from("D"), HashSet::from([String::from("d")])),
            (String::from("d"), HashSet::from([String::from("D")])),
        ]);
        OcrModel::reverse_mapping(&mut hash);
        assert_eq!(hash.get("x").unwrap().len(), 3);

        let mapping_set = transform_to_set(&hash);
        assert_eq!(mapping_set, expected_result);
        let hash = OcrModel::deduplicate(hash);
        assert_eq!(hash.get("x").unwrap().len(), 2);
        assert_eq!(hash.get("y").unwrap().len(), 1);
    }

    #[test]
    fn test_reverse_empty() {
        let mut hash = HashMap::new();
        let expected_result = HashMap::new();
        OcrModel::reverse_mapping(&mut hash);
        assert_eq!(hash, expected_result);
    }

    #[test]
    fn test_create_from_mapping() {
        let mapping = HashMap::from([
            (
                String::from("A"),
                vec![String::from("a"), String::from("a")],
            ),
            (
                String::from("B"),
                vec![String::from("b"), String::from("f")],
            ),
            (String::from("C"), vec![String::from("b")]),
        ]);
        let mut ocr_model = OcrModel::new_from_mapping(mapping);
        let expected = HashMap::from([
            (String::from("A"), vec![String::from("a")]),
            (
                String::from("B"),
                vec![String::from("b"), String::from("f")],
            ),
            (String::from("C"), vec![String::from("b")]),
            (String::from("a"), vec![String::from("A")]),
            (
                String::from("b"),
                vec![String::from("B"), String::from("C")],
            ),
            (String::from("f"), vec![String::from("B")]),
        ]);
        let obs_set = transform_to_set(&ocr_model.get_model().unwrap());
        let exp_set = transform_to_set(&expected);
        assert_eq!(obs_set, exp_set);
        assert_eq!(ocr_model.model_path, String::from("internal mapping"));
        ocr_model.load_model();
        assert_eq!(obs_set, exp_set);
        assert_eq!(ocr_model.model_path, String::from("internal mapping"));
    }

    #[test]
    fn test_load_model() {
        let mut ocr = OcrModel::new(String::from("test_res/small_mapping.json"));
        assert_eq!(ocr.get_model(), None);

        ocr.load_model();
        let exp_hash = HashMap::from([
            (
                String::from("A"),
                vec![String::from("a"), String::from("b")],
            ),
            (String::from("B"), vec![String::from("f")]),
            (String::from("a"), vec![String::from("A")]),
            (String::from("b"), vec![String::from("A")]),
            (String::from("f"), vec![String::from("B")]),
        ]);
        let obs_set = transform_to_set(ocr.get_model().unwrap());
        assert_eq!(obs_set, transform_to_set(&exp_hash));

        ocr.load_model();
        assert_eq!(obs_set, transform_to_set(&exp_hash));
    }
}
