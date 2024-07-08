use std::{char, env};
use os::get_file;

fn main() {
    println!("{:?}", get_file("test.txt".to_string()).chars().collect::<Vec<char>>());
}
