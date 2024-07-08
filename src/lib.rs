pub mod tokens;
pub mod scanner;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_file(path : String) -> String {
    let path = Path::new(path.as_str());

    let mut file = match File::open(&path) {
        Err(_) => panic!(),
        Ok(value) => value
    };


    let mut s : String = String::new();
    file.read_to_string(&mut s).unwrap();

    return s;
}
