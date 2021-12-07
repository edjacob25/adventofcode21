use super::common::read_file;

pub fn part1() -> u32 {
    let path = "test_files/7.txt".to_string();
    let lines = read_file(path);
    let positions: Vec<u32> = lines[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mean = positions.iter().sum::<u32>() / positions.len() as u32;
    let max = positions.iter().max().unwrap();
    let min = positions.iter().min().unwrap();
    let diff = max - min;
    let window = diff / 5;

    let mut min = u32::MAX;
    for i in mean - window..mean + window + 1 {
        let res = get_movement_cost(&positions, i);
        println!("Goal {} with cost {}", i, res);
        if min > res {
            min = res;
        }
    }
    min
}

pub fn get_movement_cost(positions: &[u32], goal: u32) -> u32 {
    let mut cost = 0;
    for pos in positions {
        cost += (*pos as i32 - goal as i32).abs() as u32
    }
    cost
}

pub fn part2() -> u64 {
    let path = "test_files/6.txt".to_string();
    let lines = read_file(path);
    0
}