use crate::{Part, Part::One, Part::Two, read_lines};

#[cfg(test)]
mod tests;

/// The amount that the two reports should factor to.
const SUM: i32 = 2020;

/// Day 1
///
/// [Advent of Code](https://adventofcode.com/2020/day/1)
pub(crate) fn day_1(part: Part, input_file: &str) -> i32 {
    let mut expenses: Vec<i32> = Vec::new();
    let mut factors: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.map_while(Result::ok) {
            expenses.push(line.parse::<i32>().unwrap())
        }
    }

    for idx in 0..expenses.len() {
        match scan(expenses[idx], &part, &expenses[idx + 1..]) {
            Some(v) => v.iter().for_each(|f| factors.push(*f)),
            None => continue
        }
    }

    factors.iter().product()
}

fn scan(start: i32, part: &Part, numbers: &[i32]) -> Option<Vec<i32>> {
    match part {
        One => {
            for number in numbers {
                if start + *number == SUM { return Some(vec![start, *number]) }
            }
        },
        Two => {
            for first in numbers {
                for second in &numbers[1..] {
                    if start + first + second == SUM { return Some(vec![start, *first, *second]) }
                }
            }
        }
    }

    None
}
