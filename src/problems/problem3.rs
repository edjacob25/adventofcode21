use std::collections::HashMap;

use super::common::read_file;

pub fn part1() {
    let path = "test_files/3.txt".to_string();
    let lines = read_file(path);
    let mut counts = vec![0; lines[0].len()];
    let size = lines.len();
    for line in lines {
        let split: Vec<&str> = line.split_terminator("").skip(1).collect();
        for (i, num) in split.iter().enumerate() {
            let n = num.parse::<u16>().unwrap();
            counts[i] += n;
        }
    }
    let gamma = create_bit_array(&counts, size);
    println!("{:?}", gamma);
    let epsilon = invert_bit_array(&gamma);
    let gamma = convert_array_bit_to_int(&gamma);
    let epsilon = convert_array_bit_to_int(&epsilon);
    println!("Gamma is {} and epsilon is {}", gamma, epsilon);

    let power_consumption = gamma * epsilon;
    println!("Dec 3");
    println!("The power consumption is {}", power_consumption);
}

pub fn part2() {
    let path = "test_files/3.txt".to_string();
    let lines = read_file(path);
    let lines_ref = lines.iter().map(|s| s as &str).collect();
    let o2_rating = select_string(0, lines_ref, LookingFor::MAX);
    println!("O2 rating is {}", o2_rating);
    let lines_ref = lines.iter().map(|s| s as &str).collect();
    let co2_scrubber = select_string(0, lines_ref, LookingFor::MIN);
    println!("CO2 rating is {}", co2_scrubber);

    let o2_rating = convert_array_bit_to_int(&convert_binary_string_to_byte_array(o2_rating));
    let co2_scrubber = convert_array_bit_to_int(&convert_binary_string_to_byte_array(co2_scrubber));

    let life_support = o2_rating * co2_scrubber;
    println!("Dec 3, part 2");
    println!("The life support rating is {}", life_support);
}

fn convert_binary_string_to_byte_array(string: &str) -> Vec<u8> {
    let mut vec = vec![0; 0];
    for char in string.as_bytes() {
        vec.push(char - 48);
    }
    vec
}

fn select_string(index: usize, list: Vec<&str>, looking_for: LookingFor) -> &str {
    return if list.len() < 2 {
        &list[0]
    } else {
        println!("{:?}", list);
        let mut counts: HashMap<u8, u32> = HashMap::new();
        for c in list.iter() {
            *counts.entry((*c).as_bytes()[index]).or_insert(0) += 1;
        }

        let common = match looking_for {
            LookingFor::MAX => {
                let (max, num) = counts.into_iter().max_by_key(|&(_, count)| count).unwrap();
                if num == (list.len() / 2) as u32 {
                    b'1'
                } else {
                    max
                }
            }
            LookingFor::MIN => {
                let (min, num) = counts.into_iter().min_by_key(|&(_, count)| count).unwrap();
                if num as f32 == list.len() as f32 / 2.0 {
                    b'0'
                } else {
                    min
                }
            }
        };

        let new_list = list
            .into_iter()
            .filter(|x| (*x).as_bytes()[index] == common)
            .collect();
        select_string(index + 1, new_list, looking_for)
    };
}

fn create_bit_array(counts: &[u16], len: usize) -> Vec<u8> {
    let mut bit_array = vec![0; counts.len()];
    for (i, count) in counts.iter().enumerate() {
        bit_array[i] = (*count >= (len / 2) as u16) as u8;
    }
    bit_array
    // counts.iter().map(|x| (*x > (len / 2) as u16) as u8).collect()
}

fn invert_bit_array(array: &[u8]) -> Vec<u8> {
    let mut new_arr = vec![0; array.len()];
    for (i, u) in array.iter().enumerate() {
        if *u == 0 as u8 {
            new_arr[i] = 1;
        }
    }
    new_arr
}

fn convert_array_bit_to_int(bits: &[u8]) -> u32 {
    bits.iter()
        .fold(0, |result, &bit| (result << 1) ^ bit as u32)
}

enum LookingFor {
    MIN,
    MAX,
}
