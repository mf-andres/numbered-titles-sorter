use itertools::izip;

use super::line_processor::process_line;

pub(crate) fn correct_subtitles(
    title_positions: Vec<usize>,
    number_of_lines: usize,
    subtitle_positions: Vec<usize>,
    mut file_lines: Vec<String>,
) -> String {
    let mut title_numbers_range = 1..title_positions.len() + 1;
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
