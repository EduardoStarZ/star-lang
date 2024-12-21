use star_lang::{get_file, scanner::{Scanner, Scanning}};
use std::env::args;

fn main() {
    let raw_args = args().into_iter().collect::<Vec<String>>(); 

    let file : &String = raw_args.get(1).unwrap();

    let file_content : String = get_file(file.as_str()).to_string();
    let mut scanner : Scanner = Scanner {file: file_content.clone(), char_idx: 0, curr_char: '\n', file_size: file_content.clone().chars().count() };

    scanner.scan_file();
}
