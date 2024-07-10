use os::{get_file, scanner::{Scanner, Scanning}};

fn main() {
    let file_content : String = get_file("test.txt").to_string();
    let mut scanner : Scanner = Scanner {file: file_content.clone(), char_idx: 0, curr_char: '\n', file_size: file_content.clone().chars().count() };

    scanner.scan_file();
}
