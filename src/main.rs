mod file_reader;
mod numbered_titles_sorter;
mod file_writer;

fn main() {
    let file_contents = file_reader::read_file();
    let processed_file_contents = numbered_titles_sorter::sort_numbered_titles(&file_contents);
    file_writer::write_file(&processed_file_contents);
}
