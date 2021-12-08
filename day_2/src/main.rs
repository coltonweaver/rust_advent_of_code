use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    println!("Running day 2 part 1 advent of code!");
    let f = File::open("day_2/src/input.txt").expect("Could not open file!");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let dir = parts[0];
        let unit = parts[1].parse::<i32>().expect("Cannot parse as i32");
        if dir == "forward" {
            horizontal += unit;
        } else if dir == "up" {
            depth -= unit;
        } else {
            depth += unit;
        }
    }

    println!("Final multiplied depth is {}", (horizontal * depth));
}

fn part_two() {
    println!("Running day 2 part 2 advent of code!");
    let f = File::open("day_2/src/input.txt").expect("Could not open file!");
    let lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let dir = parts[0];
        let unit = parts[1].parse::<i32>().expect("Cannot parse as i32");
        if dir == "forward" {
            horizontal += unit;
            depth += aim * unit;
        } else if dir == "up" {
            aim -= unit;
        } else {
            aim += unit;
        }
    }

    println!("Final multiplied depth is {}", (horizontal * depth));
}
