mod line_processor;
mod title_finder;
mod title_node;
mod title_pattern_generator;

pub fn sort_numbered_titles(file_contents: &str) -> String {
    // Extract lines and number of lines from file_contents
    let mut file_lines: Vec<String> = file_contents.lines().map(|x| String::from(x)).collect();
    let number_of_lines = file_lines.len();

    // Search for all title and subtitle positions
    let title_positions_matrix = title_finder::get_title_positions_matrix(file_contents);

    // Build a tree of title nodes
    let root_title_node = title_node::TitleNode::new(
        "".to_string(),
        0,
        (0, number_of_lines),
        &title_positions_matrix,
    );

    // Use the tree to correct all titles
    root_title_node.correct_children_lines(&mut file_lines);

    // Build the processed file contents
    let processed_file_contents = file_lines.join("\n");
    processed_file_contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file_contents = String::from("1.\n2.\n");
        sort_numbered_titles(&file_contents);
    }
}
