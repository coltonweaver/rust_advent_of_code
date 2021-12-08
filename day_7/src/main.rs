use std::fs::{self};

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let input = parse_input_file();

    let mut min_cost = i32::MAX;
    let maximum_position = *input.iter().max().expect("Couldn't find max value") as usize;

    for i in 1..maximum_position {
        let i = i as i32;
        // fix i, calculate distance to each of the others
        let mut current_alignment_cost = 0;
        for j in 0..input.len() {
            if i != input[j] {
                let steps = (i - input[j]).abs();
                for k in 1..=steps {
                    current_alignment_cost += k;
                }
            }
        }

        if current_alignment_cost < min_cost {
            min_cost = current_alignment_cost;
        }
    }

    println!("The minimum cost to align the subs is {}", min_cost);
}

fn part_one() {
    let input = parse_input_file();

    let mut min_cost = i32::MAX;
    let maximum_position = *input.iter().max().expect("Couldn't find max value") as usize;

    for i in 0..maximum_position {
        let i = i as i32;
        // fix i, calculate distance to each of the others
        let mut current_alignment_cost = 0;
        for j in 0..input.len() {
            if i != input[j] {
                current_alignment_cost += (i - input[j]).abs();
            }
        }

        if current_alignment_cost < min_cost {
            min_cost = current_alignment_cost;
        }
    }

    println!("The minimum cost to align the subs is {}", min_cost);
}

fn parse_input_file() -> Vec<i32> {
    let input_str =
        fs::read_to_string("day_7/src/input.txt").expect("Couldn't read input.txt into string!");

    input_str
        .split(',')
        .map(|s| s.parse().expect("Couldn't parse string as i32"))
        .collect()
}
