use regex::Regex;

use super::title_pattern_generator::get_title_pattern;

pub(crate) fn get_title_positions(file_contents: &str, title_depth: u64) -> Vec<usize> {
    let mut title_positions: Vec<usize> = vec![];
    for (line_number, line) in file_contents.lines().enumerate() {
        if has_title(line, title_depth) {
            title_positions.push(line_number);
        }
    }
    title_positions
}

fn has_title(line: &str, title_depth: u64) -> bool {
    let title_pattern = get_title_pattern(title_depth);
    let re = Regex::new(&title_pattern).unwrap();
    let has_title = re.is_match(line);
    has_title
}
