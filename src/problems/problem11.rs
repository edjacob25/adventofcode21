use super::common::read_file;
use std::collections::HashMap;

type Board = HashMap<(i32, i32), u8>;

pub fn part1() -> u32 {
    let path = "test_files/11.txt".to_string();
    let lines = read_file(path);
    let (mut board, size) = create_board(&lines);
    let mut result: u32 = 0;
    for _ in 0..100 {
        cycle(&mut board, size);
        result += board.values().filter(|&x| *x == 0).count() as u32;
    }
    result
}

pub fn part2() -> u32 {
    let path = "test_files/11.txt".to_string();
    let lines = read_file(path);
    let (mut board, size) = create_board(&lines);
    let mut i = 1;
    loop {
        cycle(&mut board, size);
        if board.values().all(|&x| x == 0) {
            return i;
        }
        i += 1;
    }
}

fn cycle(board: &mut Board, size: usize) {
    let neighbors = vec![
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut flashes_pending = Vec::new();
    for i in 0..size {
        for j in 0..size {
            let entry = board.entry((i as i32, j as i32)).or_default();

            *entry += 1;
            if *entry > 9 {
                flashes_pending.push((i as i32, j as i32));
                *entry = 0;
            }
        }
    }
    while flashes_pending.len() > 0 {
        let mut new_flashes = Vec::new();
        for (i, j) in flashes_pending {
            let real_neighbors = neighbors
                .iter()
                .map(|(y, x)| (i + y, j + x))
                .filter(|pos| (*board).contains_key(&pos))
                .collect::<Vec<_>>();

            for pos in real_neighbors {
                let entry = board.entry(pos).or_default();
                if *entry > 0 {
                    *entry += 1;
                    if *entry > 9 {
                        new_flashes.push((pos.0, pos.1));
                        *entry = 0;
                    }
                }
            }
        }
        flashes_pending = new_flashes;
    }
}

fn create_board(lines: &[String]) -> (Board, usize) {
    let mut result = HashMap::new();
    let size = lines.len();
    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let _ = *result
                .entry((j as i32, i as i32))
                .or_insert(c.to_digit(10).unwrap() as u8);
        }
    }
    (result, size)
}
