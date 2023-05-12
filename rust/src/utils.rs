use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;

use serde_json::{Map, Value};

use crate::model::Mapping;

/// Read json from path and put in HashMap
/// Expected json format String -> Vec<String>
pub fn read_mapping(
    path: &Path,
    hashmap_init_capacity: Option<usize>,
    vec_value_init_capacity: Option<usize>,
) -> Result<Mapping, Box<dyn Error>> {
    let hashmap_init_capacity: usize = hashmap_init_capacity.unwrap_or(100);
    let vec_value_init_capacity: usize = vec_value_init_capacity.unwrap_or(10);

    let mut mapping: Mapping = HashMap::with_capacity(hashmap_init_capacity);
    let file_content = fs::read_to_string(path)?;
    let json_map: Map<String, Value> = serde_json::from_str(&file_content)?;

    for (key, raw_value) in &json_map {
        if let Value::Array(json_vec) = raw_value {
            let mut vec_to_insert: Vec<String> = Vec::with_capacity(vec_value_init_capacity);
            for json_value in json_vec {
                if let Value::String(s) = json_value {
                    vec_to_insert.push(s.to_string());
                }
            }
            if vec_to_insert.len() > 0 {
                mapping.insert(key.to_string(), vec_to_insert);
            }
        }
    }
    Ok(mapping)
}

pub type MappingSet = HashMap<String, HashSet<String>>;
/// Only purpose to check vec values not by order
pub fn transform_to_set(mapping: &Mapping) -> MappingSet {
    let mut new_val: MappingSet = MappingSet::with_capacity(mapping.len());
    for (key, val_arr) in mapping {
        new_val.insert(key.clone(), HashSet::from_iter(val_arr.iter().cloned()));
    }
    new_val
}

pub fn get_chars_len(input: &str) -> usize {
    let mut counter = 0;
    for _ in input.chars() {
        counter += 1
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn read_good_mapping() {
        let good_mapping_path = Path::new("test_res/good_mapping.json");
        let readed_mapping: Mapping = read_mapping(&good_mapping_path, None, None).unwrap();

        let expected_mapping = HashMap::from([
            (
                String::from("A"),
                vec![String::from("x"), String::from("y"), String::from("z")],
            ),
            (String::from("B"), vec![String::from("b")]),
        ]);
        assert_eq!(readed_mapping, expected_mapping);
    }

    #[test]
    fn test_read_wrong_val_mapping() {
        let wrong_val_mapping_path = Path::new("test_res/wrong_val_mapping.json");
        let readed_mapping: Mapping = read_mapping(&wrong_val_mapping_path, None, None).unwrap();
        let expected_mapping = HashMap::from([
            (
                String::from("A"),
                vec![String::from("x"), String::from("y")],
            ),
            (String::from("B"), vec![String::from("b")]),
        ]);
        assert_eq!(readed_mapping, expected_mapping);
    }

    #[test]
    fn test_read_not_json_mapping() {
        let not_json_path = Path::new("test_res/not_json.txt");
        let readed_result: Result<Mapping, Box<dyn Error>> =
            read_mapping(&not_json_path, None, None);
        let err = readed_result
            .unwrap_err()
            .downcast::<serde_json::Error>()
            .unwrap();
        assert!(err.is_syntax());
    }

    #[test]
    fn test_read_not_exist_mapping() {
        let not_exist_path = Path::new("test_res/not_exist.json");
        let readed_result: Result<Mapping, Box<dyn Error>> =
            read_mapping(&not_exist_path, None, None);
        let err = readed_result.unwrap_err().downcast::<io::Error>().unwrap();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
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
    fn test_english_char_counter() {
        let input = "Example";
        let chars_count = get_chars_len(input);
        assert_eq!(input.len(), 7);
        assert_eq!(chars_count, 7)
    }

    #[test]
    fn test_cyrillic_char_counter() {
        let input: &str = "Пример";
        let chars_count = get_chars_len(input);
        assert_eq!(input.len(), 12);
        assert_eq!(chars_count, 6)
    }

    #[test]
    fn test_mixed_char_counter() {
        let input: &str = "It's my пример";
        let chars_count = get_chars_len(input);
        assert_eq!(input.len(), 20);
        assert_eq!(chars_count, 14)
    }
}
