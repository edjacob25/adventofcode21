use std::collections::HashMap;

use super::common::read_file;

type Board = HashMap<u16, Position>;

struct Position {
    x: u16,
    y: u16,
}

pub fn part1() {
    let path = "test_files/4.txt".to_string();
    let lines = read_file(path);

    let first_line = lines.first().unwrap();
    let calling_order = create_calling_order(first_line);
    let mut line_buffer = Vec::new();
    let mut results = Vec::new();
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            let (board, size) = create_table(&line_buffer);
            let result = calculate_board_value(&board, &calling_order, size);

            println!("{} steps with value {}", result.1, result.0);
            results.push(result);
            line_buffer = Vec::new();
        } else {
            line_buffer.push(line)
        }
    }

    let (result, steps) = results.iter().min_by_key(|(_, steps)| *steps).unwrap();
    println!("The resulting board did {} steps and had a result of {}", steps, result);
}

pub fn part2() {}

fn create_calling_order(line: &str) -> Vec<u16> {
    line.split(",").map(|x| x.parse::<u16>().unwrap()).collect()
}

fn create_table(lines: &[&str]) -> (Board, u16) {
    let mut result = HashMap::new();
    let mut idx: u16 = 0;
    for line in lines {
        let exploded: Vec<u16> = line.split_whitespace().map(|x| x.parse::<u16>().unwrap()).collect();
        for (i, num) in exploded.iter().enumerate() {
            result.entry(*num).or_insert(Position { x: idx, y: i as u16 });
        }
        idx += 1;
    }
    (result, idx)
}

fn calculate_board_value(board: &Board, calling_order: &[u16], side_size: u16) -> (u32, u32) {
    let mut total: u32 = board.keys().map(|x| *x as u32).sum();
    let mut xs: Vec<u16> = vec![0; side_size as usize];
    let mut ys: Vec<u16> = vec![0; side_size as usize];
    let mut steps_taken: u32 = 0;

    for number in calling_order {
        let (x, y) = match board.get(number) {
            Some(pos) => (pos.x, pos.y),
            None => continue
        };
        total -= *number as u32;
        xs[x as usize] += 1;
        ys[y as usize] += 1;
        steps_taken += 1;
        if xs[x as usize] == side_size || ys[y as usize] == side_size {
            total *= *number as u32;
            break;
        }
    }
    (total, steps_taken)
}