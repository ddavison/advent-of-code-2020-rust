use crate::{Part, Part::One, Part::Two, read_lines};

#[cfg(test)]
mod tests;

struct Policy {
    required_letter: char,
    first_number: i32,
    second_number: i32
}

struct Password {
    password: String,
    policy: Policy,
    part: Part,
}

impl Password {
    fn is_valid(&self) -> bool {
        match self.part {
            One => self.has_required_letter() && self.not_too_few() && self.not_too_many(),
            Two => self.has_required_letter()
        }
    }

    fn has_required_letter(&self) -> bool {
        let mut has_one_character: bool = false;

        for (id, char) in self.password.chars().enumerate() {
            match self.part {
                One => if char == self.policy.required_letter { has_one_character = true },
                Two => {
                    let index = (id + 1) as i32;

                    if index == self.policy.first_number {
                        if char == self.policy.required_letter { has_one_character = !has_one_character }
                    } else if index == self.policy.second_number {
                        if char == self.policy.required_letter { has_one_character = !has_one_character }
                    }
                }
            }
        }

        has_one_character
    }

    fn not_too_few(&self) -> bool {
        self.password.chars().filter(|c| *c == self.policy.required_letter).count() as i32 >= self.policy.first_number
    }

    fn not_too_many(&self) -> bool {
        self.password.chars().filter(|c| *c == self.policy.required_letter).count() as i32 <= self.policy.second_number
    }
}

/// How many valid passwords are there?
/// [Day 2](https://adventofcode.com/2020/day/2)
pub(crate) fn day_2(part: Part, input: &str) -> i32 {
    let mut passwords: Vec<Password> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            let mut first_number: i32 = 0;
            let mut second_number: i32 = 0;
            let mut required_letter: char = '0';
            let password = line.rsplit(" ").next().unwrap();

            for (idx, part) in line.split(' ').enumerate() {
                match idx {
                    0 => { // first_number-second_number
                        first_number = part.split('-').nth(0).unwrap().parse::<i32>().unwrap();
                        second_number = part.split('-').nth(1).unwrap().parse::<i32>().unwrap();
                    },
                    1 => { // letter:
                        required_letter = part.chars().next().unwrap();
                    },
                    _ => {}
                }
            }

            let policy = Policy { required_letter, first_number, second_number };
            let password = match part {
                One => Password { password: password.to_owned(), policy, part: One},
                Two => Password { password: password.to_owned(), policy, part: Two}
            };

            passwords.push(password);
        }
    }

    passwords.iter().filter(|password| password.is_valid()).count() as i32
}