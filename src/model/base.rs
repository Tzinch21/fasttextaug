use std::path::Path;
use std::collections::HashMap;

pub type Mapping = HashMap<String, Vec<String>>;

pub trait Model {
    fn new(path: &Path) -> Self;
    fn predict(&self, data: &str) -> Option<&Vec<String>>;
}