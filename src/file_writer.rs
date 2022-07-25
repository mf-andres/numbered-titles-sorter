use std::{fs, path};

// process output
// write lines
pub fn write_file(processed_file_contents: &str) {
    let file_path = path::Path::new("./example_documents/processed_example_1.txt");
    fs::write(file_path, processed_file_contents).expect("Unable to write file");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
