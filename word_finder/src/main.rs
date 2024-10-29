use std::env::args;

mod arguments;
mod errors;

fn main() {
    let args = args().collect::<Vec<String>>();
    match arguments::parse_args(args) {
        Ok(args_struct) => {
            let lines = arguments::file_reader::read_file(&args_struct.file_path);
            match lines {
                Ok(lines) => {
                    arguments::search::search_word(&lines, &args_struct.word);
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }
        Err(e) => {
            arguments::help::print_help();
            eprintln!("{:?}", e);
        }
    }
}
