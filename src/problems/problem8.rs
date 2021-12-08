use super::common::read_file;

pub fn part1() -> u32 {
    let path = "test_files/8.txt".to_string();
    let lines = read_file(path);
    let displays: Vec<Display> = Vec::new();
    let mut count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let outputs: Vec<&str>= parts[1].split_whitespace().collect();
        let line_count = outputs.into_iter().filter(|&x| x.len() < 5 || x.len() == 7 ).count();
        count += line_count;
    }
    count as u32
}

pub fn part2() -> u32 {
    let path = "test_files/8.txt".to_string();
    let lines = read_file(path);
    0
}

struct Display<'a> {
    patterns: Vec<&'a str>,
    output: Vec<&'a str>
}