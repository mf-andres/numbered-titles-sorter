use itertools::izip;

pub(crate) struct TitleNode {
    children: Vec<TitleNode>,
}

impl TitleNode {
    pub fn new(
        depth: u32,
        file_lines_range: (usize, usize),
        title_positions_matrix: &Vec<Vec<usize>>,
    ) -> TitleNode {
        if iAmLeaf(depth, title_positions_matrix) {
            return TitleNode { children: vec![] };
        }

        let all_subtitle_positions = &title_positions_matrix[depth as usize];
        let my_subtitle_positions =
            TitleNode::get_my_subtitle_positions(file_lines_range, all_subtitle_positions);
        println!("subtitle_positions {:?}", &my_subtitle_positions);
        // create subnodes
        let children = TitleNode::create_children(
            depth,
            file_lines_range,
            title_positions_matrix,
            &my_subtitle_positions,
        );
        return TitleNode { children };
    }

    fn get_my_subtitle_positions(
        file_lines_range: (usize, usize),
        all_subtitle_positions: &Vec<usize>,
    ) -> Vec<usize> {
        let my_subtitle_positions: Vec<usize> = all_subtitle_positions
            .iter()
            .filter(|x| file_lines_range.0 <= **x && **x <= file_lines_range.1)
            .map(|x| *x)
            .collect();
        return my_subtitle_positions;
    }

    fn create_children(
        depth: u32,
        file_lines_range: (usize, usize),
        title_positions_matrix: &Vec<Vec<usize>>,
        my_subtitle_positions: &Vec<usize>,
    ) -> Vec<TitleNode> {
        let mut children: Vec<TitleNode> = vec![];
        let file_line_ranges_for_children =
            TitleNode::get_file_line_ranges_for_children(file_lines_range, my_subtitle_positions);
        for file_lines_range in file_line_ranges_for_children {
            let child = TitleNode::new(depth + 1, file_lines_range, title_positions_matrix);
            children.push(child);
        }
        return children;
    }

    fn get_file_line_ranges_for_children(
        file_lines_range: (usize, usize),
        my_subtitle_positions: &Vec<usize>,
    ) -> Vec<(usize, usize)> {
        let range_starts = my_subtitle_positions.clone();
        let range_ends: Vec<usize> = my_subtitle_positions[1..].to_vec();
        let range_ends: Vec<usize> = [range_ends, [file_lines_range.1].to_vec()].concat();
        let file_line_ranges: Vec<(usize, usize)> =
            izip!(range_starts, range_ends).map(|x| x).collect();
        file_line_ranges
    }
}

fn iAmLeaf(depth: u32, title_positions_matrix: &Vec<Vec<usize>>) -> bool {
    depth == title_positions_matrix.len().try_into().unwrap()
}
