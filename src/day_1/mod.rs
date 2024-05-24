use crate::{Part, read_lines};

#[cfg(test)]
mod tests;

/// The amount that the two reports should factor to.
const SUM: i32 = 2020;

/// Day 1
///
/// [Advent of Code](https://adventofcode.com/2020/day/1)
pub(crate) fn day_1(part: Part, input_file: &str) -> i32 {
    let mut expenses: Vec<i32> = Vec::new();
    let mut factors: (i32, i32) = (0, 0);

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.map_while(Result::ok) {
            expenses.push(line.parse::<i32>().unwrap())
        }
    }

    for idx in 0..expenses.len() {
        match scan(expenses[idx], &expenses[idx + 1..]) {
            Some((a, b)) => factors = (a, b),
            None => continue
        }
    }

    factors.0 * factors.1
}

fn scan(start: i32, numbers: &[i32]) -> Option<(i32, i32)> {
    for number in numbers {
        if start + *number == SUM { return Some((start, *number)) }
    }

    None
}
