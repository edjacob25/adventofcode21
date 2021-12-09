use super::common::read_file;

pub fn part1() -> u32 {
    let path = "test_files/8.txt".to_string();
    let lines = read_file(path);
    let mut count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let outputs: Vec<&str> = parts[1].split_whitespace().collect();
        let line_count = outputs
            .into_iter()
            .filter(|&x| x.len() < 5 || x.len() == 7)
            .count();
        count += line_count;
    }
    count as u32
}

pub fn part2() -> u32 {
    let path = "test_files/8.txt".to_string();
    let lines = read_file(path);
    let mut acc = 0;
    for line in lines.into_iter() {
        let parts: Vec<&str> = line.split(" | ").collect();
        let patterns: Vec<&str> = parts[0].split_whitespace().collect();
        let output: Vec<&str> = parts[1].split_whitespace().collect();
        let mapping = create_mapping(&patterns);

        let out = calculate_output(output, mapping);
        acc += out;
    }
    acc
}

fn calculate_output(output: Vec<&str>, mapping: Vec<(String, String)>) -> u32 {
    let mut res: Vec<&str> = vec![];
    for part in output {
        let mut as_chars = part.chars().collect::<Vec<char>>();
        as_chars.sort();
        let sorted = as_chars.into_iter().collect::<String>();
        let (num, _) = mapping.iter().filter(|(_, x)| *x == sorted).next().unwrap();
        res.push(num);
    }
    let complete = res.join("");
    complete.parse::<u32>().unwrap()
}

fn create_mapping(patterns: &Vec<&str>) -> Vec<(String, String)> {
    let mut missing = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut mapping = vec![];
    let mut patterns = patterns.clone();
    patterns.sort_by_key(|&x| x.len());

    // Identify the 1 chars
    let mut one: Vec<char> = patterns[0].chars().collect();
    one.sort();

    let cf = vec![one[0], one[1]];
    missing.retain(|&x| !cf.contains(&x));

    // Identify the 7 free char
    let mut seven: Vec<char> = patterns[1].chars().collect();
    seven.sort();

    let a = *seven.iter().filter(|&x| !cf.contains(x)).next().unwrap();
    missing.retain(|&x| x != a);

    // Identify the 4s and the other chars
    let mut four: Vec<char> = patterns[2].chars().collect();
    four.sort();

    let rest = four.iter().filter(|&x| !cf.contains(x)).collect::<Vec<_>>();
    let bd = vec![*rest[0], *rest[1]];
    missing.retain(|x| !bd.contains(&x));
    let eg = vec![missing[0], missing[1]];

    // Identify the 5 so  we can complete the mapping
    let mut five = patterns
        .iter()
        .filter(|&x| {
            let as_chars = (*x).chars().collect::<Vec<char>>();
            (*x).len() == 5 && bd.iter().all(|c| as_chars.contains(c))
        })
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    five.sort();

    let f = *five.iter().filter(|x| cf.contains(x)).next().unwrap();
    let c = cf.into_iter().filter(|&x| x != f).next().unwrap();

    // Identify the 3 so we can complete the mapping
    let mut three = patterns
        .iter()
        .filter(|&x| {
            let as_chars = x.chars().collect::<Vec<char>>();
            (*x).len() == 5 && seven.iter().all(|y| as_chars.contains(y))
        })
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    three.sort();

    let d = *three.iter().filter(|&x| bd.contains(x)).next().unwrap();
    let g = *three
        .iter()
        .filter(|&x| !bd.contains(x) && !seven.contains(x))
        .next()
        .unwrap();
    let e = eg.into_iter().filter(|&x| x != g).next().unwrap();

    let mut two = patterns
        .iter()
        .filter(|&x| {
            let as_chars = x.chars().collect::<Vec<char>>();
            x.len() == 5 && as_chars.contains(&e)
        })
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    two.sort();

    let mut six = patterns
        .iter()
        .filter(|&x| {
            let as_chars = x.chars().collect::<Vec<char>>();
            x.len() == 6 && !as_chars.contains(&c)
        })
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    six.sort();

    let mut nine = patterns
        .iter()
        .filter(|&x| {
            let as_chars = x.chars().collect::<Vec<char>>();
            x.len() == 6 && !as_chars.contains(&e)
        })
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    nine.sort();

    let all = "abcdefg".to_string();
    let mut zero = all.clone();

    mapping.push(("1".to_string(), one.into_iter().collect::<String>()));
    mapping.push(("2".to_string(), two.into_iter().collect::<String>()));
    mapping.push(("3".to_string(), three.into_iter().collect::<String>()));
    mapping.push(("4".to_string(), four.into_iter().collect::<String>()));
    mapping.push(("5".to_string(), five.into_iter().collect::<String>()));
    mapping.push(("6".to_string(), six.into_iter().collect::<String>()));
    mapping.push(("7".to_string(), seven.into_iter().collect::<String>()));
    mapping.push(("8".to_string(), all));
    mapping.push(("9".to_string(), nine.into_iter().collect::<String>()));

    zero.retain(|x| x != d);
    mapping.push(("0".to_string(), zero));

    mapping
}
