use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug, PartialEq)]
pub enum Command {
    L(u64),
    R(u64),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty string");
        }

        let (prefix, rest) = s.split_at(1);
        let n: u64 = rest.parse().map_err(|_| "invalid number")?;

        match prefix {
            "L" => Ok(Command::L(n)),
            "R" => Ok(Command::R(n)),
            _ => Err("invalid prefix"),
        }
    }
}

pub fn apply(state: u64, command: Command) -> (u64, u64) {
    match command {
        Command::R(distance) => ((state + distance) % 100, (state + distance) / 100),
        Command::L(distance) => (
            (state + 100 - (distance % 100)) % 100,
            if distance >= state {
                ((distance - state) / 100) + (state > 0) as u64
            } else {
                0
            },
        ),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    if input.trim().is_empty() {
        return None;
    }

    let mut state = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let command = Command::from_str(line).unwrap();
        let (new_state, _) = apply(state, command);
        if new_state == 0 {
            zeros += 1;
        }
        state = new_state;
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    if input.trim().is_empty() {
        return None;
    }

    let mut state = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let command = Command::from_str(line).ok()?;
        let (new_state, count) = apply(state, command);
        zeros += count;
        state = new_state;
    }

    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
