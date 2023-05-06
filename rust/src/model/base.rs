use std::collections::HashMap;
use std::path::Path;

pub type Mapping = HashMap<String, Vec<String>>;

pub trait Model {
    fn from_json(path: &Path) -> Self;
    fn predict(&self, data: &str) -> Option<&Vec<String>>;
}
