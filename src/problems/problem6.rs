use super::common::read_file;

pub fn part1() {
    let path = "test_files/6.txt".to_string();
    let lines = read_file(path);
    let mut fishes: Vec<u8> = lines[0]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    for _ in 0..80 {
        let mut fishes_new = fishes.clone();
        for (i, fish) in fishes.iter().enumerate() {
            if *fish == 0 {
                fishes_new[i] = 6;
                fishes_new.push(8);
            } else {
                fishes_new[i] -= 1;
            }
        }
        fishes = fishes_new;
    }

    let num_of_fish = fishes.len();
    println!("The number of fishes after 80 days is {}", num_of_fish);
}

pub fn part2() {}
