use regex::Regex;

use super::title_pattern_generator::get_title_pattern;

pub(crate) fn get_title_positions_matrix(file_contents: &str) -> Vec<Vec<usize>> {
    let mut title_positions_matrix: Vec<Vec<usize>> = vec![];
    let mut n: u32 = 0;
    loop {
        let title_positions = get_title_positions(file_contents, n);
        if title_positions.len() == 0 {
            break;
        } else {
            title_positions_matrix.push(title_positions);
            n += 1;
        }
    }
    title_positions_matrix
}

fn get_title_positions(file_contents: &str, title_depth: u32) -> Vec<usize> {
    let mut title_positions: Vec<usize> = vec![];
    for (line_number, line) in file_contents.lines().enumerate() {
        if has_title(line, title_depth) {
            title_positions.push(line_number);
        }
    }
    title_positions
}

fn has_title(line: &str, title_depth: u32) -> bool {
    let title_pattern = get_title_pattern(title_depth);
    let re = Regex::new(&title_pattern).unwrap();
    let has_title = re.is_match(line);
    has_title
}
