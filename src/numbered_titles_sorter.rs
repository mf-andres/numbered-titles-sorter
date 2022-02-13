use core::num;
use std::{borrow::Borrow, ops::Add, iter::{repeat}};
use itertools::{izip, Itertools};


use regex::Regex;

pub fn sort_numbered_titles(file_contents: &str) -> String {
    //depth 1
    let title_positions: Vec<usize> = get_title_positions(file_contents);
    let number_of_titles: usize = title_positions.len();
    let mut processed_file_contents = correct_titles(file_contents, number_of_titles);

    // depth 2
    let subtitle_positions: Vec<usize> = get_subtitle_positions(file_contents);
    let next_title_positions: Vec<usize> = title_positions[1..title_positions.len()].to_vec();
    let next_title_positions: Vec<usize> = [next_title_positions, [file_contents.len()].to_vec()].concat();

    let subtitles_groups = izip!(title_positions, next_title_positions, repeat(subtitle_positions)).map(|x| create_subtitle_group(x)).collect_vec();

    let mut number_of_subtitles_per_titles: Vec<usize> = vec![]; 
    for i in 0..number_of_titles {
        number_of_subtitles_per_titles.push(subtitles_groups[i].len());
    }

    //iterate titles and subtitles and replace
    println!("{:?}", subtitles_groups);
    println!("{:?}", number_of_subtitles_per_titles);
    println!("{}", processed_file_contents);
    processed_file_contents
} 

fn get_title_positions(file_contents: &str) -> Vec<usize>{
    let mut title_positions: Vec<usize> = vec![];
    for (line_number, line) in file_contents.lines().enumerate() {
        if has_title(line) {
            title_positions.push(line_number);
        }
    }
    title_positions
}

fn has_title(line: &str) -> bool {
    let re = Regex::new(r"^\d\. ").unwrap();
    let has_title = re.is_match(line);
    has_title
}

fn process_line(line: &str, line_number: usize) -> String {
    let new_title = format!("{} ", line_number.to_string());
    let re = Regex::new(r"^\d\. ").unwrap();
    let processed_line = re.replace(line, new_title).to_string();
    processed_line
}

fn get_title_pattern(depth: u64) -> String {
    let base_pattern: &str  = r"^\d\. ";
    let mut title_pattern: String = String::from("");
    for _ in 0..depth {
        title_pattern.push_str(base_pattern);
    }
    title_pattern
}

fn correct_titles(file_contents: &str, number_of_titles: usize) -> String {
    let mut processed_file_contents = String::from("");
    let mut title_number_range = 1..number_of_titles+1;
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
    processed_file_contents
}

fn get_subtitle_positions(file_contents: &str) -> Vec<usize>{
    let mut subtitle_positions: Vec<usize> = vec![];
    for (line_number, line) in file_contents.lines().enumerate() {
        if has_subtitle(line) {
            subtitle_positions.push(line_number);
        }
    }
    subtitle_positions
}

fn has_subtitle(line: &str) -> bool {
    let re = Regex::new(r"^\d\.\d\. ").unwrap();
    let has_subtitle = re.is_match(line);
    has_subtitle
}

fn create_subtitle_group(_tuple: (usize, usize, Vec<usize>)) -> Vec<usize> {
    let title_position: usize = _tuple.0; 
    let other_title_position: usize = _tuple.1; 
    let subtitle_positions: Vec<usize> = _tuple.2;
    let subtitle_group = izip!(repeat(title_position), repeat(other_title_position), subtitle_positions).filter(|x| is_subtitle_between_titles(x));
    let subtitle_group = subtitle_group.map(|x| x.2).collect_vec(); 
    subtitle_group
}

fn is_subtitle_between_titles(_tuple: &(usize, usize, usize)) -> bool {
    let title_position = _tuple.0;
    let other_title_position = _tuple.1;
    let subtitle_position = _tuple.2;
    if title_position < subtitle_position && subtitle_position < other_title_position
    {
        return true;
    }
    else {
        return false;
    }
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