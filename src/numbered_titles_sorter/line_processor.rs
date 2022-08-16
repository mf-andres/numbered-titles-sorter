use regex::Regex;

use super::title_pattern_generator;

pub(crate) fn process_line(line: &String, title_depth: u32, title_number: &String) -> String {
    let title_number = title_number.to_owned() + " ";
    let title_pattern = title_pattern_generator::get_title_pattern(title_depth);
    let re = Regex::new(&title_pattern).unwrap();
    let processed_line = re.replace(&line, title_number).to_string();
    processed_line
}
