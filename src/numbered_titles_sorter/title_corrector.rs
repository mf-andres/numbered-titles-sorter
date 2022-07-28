use super::line_processor::process_line;

pub(crate) fn correct_titles(
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
