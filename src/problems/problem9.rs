use super::common::read_file;

pub fn part1() -> u32 {
    let path = "test_files/9.txt".to_string();
    let lines = read_file(path);
    let mut board = Vec::new();
    for line in lines {
        let row = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        board.push(row)
    }

    let x_len = board[0].len() - 1;
    let y_len = board.len() - 1;

    let mut risk_level = 0;

    for (i, row) in board.iter().enumerate(){
        for (j, &num) in row.iter().enumerate() {
            let is_top = i == 0;
            let is_bottom = i == y_len;
            let is_left = j ==0;
            let is_right = j == x_len;
            let mut is_min = true;

            is_min = is_min && match is_top {
                true => true,
                false => board[i-1][j] > num
            };

            is_min = is_min && match is_bottom {
                true => true,
                false => board[i+1][j] > num
            };

            is_min = is_min && match is_left {
                true => true,
                false => board[i][j-1] > num
            };

            is_min = is_min && match is_right {
                true => true,
                false => board[i][j+1] > num
            };

            if is_min {
                println!("Min {} at {},{}", num, j, i);
                risk_level += num + 1;
            }
        }
    }

    risk_level
}

pub fn part2() -> u32 {
    let path = "test_files/9.txt".to_string();
    let lines = read_file(path);
    0
}