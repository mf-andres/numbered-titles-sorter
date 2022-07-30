use itertools::izip;

pub(crate) struct TitleNode<'a> {
    children: Vec<TitleNode<'a>>,
    depth: u32,
    file_lines_range: (usize, usize),
    title_positions_matrix: &'a Vec<Vec<usize>>,
}

impl<'a> TitleNode<'a> {
    pub fn new(
        depth: u32,
        file_lines_range: (usize, usize),
        title_positions_matrix: &Vec<Vec<usize>>,
    ) -> TitleNode {
        let mut title_node = TitleNode {
            children: vec![],
            depth,
            file_lines_range,
            title_positions_matrix,
        };
        title_node.init();
        return title_node;
    }

    fn init(&mut self) {
        if self.i_am_leaf() {
            return;
        }
        self.create_children();
    }

    fn i_am_leaf(&self) -> bool {
        self.depth == self.title_positions_matrix.len().try_into().unwrap()
    }

    fn create_children(&mut self) {
        let my_subtitle_positions = self.get_my_subtitle_positions();
        println!("subtitle_positions {:?}", &my_subtitle_positions);

        let file_line_ranges_for_children =
            self.get_file_line_ranges_for_children(&my_subtitle_positions);

        for file_lines_range in file_line_ranges_for_children {
            let child = TitleNode::new(
                self.depth + 1,
                file_lines_range,
                self.title_positions_matrix,
            );
            self.children.push(child);
        }
    }

    fn get_my_subtitle_positions(&self) -> Vec<usize> {
        let all_subtitle_positions = &self.title_positions_matrix[self.depth as usize];
        let my_subtitle_positions: Vec<usize> = all_subtitle_positions
            .iter()
            .filter(|x| self.file_lines_range.0 <= **x && **x <= self.file_lines_range.1)
            .map(|x| *x)
            .collect();
        return my_subtitle_positions;
    }

    fn get_file_line_ranges_for_children(
        &self,
        my_subtitle_positions: &Vec<usize>,
    ) -> Vec<(usize, usize)> {
        let range_starts = my_subtitle_positions.clone();
        let range_ends: Vec<usize> = my_subtitle_positions[1..].to_vec();
        // concat last range end which is the range end of current node
        let range_ends: Vec<usize> = [range_ends, [self.file_lines_range.1].to_vec()].concat();
        let file_line_ranges: Vec<(usize, usize)> =
            izip!(range_starts, range_ends).map(|x| x).collect();
        file_line_ranges
    }
}
