use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    println!("Running day 3 part 1 advent of code!");
    let f = File::open("src/input.txt").expect("Could not open file!");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let first_line = &lines[0];
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for i in 0..first_line.len() {
        let mut one_count = 0;
        for j in 0..lines.len() {
            let char_array = lines[j].as_bytes();
            if char_array[i] == b'1' {
                one_count += 1;
            }
        }

        if one_count >= (lines.len() / 2) {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    let product = gamma_rate * epsilon_rate;

    println!("Gamma: {} and Epsilon: {}", gamma_rate, epsilon_rate);
    println!("Power Consumption: {}", product);
}

fn part_two() {
    println!("Running day 3 part 2 advent of code!");
    let f = File::open("src/input.txt").expect("Could not open file!");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut oxy_gen_list: Vec<String> = lines.clone();
    let mut co2_list: Vec<String> = lines.clone();

    for i in 0..lines[0].len() {
        if oxy_gen_list.len() > 1 {
            oxy_gen_list = filter_values(oxy_gen_list, i, true);
        }

        if co2_list.len() > 1 {
            co2_list = filter_values(co2_list, i, false);
        }

        if oxy_gen_list.len() == 1 && co2_list.len() == 1 {
            println!("Filtered out all incorrect values.");
            break;
        }
    }

    println!("{:?}", oxy_gen_list);
    println!("{:?}", co2_list);

    let oxy_gen_rate = isize::from_str_radix(&oxy_gen_list[0], 2).unwrap();
    let co2_rate = isize::from_str_radix(&co2_list[0], 2).unwrap();

    let product = oxy_gen_rate * co2_rate;

    println!("Life support rating: {}", product);
}

fn filter_values(mut bin_nums: Vec<String>, char_idx: usize, most_common: bool) -> Vec<String> {
    let mut one_count = 0;
    let mut zero_count = 0;
    for j in 0..bin_nums.len() {
        let bytes = bin_nums[j].as_bytes();
        if bytes[char_idx] == b'1' {
            one_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let one_most_common = one_count >= zero_count;
    let zero_most_common = !one_most_common;

    bin_nums.retain(|s| {
        let bytes = s.as_bytes();

        // Writing this to be extremely clear
        if most_common && one_most_common {
            bytes[char_idx] == b'1'
        } else if most_common && zero_most_common {
            bytes[char_idx] == b'0'
        } else if !most_common && zero_most_common {
            bytes[char_idx] == b'1'
        } else if !most_common && one_most_common {
            bytes[char_idx] == b'0'
        } else {
            // This case can't be hit, but is to make the compiler happy.
            true
        }
    });

    bin_nums
}
