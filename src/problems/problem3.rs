use super::common::read_file;

pub fn do_something() {
    let path = "test_files/3.txt".to_string();
    let lines = read_file(path);
    let mut counts: [u16; 12] = [0; 12];
    let size = lines.len();
    for line in lines {
        let split: Vec<&str> = line.split_terminator("").skip(1).collect();
        for (i, num) in split.iter().enumerate() {
            let n = num.parse::<u16>().unwrap();
            counts[i] += n;
        }
    }
    let gamma_arr: [u8; 12] = create_bit_array(&counts, size);
    let epsilon_arr = invert_bit_array(&gamma_arr);
    let gamma = convert_array_bit_to_int(&gamma_arr);
    let epsilon = convert_array_bit_to_int(&epsilon_arr);
    println!("Gamma is {} and epsilon is {}", gamma, epsilon);


    let power_consumption = gamma * epsilon;
    println!("Dec 3");
    println!("The power consumption is {}", power_consumption);
}

fn create_bit_array(counts: &[u16; 12], len: usize) -> [u8; 12] {
    let mut bit_array = [0;12];
    for (i, count) in counts.iter().enumerate() {
        bit_array[i] =  (*count > (len / 2) as u16) as u8;
    }
    bit_array
    // counts.iter().map(|x| (*x > (len / 2) as u16) as u8).collect()
}

fn invert_bit_array(array: &[u8; 12]) -> [u8; 12] {
    let mut new_arr = [0; 12];
    for (i, u) in array.iter().enumerate() {
        if *u == 0 as u8 {
            new_arr[i] = 1;
        }
    }
    new_arr
}

fn convert_array_bit_to_int(bits: &[u8; 12]) -> u32 {
    bits.iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ bit as u32
        })
}