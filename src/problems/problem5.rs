use std::collections::HashMap;

use super::common::read_file;

pub fn part1() {
    let path = "test_files/5.txt".to_string();
    let lines = read_file(path);

    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    for line in lines{
        let coords = create_line(&line);
        for c in coords {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    let mut passed_multiple_times: u32 = 0;
    for (_, passes) in counts.iter(){
        if *passes > 1 {
            passed_multiple_times += 1;
        }
    }

    println!("Day 5");
    println!("The number of overlapping points is {}", passed_multiple_times);
}

pub fn part2() {
    unimplemented!()
}

fn create_line(line: &str) -> Vec<(u32, u32)> {
    let coords: Vec<&str> = line.split(" -> ").collect();
    let first_coord: Vec<u32> = coords[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let second_coord: Vec<u32> = coords[1]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (changing_idx, static_idx) = if first_coord[0] == second_coord[0] {
        (1, 0)
    } else if first_coord[1] == second_coord[1] {
        (0, 1)
    } else {
        return vec!{};
    };
    let (bigger, smaller) = if first_coord[changing_idx] > second_coord[changing_idx] {
        (first_coord[changing_idx], second_coord[changing_idx])
    } else {
        (second_coord[changing_idx], first_coord[changing_idx])
    };
    (smaller..bigger + 1)
        .map(|x| if static_idx == 0 {(first_coord[0], x)} else {(x, first_coord[1])})
        .collect::<Vec<(u32, u32)>>()
}
