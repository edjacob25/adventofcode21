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
    let (seq, rules) = create_seq_and_rules(&lines);

    let mut pairs = HashMap::new();
    for i in 0..seq.len() - 1 {
        *pairs.entry((seq[i], seq[i + 1])).or_insert(0usize) += 1;
    }

    let mut individuals = HashMap::new();
    for c in seq {
        *individuals.entry(c).or_insert(0usize) += 1;
    }

    for _ in 0..40 {
        //println!("{:?}, {:?}", pairs, individuals);
        step_alt(&mut pairs, &mut individuals, &rules);
    }

    individuals.values().max().unwrap() - individuals.values().min().unwrap()
}

fn create_seq_and_rules(lines: &[String]) -> (Vec<char>, HashMap<(char, char), char>) {
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
        to_insert.push(*rules.get(&(sequence[i], sequence[i + 1])).unwrap())
    }

    for (i, &c) in to_insert.iter().enumerate() {
        sequence.insert(i * 2 + 1, c);
    }
}

fn step_alt(
    pairs: &mut HashMap<(char, char), usize>,
    individuals: &mut HashMap<char, usize>,
    rules: &HashMap<(char, char), char>,
) {
    let mut to_add = Vec::new();
    let mut to_remove = Vec::new();

    for (item, count) in pairs.iter() {
        let rule = *rules.get(item).unwrap();
        to_add.push(((item.0, rule), *count));
        to_add.push(((rule, item.1), *count));
        to_remove.push((*item, *count));
        *individuals.entry(rule).or_default() += *count;
    }

    for (place, new) in to_add {
        *pairs.entry(place).or_default() += new;
    }
    for (place, new) in to_remove {
        *pairs.entry(place).or_default() -= new;
    }
}
