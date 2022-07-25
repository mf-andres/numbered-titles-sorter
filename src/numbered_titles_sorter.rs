use itertools::izip;

use regex::Regex;

pub fn sort_numbered_titles(file_contents: &str) -> String {
    // Extract lines and number of lines from file_contents
    let mut file_lines: Vec<String> = file_contents.lines().map(|x| String::from(x)).collect();
    let number_of_lines = file_lines.len();

    // Search for all title and subtitle positions
    let title_positions: Vec<usize> = get_title_positions(file_contents, 0);
    let subtitle_positions: Vec<usize> = get_title_positions(file_contents, 1);

    //depth 1
    let number_of_titles: usize = title_positions.len();
    correct_titles(&mut file_lines, &title_positions, number_of_titles);

    // depth 2
    let mut title_numbers_range = 1..number_of_titles + 1;
    let next_title_positions = get_next_title_positions(&title_positions, number_of_lines);
    for (previous_title_position, next_title_position) in
        izip!(title_positions, next_title_positions)
    {
        let title_number = title_numbers_range.next().unwrap();
        let subtitle_positions_between_titles: Vec<usize> = get_subtitle_positions_between_titles(
            &subtitle_positions,
            previous_title_position,
            next_title_position,
        );
        let number_of_subtitles_between_titles: usize = subtitle_positions_between_titles.len();
        correct_subtitles_between_titles(
            &mut file_lines,
            title_number,
            &subtitle_positions_between_titles,
            number_of_subtitles_between_titles,
        );
    }
    let processed_file_contents = file_lines.join("\n");
    processed_file_contents
}

fn get_title_positions(file_contents: &str, title_depth: u64) -> Vec<usize> {
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

fn get_title_pattern(title_depth: u64) -> String {
    let base_pattern: &str = r"^\d\.";
    let pattern_extension: &str = r"\d\.";
    let mut title_pattern: String = String::from(base_pattern);
    for _ in 1..title_depth + 1 {
        title_pattern.push_str(pattern_extension);
    }
    title_pattern.push_str(" ");
    title_pattern
}

fn correct_titles(
    file_lines: &mut Vec<String>,
    title_positions: &Vec<usize>,
    number_of_titles: usize,
) {
    let mut title_number_range = 1..number_of_titles + 1;
    for title_position in title_positions {
        let title_number = title_number_range.next().unwrap();
        let line = &file_lines[*title_position];
        let processed_line = process_line(line, [title_number].to_vec());
        file_lines[*title_position] = processed_line;
    }
}

fn process_line(line: &String, title_numbers: Vec<usize>) -> String {
    let title_depth: u64 = title_numbers.len() as u64 - 1;
    let new_title = title_from(title_numbers);
    let title_pattern = get_title_pattern(title_depth);
    let re = Regex::new(&title_pattern).unwrap();
    let processed_line = re.replace(&line, new_title).to_string();
    processed_line
}

fn title_from(title_numbers: Vec<usize>) -> String {
    let mut title = String::from("");
    for title_number in title_numbers {
        let title_fragment = format!("{}.", title_number.to_string());
        title.push_str(&title_fragment);
    }
    title.push_str(" ");
    title
}

fn get_next_title_positions(title_positions: &Vec<usize>, number_of_lines: usize) -> Vec<usize> {
    let next_title_positions: Vec<usize> = title_positions[1..title_positions.len()].to_vec();
    let next_title_positions: Vec<usize> =
        [next_title_positions, [number_of_lines].to_vec()].concat();
    next_title_positions
}

fn get_subtitle_positions_between_titles(
    subtitle_positions: &Vec<usize>,
    previous_title_position: usize,
    next_title_position: usize,
) -> Vec<usize> {
    let subtitle_positions_between_titles: Vec<usize> = subtitle_positions
        .iter()
        .filter(|x| previous_title_position < **x && **x < next_title_position)
        .map(|x| *x)
        .collect();
    subtitle_positions_between_titles
}

fn correct_subtitles_between_titles(
    file_lines: &mut Vec<String>,
    title_number: usize,
    subtitle_positions_between_titles: &Vec<usize>,
    number_of_subtitles_between_titles: usize,
) {
    let mut subtitle_numbers_range = 1..number_of_subtitles_between_titles + 1;
    for subtitle_position in subtitle_positions_between_titles {
        let subtitle_number = subtitle_numbers_range.next().unwrap();
        let line = &file_lines[*subtitle_position];
        let processed_line = process_line(line, [title_number, subtitle_number].to_vec());
        file_lines[*subtitle_position] = processed_line;
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
