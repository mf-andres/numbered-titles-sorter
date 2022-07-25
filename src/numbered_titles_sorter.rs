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
        )
    }
    let processed_file_contents = String::from("");
    processed_file_contents
}

fn correct_subtitles_between_titles(
    file_lines: &mut Vec<String>,
    title_number: usize,
    subtitle_positions_between_titles: &Vec<usize>,
    number_of_subtitles_between_titles: usize,
) -> String {
    let mut processed_file_contents = String::from("");
    let mut subtitle_numbers_range = 1..number_of_subtitles_between_titles + 1;
    let lines_enumeration = file_contents.lines().enumerate();
    for (line_number, line) in lines_enumeration {
        let processed_line;
        if subtitle_positions_between_titles.contains(&line_number) {
            let subtitle_number = subtitle_numbers_range.next().unwrap();
            processed_line = process_subtitle_line(line, title_number, subtitle_number);
        } else {
            processed_line = line.to_string();
        }
        processed_file_contents = format!("{}{}\n", processed_file_contents, processed_line);
    }
    processed_file_contents
}

fn process_subtitle_line(line: &str, title_number: usize, subtitle_number: usize) -> String {
    // TODO we are here
    let new_subtitle = format!(
        "{}.{}. ",
        title_number.to_string(),
        subtitle_number.to_string()
    );
    let re = Regex::new(r"^\d\.\d\. ").unwrap();
    let processed_line = re.replace(line, new_subtitle).to_string();
    processed_line
}

fn get_next_title_positions(title_positions: &Vec<usize>, number_of_lines: usize) -> Vec<usize> {
    let next_title_positions: Vec<usize> = title_positions[1..title_positions.len()].to_vec();
    let next_title_positions: Vec<usize> =
        [next_title_positions, [number_of_lines].to_vec()].concat();
    next_title_positions
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
        let processed_line = process_line(line, title_number);
        file_lines[*title_position] = processed_line;
    }
}

// TODO generalize with depth
fn process_line(line: &String, line_number: usize) -> String {
    let new_title = format!("{}. ", line_number.to_string());
    let re = Regex::new(r"^\d\. ").unwrap();
    let processed_line = re.replace(&line, new_title).to_string();
    processed_line
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
    println!("{:?}", subtitle_positions_between_titles);
    subtitle_positions_between_titles
}

fn has_subtitle(line: &str) -> bool {
    let re = Regex::new(r"^\d\.\d\. ").unwrap();
    let has_subtitle = re.is_match(line);
    has_subtitle
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
