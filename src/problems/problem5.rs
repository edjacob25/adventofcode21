use std::collections::HashMap;

use super::common::read_file;

pub fn part1() -> u32 {
    do_work(&Mode::Problem1)
}

pub fn part2() -> u32 {
    do_work(&Mode::Problem2)
}

fn do_work(mode: &Mode) -> u32 {
    let path = "test_files/5.txt".to_string();
    let lines = read_file(path);

    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();

    for line in lines {
        let coords = create_line(&line, mode);
        for c in coords {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    let mut passed_multiple_times: u32 = 0;
    for (_, passes) in counts.iter() {
        if *passes > 1 {
            passed_multiple_times += 1;
        }
    }

    passed_multiple_times
}

fn create_line(line: &str, mode: &Mode) -> Vec<(u32, u32)> {
    let coords: Vec<&str> = line.split(" -> ").collect();
    let init: Vec<u32> = coords[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let end: Vec<u32> = coords[1]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (changing_idx, static_idx) = if init[0] == end[0] {
        (1, 0)
    } else if init[1] == end[1] {
        (0, 1)
    } else {
        return match *mode {
            Mode::Problem1 => vec![],
            Mode::Problem2 => create_diagonal((init[0], init[1]), (end[0], end[1])),
        };
    };
    let (bigger, smaller) = if init[changing_idx] > end[changing_idx] {
        (init[changing_idx], end[changing_idx])
    } else {
        (end[changing_idx], init[changing_idx])
    };
    (smaller..bigger + 1)
        .map(|x| {
            if static_idx == 0 {
                (init[0], x)
            } else {
                (x, init[1])
            }
        })
        .collect::<Vec<(u32, u32)>>()
}

fn create_diagonal(start: (u32, u32), end: (u32, u32)) -> Vec<(u32, u32)> {
    let range_x: Vec<u32> = if start.0 > end.0 {
        (end.0..start.0 + 1).rev().collect()
    } else {
        (start.0..end.0 + 1).collect()
    };
    let range_y: Vec<u32> = if start.1 > end.1 {
        (end.1..start.1 + 1).rev().collect()
    } else {
        (start.1..end.1 + 1).collect()
    };
    range_x.into_iter().zip(range_y).collect()
}

enum Mode {
    Problem1,
    Problem2,
}
