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
        .fold(0, |result, &bit| {
            (result << 1) ^ bit as u32
        })
}

enum LookingFor {
    MIN,
    MAX,
}