use std::collections::HashSet;

use super::super::utils;
use super::base::{Mapping, Model};

pub struct OcrModel {
    pub model: Mapping,
}

impl OcrModel {
    /// This function useful because if 'l' looks like '1', so '1' looks like 'l'
    fn reverse_mapping(mapping: &mut Mapping) {
        let mut pairs_to_insert: HashSet<(String, String)> = HashSet::with_capacity(200);
        for (key, vec_value) in mapping.iter() {
            for value in vec_value {
                let is_in_hash = mapping.get(value);
                match is_in_hash {
                    None => {
                        pairs_to_insert.insert((value.clone(), key.clone()));
                    }
                    Some(vec_not_key) => {
                        if !vec_not_key.contains(key) {
                            pairs_to_insert.insert((value.clone(), key.clone()));
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
}

impl Model for OcrModel {
    fn from_json(model_path: &std::path::Path) -> Self {
        let mut mapping_from_file = utils::read_mapping(model_path, Some(100), Some(10)).unwrap();
        Self::reverse_mapping(&mut mapping_from_file);
        OcrModel {
            model: mapping_from_file,
        }
    }

    fn predict(&self, data: &str) -> Option<&Vec<String>> {
        self.model.get(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Only purpose to check vec values not by order
    type MappingSet = HashMap<String, HashSet<String>>;
    fn transform_to_set(mapping: &Mapping) -> MappingSet {
        let mut new_val: MappingSet = MappingSet::with_capacity(mapping.len());
        for (key, val_arr) in mapping {
            new_val.insert(key.clone(), HashSet::from_iter(val_arr.iter().cloned()));
        }
        new_val
    }

    #[test]
    fn test_transform_to_set() {
        let input_mapping = HashMap::from([
            (
                String::from("A"),
                vec![String::from("a"), String::from("a"), String::from("c")],
            ),
            (String::from("B"), vec![String::from("b")]),
            (
                String::from("C"),
                vec![String::from("c"), String::from("s")],
            ),
        ]);
        let mapping_set = transform_to_set(&input_mapping);
        let expected_result = HashMap::from([
            (
                String::from("A"),
                HashSet::from([String::from("a"), String::from("c")]),
            ),
            (String::from("B"), HashSet::from([String::from("b")])),
            (
                String::from("C"),
                HashSet::from([String::from("c"), String::from("s")]),
            ),
        ]);
        assert_eq!(mapping_set, expected_result)
    }

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
        let mapping_set = transform_to_set(&hash);
        assert_eq!(mapping_set, expected_result);
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
}
