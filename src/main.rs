mod file_reader;
mod file_writer;
mod numbered_titles_sorter;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "numbered_titles_sorter")]
#[clap(author = "MF Andrés <munozfernandezandres@gmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "Sorts/updates title numbers", long_about = None)]
struct Cli {
    #[clap(value_parser)]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);

    let file_contents = file_reader::read_file();
    let processed_file_contents = numbered_titles_sorter::sort_numbered_titles(&file_contents);
    file_writer::write_file(&processed_file_contents);
}
