use super::common::read_file;

pub fn do_something() {
    let path = "test_files/2.txt".to_string();
    let lines = read_file(path);
    let mut x: u32 = 0;
    let mut y = 0;
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        let quantity = split[1].parse::<u32>().unwrap();
        match split[0] {
            "forward" => { x += quantity }
            "up" => { y -= quantity }
            "down" => { y += quantity }
            _ => ()
        }
    }
    let result = x * y;
    println!("Dec 2");
    println!("The resulting number is {}", result);
}