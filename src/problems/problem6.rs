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
    println!("Day 6, part 1");
    println!("The number of fishes after 80 days is {}", num_of_fish);
}

pub fn part2() {
    let path = "test_files/6.txt".to_string();
    let lines = read_file(path);
    let mut fishes: Vec<u64> = vec![0; 9];
    let initial: Vec<usize> = lines[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for num in initial {
        fishes[num] += 1;
    }

    for _ in 0..256 {
        let mut new_pop = vec![0; 9];
        for i in (0..9).rev() {
            if i == 0 {
                new_pop[6] += fishes[0];
                new_pop[8] = fishes[0];
            } else {
                new_pop[i - 1] = fishes[i];
            }
        }
        fishes = new_pop;
    }
    let num_of_fish: u64 = fishes.iter().sum();
    println!("Day 6, part 1");
    println!("The number of fishes after 80 days is {}", num_of_fish);
}
