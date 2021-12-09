use super::common::read_file;

pub fn part1() -> u32 {
    let path = "test_files/9.txt".to_string();
    let lines = read_file(path);
    let mut board = Vec::new();
    for line in lines {
        let row = line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        board.push(row)
    }
    let mut risk_level = 0;

    let low_points = find_low_points(&board);

    for (x, y) in low_points {
        risk_level += board[y][x] + 1;
    }

    risk_level
}

pub fn part2() -> u32 {
    let path = "test_files/9.txt".to_string();
    let lines = read_file(path);
    let mut board = Vec::new();
    for line in lines {
        let row = line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        board.push(row)
    }
    let low_points = find_low_points(&board);
    let mut basins = Vec::new();
    let mut passed = vec![vec![false; board[0].len()]; board.len()];

    for low_point in low_points {
        let basin = calculate_basin(low_point, &board, &mut passed);
        basins.push(basin);
    }

    let mut lens = basins
        .into_iter()
        .map(|x| x.len() as u32)
        .collect::<Vec<u32>>();
    lens.sort();
    let size = lens.len();
    lens.into_iter()
        .skip(size - 3)
        .fold(1, |mul, num| mul * num)
}

fn calculate_basin(
    low_point: (usize, usize),
    board: &Vec<Vec<u32>>,
    passed: &mut Vec<Vec<bool>>,
) -> Vec<u32> {
    let x_len = board[0].len() - 1;
    let y_len = board.len() - 1;

    let (x, y) = low_point;
    let mut result = Vec::new();
    passed[y][x] = true;
    let current: u32 = board[y][x];
    result.push(current);

    if x > 0 {
        let other = board[y][x - 1];
        if other > current && other < 9 && !passed[y][x - 1] {
            result.append(&mut calculate_basin((x - 1, y), board, &mut *passed))
        }
    }

    if x < x_len {
        let other = board[y][x + 1];
        if other > current && other < 9 && !passed[y][x + 1] {
            result.append(&mut calculate_basin((x + 1, y), board, &mut *passed))
        }
    }
    if y > 0 {
        let other = board[y - 1][x];
        if other > current && other < 9 && !passed[y - 1][x] {
            result.append(&mut calculate_basin((x, y - 1), board, &mut *passed))
        }
    }
    if y < y_len {
        let other = board[y + 1][x];
        if other > current && other < 9 && !passed[y + 1][x] {
            result.append(&mut calculate_basin((x, y + 1), board, &mut *passed))
        }
    }

    result
}

fn find_low_points(board: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let x_len = board[0].len() - 1;
    let y_len = board.len() - 1;
    let mut res = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            let mut is_min = true;
            is_min = is_min
                && match i == 0 {
                    true => true,
                    false => board[i - 1][j] > num,
                };

            is_min = is_min
                && match i == y_len {
                    true => true,
                    false => board[i + 1][j] > num,
                };

            is_min = is_min
                && match j == 0 {
                    true => true,
                    false => board[i][j - 1] > num,
                };

            is_min = is_min
                && match j == x_len {
                    true => true,
                    false => board[i][j + 1] > num,
                };

            if is_min {
                res.push((j, i))
            }
        }
    }
    res
}
