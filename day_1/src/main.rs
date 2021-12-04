use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    println!("Running day 1 part 1 advent of code!");
    let f = File::open("src/input.txt").expect("Cannot open file!");
    let lines = io::BufReader::new(f).lines();

    let mut prev_line = -1;
    let mut increases = 0;

    for line in lines {
        if let Ok(line) = line {
            let val = line.parse::<i32>().expect("Cannot parse line as i32!");
            if prev_line != -1 && val > prev_line {
                increases += 1;
            }

            prev_line = val;
        }
    }

    println!("There were {} increases!", increases);
}

fn part_two() {
    println!("Running day 1 part 2 advent of code!");
    let f = File::open("src/input.txt").expect("Cannot open file!");
    let lines: Vec<i32> = io::BufReader::new(f)
        .lines()
        .map(|l| {
            l.expect("Cannot parse line!")
                .parse()
                .expect("Cannot parse line as i32")
        })
        .collect();

    let mut prev_sum = -1;
    let mut increases = 0;

    for i in 0..lines.len() {
        if (i + 2) < lines.len() {
            let sum = lines[i] + lines[i + 1] + lines[i + 2];
            if prev_sum != -1 && sum > prev_sum {
                increases += 1;
            }

            prev_sum = sum;
        }
    }

    println!("There were {} increases!", increases);
}
