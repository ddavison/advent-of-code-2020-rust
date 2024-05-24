use crate::read_lines;

#[cfg(test)]
mod tests;

struct Policy {
    required_letter: char,
    min: i32,
    max: i32
}

struct Password {
    password: String,
    policy: Policy
}

impl Password {
    fn is_valid(&self) -> bool {
        self.has_required_letter() && self.not_too_few() && self.not_too_many()
    }

    fn has_required_letter(&self) -> bool {
        for char in self.password.chars() {
            if char == self.policy.required_letter { return true }
        }
        false
    }

    fn not_too_few(&self) -> bool {
        self.password.chars().filter(|c| *c == self.policy.required_letter).count() as i32 >= self.policy.min
    }

    fn not_too_many(&self) -> bool {
        self.password.chars().filter(|c| *c == self.policy.required_letter).count() as i32 <= self.policy.max
    }
}

/// How many valid passwords are there?
pub(crate) fn day_2(input: &str) -> i32 {
    let mut passwords: Vec<Password> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines.map_while(Result::ok) {
            let mut min: i32 = 0;
            let mut max: i32 = 0;
            let mut required_letter: char = '0';
            let password = line.rsplit(" ").next().unwrap();

            for (idx, part) in line.split(' ').enumerate() {
                match idx {
                    0 => { // min-max
                        min = part.split('-').nth(0).unwrap().parse::<i32>().unwrap();
                        max = part.split('-').nth(1).unwrap().parse::<i32>().unwrap();
                    },
                    1 => { // letter:
                        required_letter = part.chars().next().unwrap();
                    },
                    _ => {}
                }
            }

            let policy = Policy { required_letter, min, max };
            let password = Password { password: password.to_owned(), policy};

            passwords.push(password);
        }
    }

    passwords.iter().filter(|password| password.is_valid()).count() as i32
}