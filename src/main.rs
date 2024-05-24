use std::fs::File;
use std::{env, io};
use std::io::BufRead;
use std::path::Path;

mod day_1;
mod day_2;

enum Part {
    One,
    Two
}

pub fn read_lines<I>(input: I) -> io::Result<io::Lines<io::BufReader<File>>>
where I: AsRef<Path> {
    let file = File::open(input)?;
    Ok(io::BufReader::new(file).lines())
}

/// Run a day of Advent of Code
///
/// # Example
///
/// ```shell
/// $ ./aoc day_1
/// ```
fn main() {
    if let Some(day) = env::args().nth(2) {
        match day.as_str() {
            "day_1" => { day_1::day_1(Part::One, "src/day_1/data/input.txt"); },
            "day_2" => { day_2::day_2("src/day_2/data/input.txt"); },
            "day_3" => { day_1::day_1(Part::One, "src/day_1/data/input.txt"); },
            "day_4" => { day_1::day_1(Part::One, "src/day_1/data/input.txt"); },
            "day_5" => { day_1::day_1(Part::One, "src/day_1/data/input.txt"); },
            "day_6" => { day_1::day_1(Part::One, "src/day_1/data/input.txt"); },
            _ => {}
        }
    }
}
