use super::common::read_file;
use std::collections::HashMap;

pub fn part1() -> usize {
    let path = "test_files/14.txt".to_string();
    let lines = read_file(path);
    let (mut seq, rules) = create_seq_and_rules(&lines);

    for _ in 0..10 {
        step(&mut seq, &rules);
    }
    let mut counts = HashMap::new();
    for c in seq {
        *counts.entry(c).or_insert(0usize) += 1;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn part2() -> usize {
    let path = "test_files/14.txt".to_string();
    let lines = read_file(path);
    let (mut seq, rules) = create_seq_and_rules(&lines);

    for i in 0..40 {
        step(&mut seq, &rules);
        println!("{}", i);
    }
    let mut counts = HashMap::new();
    for c in seq {
        *counts.entry(c).or_insert(0usize) += 1;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn create_seq_and_rules (lines: &[String]) -> (Vec<char>, HashMap<(char, char), char>){
    let mut iter = lines.into_iter();
    let pattern = iter.next().unwrap().chars().collect::<Vec<_>>();
    iter.next();
    let mut rules = HashMap::new();
    for line in iter {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let key = parts[0].chars().collect::<Vec<_>>();
        let _ = rules.insert((key[0], key[1]), parts[1].chars().next().unwrap());
    }
    (pattern, rules)
}

fn step(sequence: &mut Vec<char>, rules: &HashMap<(char, char), char>) {
    let mut to_insert = Vec::new();
    for i in 0..sequence.len() - 1 {
        to_insert.push(*rules
            .get(&(sequence[i], sequence[i+1]))
            .unwrap())
    }

    for (i, &c) in to_insert.iter().enumerate(){
        sequence.insert(i*2+ 1,c);
    }
}
