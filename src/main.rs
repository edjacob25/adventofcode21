#![allow(dead_code)]
use crate::problems::problem9::{part1, part2};

mod problems;

fn main() {
    println!("Hello, world!");

    let day = 9;

    let result = part1();

    println!("Day {}, part 1", day);
    println!("Result is {}", result);

    let result2 = part2();

    println!("Day {}, part 2", day);
    println!("Result is {}", result2);
}
