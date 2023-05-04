use std::path::Path;

use fasttextaug;

fn main() {
    let file_path = Path::new("res/en.json");
    let result = fasttextaug::utils::read_mapping(file_path, None, None);
    println!("{:?}", result);
}
