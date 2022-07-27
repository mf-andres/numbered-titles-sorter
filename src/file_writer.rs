use std::{fs, path};

// process output
// write lines
pub fn write_file(processed_file_contents: &str, source_file_path: &str) {
    let source_file_path = path::Path::new(source_file_path);
    let source_file_path_parent = source_file_path.parent();
    let source_file_path_parent =
        source_file_path_parent.expect("Error while retrieving source file metadata");
    let source_file_name = source_file_path.file_name();
    let source_file_name = source_file_name.expect("Error while retrieving source file metadata");
    let source_file_name = source_file_name.to_str();
    let source_file_name = source_file_name.expect("Error while retrieving source file metadata");
    let processed_file_name = format!("processed_{}", source_file_name);
    let target_file_path = source_file_path_parent.join(processed_file_name);
    fs::write(target_file_path, processed_file_contents).expect("Unable to write file");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
