use regex::Regex;

use super::title_pattern_generator;

pub(crate) fn process_line(line: &String, title_depth: u32, title_number: &String) -> String {
    let title_pattern = title_pattern_generator::get_title_pattern(title_depth);
    println!("title_pattern {}", &title_pattern);
    let re = Regex::new(&title_pattern).unwrap();
    let processed_line = re.replace(&line, title_number).to_string();
    processed_line
}

pub(crate) fn title_from(title_numbers: Vec<usize>) -> String {
    let mut title = String::from("");
    for title_number in title_numbers {
        let title_fragment = format!("{}.", title_number.to_string());
        title.push_str(&title_fragment);
    }
    title.push_str(" ");
    title
}
