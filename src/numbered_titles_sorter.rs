use core::num;
use std::borrow::Borrow;

use regex::Regex;

pub fn sort_numbered_titles(file_contents: &str) -> String {
    let number_of_lines_with_titles = get_number_of_titles(&file_contents);
    let mut processed_file_contents = String::from("");
    let mut title_number_range = 1..number_of_lines_with_titles+1;
    for line in file_contents.lines() {
        let processed_line;
        if has_title(line) {
            let title_number = title_number_range.next().unwrap();
            processed_line = process_line(line, title_number);
        }
        else {
            processed_line = line.to_string();
        }
        processed_file_contents = format!("{}{}\n", processed_file_contents, processed_line);
    }
    println!("{}", processed_file_contents);
    processed_file_contents
} 

fn get_number_of_titles(file_contents: &str) -> u64 {
    let mut number_of_lines_with_titles: u64 = 0;
    for line in file_contents.lines() {
        if has_title(line) {
            number_of_lines_with_titles += 1;
        }
    }
    number_of_lines_with_titles
}

fn has_title(line: &str) -> bool {
    let re = Regex::new(r"^\d\. ").unwrap();
    let has_title = re.is_match(line);
    has_title
}

fn process_line(line: &str, line_number: u64) -> String {
    let new_title = format!("{} ", line_number.to_string());
    let re = Regex::new(r"^\d\. ").unwrap();
    let processed_line = re.replace(line, new_title).to_string();
    processed_line
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