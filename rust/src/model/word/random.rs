use super::super::{BaseModel, Mapping};
use super::WordModel;

/// Random word augmentations model
pub struct RandomWordModel {
    /// Vector, if we want to replace word with any other word from this vector
    candidates: Option<Vec<String>>,
    /// Mapping, if we want to replace specific word with specific candidates
    candidates_map: Option<Mapping>,
}

impl RandomWordModel {
    /// Initialization from Vector
    pub fn from_vec(candidates: Vec<String>) -> Self {
        Self {
            candidates: Some(candidates),
            candidates_map: None,
        }
    }
    /// Initialization from Mapping
    pub fn from_map(candidates_map: Mapping) -> Self {
        Self {
            candidates: None,
            candidates_map: Some(candidates_map),
        }
    }

    pub fn empty_model() -> Self {
        Self {
            candidates: None,
            candidates_map: None,
        }
    }

    pub fn load_model(&self) -> () {}
}

impl BaseModel for RandomWordModel {
    fn get_mapping(&self) -> Option<&Mapping> {
        return None;
    }

    fn get_stats(&self) -> (usize, usize, Vec<(usize, usize)>) {
        if let Some(data) = &self.candidates {
            return (data.len(), data.capacity(), Vec::new());
        }
        if let Some(data) = &self.candidates_map {
            let mut arr_stats = Vec::with_capacity(data.len());
            for (_, arr) in data {
                arr_stats.push((arr.len(), arr.capacity()));
            }
            return (data.len(), data.capacity(), arr_stats);
        }
        (0, 0, Vec::new())
    }

    fn key_exists(&self, data: &str) -> bool {
        match (&self.candidates, &self.candidates_map) {
            (None, Some(mapping)) => mapping.contains_key(data),
            (Some(_), None) => true,
            (Some(_), Some(_)) => true,
            (None, None) => false,
        }
    }

    fn predict(&self, data: &str) -> Option<&Vec<String>> {
        if let Some(_) = &self.candidates {
            return self.candidates.as_ref();
        }
        if let Some(mapping) = &self.candidates_map {
            return mapping.get(data);
        }
        None
    }
}

impl WordModel for RandomWordModel {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_vec_model() {
        let model = RandomWordModel::from_vec(vec![
            String::from("a"),
            String::from("b"),
            String::from("world"),
        ]);
        model.load_model();
        let expected = vec![String::from("a"), String::from("b"), String::from("world")];
        assert!(model.key_exists("world"));
        assert!(model.key_exists("key"));
        assert_eq!(model.predict("a"), Some(&expected));
        assert_eq!(model.predict("hello"), Some(&expected));
    }

    #[test]
    fn test_map_model() {
        let input_data = HashMap::from([
            (
                String::from("data"),
                vec![String::from("A"), String::from("Ok")],
            ),
            (String::from("key"), vec![String::from("value")]),
        ]);
        let model = RandomWordModel::from_map(input_data);
        model.load_model();

        let expected_one = vec![String::from("A"), String::from("Ok")];
        let expected_two = vec![String::from("value")];
        assert!(model.key_exists("key"));
        assert!(!model.key_exists("world"));
        assert_eq!(model.predict("data"), Some(&expected_one));
        assert_eq!(model.predict("key"), Some(&expected_two));
        assert_eq!(model.predict("hello"), None);
    }
}
