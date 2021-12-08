use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
struct Board {
    // The actual values at each position
    numbers: Vec<Vec<i32>>,
    // Used to track registered inputs
    positions: Vec<Vec<i32>>,
    already_completed: bool,
}

impl Board {
    fn new(numbers: Vec<Vec<i32>>) -> Self {
        Self {
            numbers,
            positions: vec![vec![0; 5]; 5],
            already_completed: false,
        }
    }

    fn register_number(&mut self, number: i32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == number {
                    self.positions[i][j] = 1;
                }
            }
        }
    }

    fn get_unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.positions.len() {
            for j in 0..self.positions[i].len() {
                if self.positions[i][j] == 0 {
                    sum += self.numbers[i][j];
                }
            }
        }

        sum
    }

    fn is_already_completed(&self) -> bool {
        self.already_completed
    }

    // Return true if win, false if not
    fn check_win(&mut self) -> bool {
        let win = self.check_row() || self.check_col();
        self.already_completed = win;
        win
    }

    fn check_row(&self) -> bool {
        // Scan rows first
        for i in 0..self.positions.len() {
            let mut row_count = 0;
            for j in 0..self.positions[i].len() {
                if self.positions[i][j] == 1 {
                    row_count += 1;
                } else {
                    // Row does not contain all 1s, break and go to next
                    break;
                }
            }

            // If all elements in row were 1, return the sum for that row
            if row_count == 5 {
                return true;
            }
        }

        false
    }

    fn check_col(&self) -> bool {
        // Scan cols next
        for j in 0..self.positions[0].len() {
            let mut col_count = 0;
            for i in 0..self.positions.len() {
                if self.positions[i][j] == 1 {
                    col_count += 1;
                } else {
                    break;
                }
            }

            if col_count == 5 {
                return true;
            }
        }

        false
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    println!("Running day 4 part 1 of advent of code!");
    let f = File::open("day_4/src/input.txt").expect("Could not open file!");
    let mut lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // First line is the bingo input, parse it as such.
    let bingo_input = parse_bingo_input(lines[0].clone());
    println!("{:?}", bingo_input);

    // Clean our input now

    // Remove all empty lines
    lines.retain(|s| !s.is_empty());

    // Remove the first line since that's the input
    lines.remove(0);

    println!("Lines now: {}", lines.len());

    // Now parse the bingo boards themselves.
    let mut bingo_boards: Vec<Board> = Vec::new();
    for i in (0..lines.len()).step_by(5) {
        bingo_boards.push(parse_bingo_board(&lines[i..i + 5]));
    }

    // Helpful for debugging our parsing
    println!("Parsed {} bingo boards", bingo_boards.len());
    println!("First board: {:?}", bingo_boards[0]);
    println!("Last board: {:?}", bingo_boards[bingo_boards.len() - 1]);

    for num in bingo_input {
        for board in &mut bingo_boards {
            board.register_number(num);
            if board.check_win() {
                let sum = board.get_unmarked_sum();
                println!("Winning board product: {}", num * sum);
                println!("Winning board: {:?}", board);
                return;
            }
        }
    }
}

fn part_two() {
    println!("Running day 4 part 2 of advent of code!");
    let f = File::open("day_4/src/input.txt").expect("Could not open file!");
    let mut lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // First line is the bingo input, parse it as such.
    let bingo_input = parse_bingo_input(lines[0].clone());
    println!("{:?}", bingo_input);

    // Clean our input now

    // Remove all empty lines
    lines.retain(|s| !s.is_empty());

    // Remove the first line since that's the input
    lines.remove(0);

    println!("Lines now: {}", lines.len());

    // Now parse the bingo boards themselves.
    let mut bingo_boards: Vec<Board> = Vec::new();
    for i in (0..lines.len()).step_by(5) {
        bingo_boards.push(parse_bingo_board(&lines[i..i + 5]));
    }

    // Helpful for debugging our parsing
    println!("Parsed {} bingo boards", bingo_boards.len());
    println!("First board: {:?}", bingo_boards[0]);
    println!("Last board: {:?}", bingo_boards[bingo_boards.len() - 1]);

    let mut final_observed_winning_score = 0;
    let mut final_board: Option<Board> = None;
    for num in bingo_input {
        for board in &mut bingo_boards {
            // No need to register nums on already completed boards
            if board.is_already_completed() {
                continue;
            }

            board.register_number(num);
            if board.check_win() {
                let sum = board.get_unmarked_sum();
                final_observed_winning_score = sum * num;
                final_board.replace(board.clone());
            }
        }
    }

    println!("Last winning board product: {}", final_observed_winning_score);
    println!("Final winning board: {:?}", final_board);
}

// Helper function to hide away the input parsing boilerplate
fn parse_bingo_input(line: String) -> Vec<i32> {
    line.split(',')
        .map(|split| split.parse().expect("Could not parse string as i32"))
        .collect()
}

// Helper function to hide away the bingo board parsing boilerplate.
fn parse_bingo_board(lines: &[String]) -> Board {
    let mut board_mat: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Could not parse as i32"))
            .collect();
        board_mat.push(row);
    }

    Board::new(board_mat)
}
