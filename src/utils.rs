use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;

use serde_json::{Map, Value};

use super::model::base;

/// Read json from path and put in HashMap
/// Expected json format String -> Vec<String>
pub fn read_mapping(
    path: &Path,
    hashmap_init_capacity: Option<usize>,
    vec_value_init_capacity: Option<usize>,
) -> Result<base::Mapping, Box<dyn Error>> {
    let hashmap_init_capacity: usize = hashmap_init_capacity.unwrap_or(50);
    let vec_value_init_capacity: usize = vec_value_init_capacity.unwrap_or(5);

    let mut mapping: base::Mapping = HashMap::with_capacity(hashmap_init_capacity);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn read_good_mapping() {
        let good_mapping_path = Path::new("res/test/good_mapping.json");
        let readed_mapping: base::Mapping =
            read_mapping(&good_mapping_path, None, None).unwrap();

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
    fn read_wrong_val_mapping() {
        let wrong_val_mapping_path = Path::new("res/test/wrong_val_mapping.json");
        let readed_mapping: base::Mapping =
            read_mapping(&wrong_val_mapping_path, None, None).unwrap();
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
    fn read_not_json_mapping() {
        let not_json_path = Path::new("res/test/not_json.txt");
        let readed_result: Result<base::Mapping, Box<dyn Error>> =
            read_mapping(&not_json_path, None, None);
        let err = readed_result.unwrap_err().downcast::<serde_json::Error>().unwrap();
        assert!(err.is_syntax());
    }

    #[test]
    fn read_not_exist_mapping() {
        let not_exist_path = Path::new("res/test/not_exist.json");
        let readed_result: Result<base::Mapping, Box<dyn Error>> =
            read_mapping(&not_exist_path, None, None);
        let err = readed_result.unwrap_err().downcast::<io::Error>().unwrap();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
