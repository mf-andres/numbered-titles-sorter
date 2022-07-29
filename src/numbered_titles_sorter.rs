mod line_processor;
mod subtitle_corrector;
mod title_corrector;
mod title_finder;
mod title_node;
mod title_pattern_generator;

pub fn sort_numbered_titles(file_contents: &str) -> String {
    // Extract lines and number of lines from file_contents
    let mut file_lines: Vec<String> = file_contents.lines().map(|x| String::from(x)).collect();
    let number_of_lines = file_lines.len();

    // Search for all title and subtitle positions
    let title_positions_matrix = title_finder::get_title_positions_matrix(file_contents);
    let root_title_node =
        title_node::TitleNode::new(0, (0, number_of_lines), &title_positions_matrix);

    //TODO change to tree driven solution
    let title_positions: Vec<usize> = title_positions_matrix[0].clone();
    let subtitle_positions: Vec<usize> = title_positions_matrix[1].clone();

    //depth 1
    title_corrector::correct_titles(&mut file_lines, &title_positions, title_positions.len());

    // depth 2
    let processed_file_contents = subtitle_corrector::correct_subtitles(
        title_positions,
        number_of_lines,
        subtitle_positions,
        file_lines,
    );
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
