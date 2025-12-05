use std::cmp;

advent_of_code::solution!(5);

#[derive(Debug, Copy, PartialEq)]
pub struct FreshRange {
    from: u64,
    to: u64,
}

impl FreshRange {
    pub fn num_ids(&self) -> u64 {
        self.to - self.from + 1
    }
}

impl Clone for FreshRange {
    fn clone(&self) -> Self {
        FreshRange {
            from: self.from,
            to: self.to,
        }
    }
}

pub fn resolve_ranges(ranges: &[FreshRange]) -> Vec<FreshRange> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut sorted = ranges.to_vec();
    sorted.sort_unstable_by_key(|r| r.from);

    let mut merged = vec![sorted[0]];

    for &current in &sorted[1..] {
        let last = merged.last_mut().unwrap();

        if current.from <= last.to + 1 {
            last.to = cmp::max(last.to, current.to);
        } else {
            merged.push(current);
        }
    }

    merged
}

pub fn is_fresh(num: u64, ranges: &[FreshRange]) -> bool {
    if ranges.is_empty() {
        return false;
    }

    ranges
        .binary_search_by(|range| {
            if num < range.from {
                cmp::Ordering::Greater
            } else if num > range.to {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Equal
            }
        })
        .is_ok()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let mut split_input_str = input_str.split("\n\n");
    let ranges_str = split_input_str.next()?;
    let tests_str = split_input_str.next()?;

    let mut parsed_ranges: Vec<FreshRange> = vec![];

    for range_line in ranges_str.lines() {
        let split_range: Vec<&str> = range_line.split("-").collect();
        let from = split_range[0].parse::<u64>().ok()?;
        let to = split_range[1].parse::<u64>().ok()?;
        parsed_ranges.push(FreshRange { from, to })
    }

    let ranges = resolve_ranges(&mut parsed_ranges);

    let mut fresh_ingredients = 0;

    for test_line in tests_str.lines() {
        let num = test_line.parse::<u64>().ok()?;

        if is_fresh(num, &ranges) {
            fresh_ingredients += 1;
        }
    }

    Some(fresh_ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let split_input_str: Vec<&str> = input_str.split("\n\n").collect();
    let ranges_str = split_input_str[0];

    let mut parsed_ranges: Vec<FreshRange> = vec![];

    for range_line in ranges_str.lines() {
        let split_range: Vec<&str> = range_line.split("-").collect();
        let from = split_range[0].parse::<u64>().ok()?;
        let to = split_range[1].parse::<u64>().ok()?;
        parsed_ranges.push(FreshRange { from, to })
    }

    let ranges = resolve_ranges(&mut parsed_ranges);

    let total_fresh = ranges.iter().fold(0, |total, r| total + r.num_ids());

    Some(total_fresh)
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
