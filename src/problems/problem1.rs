use super::common::read_file;

pub fn part1() -> u16 {
    let path = "test_files/1.txt".to_string();
    let lines = read_file(path);
    let mut last = u32::MAX;
    let mut counter: u16 = 0;
    for line in lines {
        let num = line.parse::<u32>().unwrap();
        if last < num {
            counter += 1;
        }
        last = num;
    }
    counter
}

pub fn part2() -> u16 {
    let path = "test_files/1.txt".to_string();
    let lines = read_file(path);
    let mut counter: u16 = 0;
    let len = lines.len();
    let mut nums = Vec::with_capacity(len);
    for line in lines {
        let num = line.parse::<u32>().unwrap();
        nums.push(num);
    }
    for i in 0..len - 3 {
        let this_window = nums[i] + nums[i + 1] + nums[i + 2];
        let next_window = nums[i + 1] + nums[i + 2] + nums[i + 3];
        if this_window < next_window {
            counter += 1;
        }
    }
    counter
}
