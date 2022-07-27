use std::fs;
use std::path;

pub fn read_file(source_file_path: &str) -> String {
    let file_path = path::Path::new(source_file_path);
    let contents = fs::read_to_string(file_path).expect("Error while reading file");
    contents
}
