use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<i32>>,
}

impl Map {
    fn new(x: usize, y: usize) -> Self {
        Self {
            grid: vec![vec![0; y]; x],
        }
    }

    fn write_vertical_line_to_map(&mut self, line: &Line) {
        let y1 = line.start.1;
        let y2 = line.end.1;
        let range = if y1 > y2 { y2..=y1 } else { y1..=y2 };

        let x = line.start.0 as usize;
        for y in range {
            let y = y as usize;
            self.grid[x][y] += 1;
        }
    }

    fn write_horizontal_line_to_map(&mut self, line: &Line) {
        let x1 = line.start.0;
        let x2 = line.end.0;
        let range = if x1 > x2 { x2..=x1 } else { x1..=x2 };

        let y = line.start.1 as usize;
        for x in range {
            let x = x as usize;
            self.grid[x][y] += 1;
        }
    }

    fn write_diagonal_line_to_map(&mut self, line: &Line) {
        let slope = (line.end.1 - line.start.1) / (line.end.0 - line.start.0);
        let y_int = line.start.1 - (slope * line.start.0);

        let x1 = line.start.0;
        let x2 = line.end.0;
        let range = if x1 > x2 { x2..=x1 } else { x1..=x2 };

        for x in range {
            let y = (slope * x) + y_int;
            self.grid[x as usize][y as usize] += 1;
        }
    }

    fn get_dangerous_zones(&self) -> i32 {
        let mut zones = 0;
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                if self.grid[x][y] > 1 {
                    zones += 1;
                }
            }
        }

        zones
    }
}

fn main() {
    // Only difference between these two is that part_one filters out diagonal lines
    part_one();
    part_two();
}

fn part_two() {
    println!("Running day 5 part 2 of advent of code!");
    let line_vec = parse_input_file();
    let dimensions_of_map = get_dimensions_of_map(&line_vec);
    let mut map = Map::new(dimensions_of_map.0, dimensions_of_map.1);
    write_lines_to_map(&mut map, &line_vec);
    println!(
        "There are {} dangerous zones in the map!",
        map.get_dangerous_zones()
    )
}

fn part_one() {
    println!("Running day 5 part 1 of advent of code!");
    // Generate the input file into a vector of lines, and then retain only the horizontal and vertical lines
    let mut line_vec = parse_input_file();
    line_vec.retain(|l| l.is_horizontal() || l.is_vertical());
    let dimensions_of_map = get_dimensions_of_map(&line_vec);
    let mut map = Map::new(dimensions_of_map.0, dimensions_of_map.1);
    write_lines_to_map(&mut map, &line_vec);
    println!(
        "There are {} dangerous zones in the map!",
        map.get_dangerous_zones()
    )
}

fn write_lines_to_map(map: &mut Map, line_vec: &Vec<Line>) {
    for line in line_vec {
        if line.is_horizontal() {
            map.write_horizontal_line_to_map(line);
        } else if line.is_vertical() {
            map.write_vertical_line_to_map(line);
        } else {
            map.write_diagonal_line_to_map(line);
        }
    }
}

fn get_dimensions_of_map(line_vec: &Vec<Line>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in line_vec {
        let x = line.start.0.max(line.end.0);
        let y = line.start.1.max(line.end.1);

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }
    }

    ((max_x + 1) as usize, (max_y + 1) as usize)
}

fn parse_input_file() -> Vec<Line> {
    let f = File::open("day_5/src/input.txt").expect("Couldn't open input.txt!");
    let raw_lines: Vec<String> = io::BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line in input file!"))
        .collect();

    let mut line_vec: Vec<Line> = Vec::new();

    for line in raw_lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        if points.len() != 2 {
            panic!(
                "points in line {} had a length not equal to 2! Len was {}, points was {:?}",
                line,
                points.len(),
                points
            );
        }

        let mut start_str_split = points[0].split(',');
        let mut end_str_split = points[1].split(',');

        line_vec.push(Line {
            start: (
                start_str_split
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Couldn't parse"),
                start_str_split
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Couldn't parse"),
            ),
            end: (
                end_str_split
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Couldn't parse"),
                end_str_split
                    .next()
                    .unwrap()
                    .parse()
                    .expect("Couldn't parse"),
            ),
        });
    }

    line_vec
}
