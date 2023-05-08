use std::collections::{HashMap, HashSet};

pub type Mapping = HashMap<String, Vec<String>>;

pub trait BaseModel {
    fn get_model(&self) -> Option<&Mapping>;

    /// Get model stats (len, capacity, Vec<len, capacity> for each array)
    fn get_stats(&self) -> (usize, usize, Vec<(usize, usize)>) {
        match self.get_model() {
            Some(model) => {
                let mut arr_stats = Vec::with_capacity(model.len());
                for (_, arr) in model {
                    arr_stats.push((arr.len(), arr.capacity()));
                }
                (model.len(), model.capacity(), arr_stats)
            }
            None => (0, 0, vec![]),
        }
    }

    fn key_exists(&self, data: &str) -> bool {
        if let Some(model) = self.get_model() {
            return model.contains_key(data);
        }
        false
    }

    fn predict(&self, data: &str) -> Option<&Vec<String>> {
        if let Some(model) = self.get_model() {
            return model.get(data);
        }
        None
    }

    fn deduplicate(mapping: Mapping) -> Mapping {
        let mut new_mapping = Mapping::with_capacity(mapping.capacity());
        for (key, value) in mapping {
            let mut buffer = HashSet::with_capacity(value.capacity());
            for elem in value {
                let _ = buffer.insert(elem);
            }
            new_mapping.insert(key, buffer.into_iter().collect());
        }
        new_mapping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockModel {
        model: Option<Mapping>,
    }
    impl BaseModel for MockModel {
        fn get_model(&self) -> Option<&Mapping> {
            if let Some(model) = &self.model {
                return Some(model);
            }
            return None;
        }
    }

    #[test]
    fn test_deduplicate() {
        let input_hash = HashMap::from([
            (
                String::from("A"),
                vec![String::from("a"), String::from("a"), String::from("c")],
            ),
            (String::from("B"), vec![String::from("b")]),
        ]);
        let deduplicated_hash = MockModel::deduplicate(input_hash);
        assert_eq!(deduplicated_hash.get("A").unwrap().len(), 2);
        assert_eq!(deduplicated_hash.get("B").unwrap().len(), 1);
    }

    #[test]
    fn test_get_non_model_stats() {
        let empty_model = MockModel { model: None };
        assert_eq!(empty_model.get_stats(), (0, 0, vec![]));
    }

    #[test]
    fn test_get_empty_model_stats() {
        let empty_model = MockModel {
            model: Some(HashMap::with_capacity(3)),
        };
        assert_eq!(empty_model.get_stats(), (0, 3, vec![]));
    }

    #[test]
    fn test_get_model_stats() {
        let mut hash = HashMap::with_capacity(3);
        hash.insert(
            String::from("A"),
            vec![String::from("a"), String::from("b")],
        );
        let model = MockModel { model: Some(hash) };
        assert_eq!(model.get_stats(), (1, 3, vec![(2, 2)]));
    }

    #[test]
    fn test_predict_non_model() {
        let empty_model = MockModel { model: None };
        assert_eq!(empty_model.predict("a"), None);
    }

    #[test]
    fn test_predict_empty_model() {
        let empty_model = MockModel {
            model: Some(HashMap::with_capacity(3)),
        };
        assert_eq!(empty_model.predict("a"), None);
    }

    #[test]
    fn test_predict_model() {
        let hash = HashMap::from([
            (
                String::from("A"),
                vec![String::from("a"), String::from("b")],
            ),
            (String::from("B"), vec![String::from("f")]),
        ]);
        let model = MockModel { model: Some(hash) };
        assert_eq!(
            *model.predict("A").unwrap(),
            vec![String::from("a"), String::from("b")]
        );
        assert_eq!(*model.predict("B").unwrap(), vec![String::from("f")]);
        assert_eq!(model.predict("C"), None);
    }
}
