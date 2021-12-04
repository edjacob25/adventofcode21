use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: String) -> Vec<String> {
    let input = File::open(path).expect("File not found");
    let buffered = BufReader::new(input);
    buffered.lines().map(|x| x.unwrap()).collect()
}