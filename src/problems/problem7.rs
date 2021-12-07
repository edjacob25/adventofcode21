use super::common::read_file;

pub fn part1() -> u32 {
    common(Mode::First)
}

pub fn part2() -> u32 {
    common(Mode::Second)
}

fn common(mode: Mode) -> u32 {
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
        let res = get_movement_cost(&positions, i, &mode);
        if min > res {
            min = res;
        }
    }
    min
}

fn get_movement_cost(positions: &[u32], goal: u32, mode: &Mode) -> u32 {
    let mut cost = 0;
    for pos in positions {
        match mode {
            Mode::First => cost += (*pos as i32 - goal as i32).abs() as u32,
            Mode::Second => {
                let diff = (*pos as i32 - goal as i32).abs() as u32;
                let partial_cost = (diff * (diff + 1)) / 2;
                cost += partial_cost
            }
        }
    }
    cost
}

enum Mode {
    First,
    Second,
}
