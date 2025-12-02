use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
pub struct IDRange {
    from: u64,
    to: u64,
}

impl FromStr for IDRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("empty string");
        }

        let (from, to) = match s.split_once("-") {
            Some((from, to)) => (from.parse::<u64>(), to.parse::<u64>()),
            None => return Err("invalid range"),
        };

        match (from, to) {
            (Ok(f), Ok(t)) => Ok(IDRange { from: f, to: t }),
            _ => Err("invalid range"),
        }
    }
}
pub fn is_id_invalid_1(id: u64) -> bool {
    let s = id.to_string();
    let b = s.as_bytes();
    let len = b.len();

    len % 2 == 0 && &b[..len / 2] == &b[len / 2..]
}

pub fn is_id_invalid_2(id: u64) -> bool {
    let s = id.to_string();
    let ss = s.clone() + &s;
    ss[1..ss.len() - 1].contains(&s)
}

pub fn extract_invalid(range: &IDRange, invalid_fn: fn(u64) -> bool) -> Vec<u64> {
    (range.from..range.to)
        .filter(|&id| invalid_fn(id))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let id_ranges_result: Result<Vec<IDRange>, &str> =
        input_str.split(",").map(|s| IDRange::from_str(s)).collect();

    match id_ranges_result {
        Ok(id_ranges) => {
            let invalid_ids: Vec<u64> = id_ranges
                .iter()
                .flat_map(|r| extract_invalid(r, is_id_invalid_1))
                .collect();
            Some(invalid_ids.into_iter().fold(0, |acc, x| acc + x))
        }
        Err(_) => None,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let id_ranges_result: Result<Vec<IDRange>, &str> =
        input_str.split(",").map(|s| IDRange::from_str(s)).collect();

    match id_ranges_result {
        Ok(id_ranges) => {
            let invalid_ids: Vec<u64> = id_ranges
                .iter()
                .flat_map(|r| extract_invalid(r, is_id_invalid_2))
                .collect();
            Some(invalid_ids.into_iter().fold(0, |acc, x| acc + x))
        }
        Err(_) => None,
    }
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
