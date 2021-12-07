use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    println!("Running day 6 part 2 of advent of code!");
    let lanternfish = parse_input_file();
    let mut days_to_lanternfish: Vec<i64> = vec![0; 9];

    for i in &lanternfish {
        days_to_lanternfish[*i as usize] += 1;
    }

    for _day in 0..256 {
        let mut temp_vec = vec![0; 9];
        for i in 0..=8 {
            if i == 0 {
                // Add a new lanternfish at 8
                temp_vec[8] = days_to_lanternfish[0];
                // Reset this lanternfish back to 6
                temp_vec[6] = days_to_lanternfish[0];
                // Remove from 0
                days_to_lanternfish[0] = 0;
            } else {
                // Add to day i - 1 in temp_vec
                temp_vec[i - 1] += days_to_lanternfish[i];
                days_to_lanternfish[i] = 0;
            }
        }

        for i in 0..=8 {
            days_to_lanternfish[i] += temp_vec[i];
        }
    }
    println!(
        "There are now {} lanternfish.",
        days_to_lanternfish.iter().sum::<i64>()
    );
}

fn part_one() {
    println!("Running day 6 part 1 of advent of code!");
    let mut lanternfish = parse_input_file();
    for _day in 0..80 {
        let initial_size = lanternfish.len();
        for i in 0..initial_size {
            if lanternfish[i] == 0 {
                lanternfish.push(8);
                lanternfish[i] = 6;
            } else {
                lanternfish[i] -= 1;
            }
        }
    }
    println!("There are now {} lanternfish.", lanternfish.len());
}

fn parse_input_file() -> Vec<i32> {
    let f = File::open("src/input.txt").expect("Couldn't open input.txt!");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Couldn't parse line"))
        .collect::<Vec<String>>();

    let input_line = lines[0].clone();

    input_line
        .split(',')
        .map(|s| s.parse().expect("Couldn't parse string as i32"))
        .collect()
}
