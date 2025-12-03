use std::vec;

advent_of_code::solution!(3);

pub fn largest(nums: Vec<u64>) -> Option<u64> {
    nums.iter().max().copied()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let mut total_jolts = 0;

    for bank in input_str.lines() {
        let batteries: Vec<u64> = bank
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u64)
            .collect();

        let first_battery = largest(batteries[0..batteries.len() - 1].to_vec())?;
        let first_battery_idx = bank.find(&first_battery.to_string())?;
        let rest_batteries: Vec<u64> = batteries[first_battery_idx + 1..].to_vec();
        let second_battery = largest(rest_batteries)?;

        let max_jolts = format!("{}{}", first_battery, second_battery)
            .parse::<u64>()
            .expect("Failed to parse string to u64");

        total_jolts += max_jolts;
    }

    Some(total_jolts)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let mut total_jolts = 0;

    for bank in input_str.lines() {
        let batteries: Vec<u64> = bank
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u64)
            .collect();

        let mut highest_joltage: Vec<u64> = vec![];
        let mut prev_battery_idx = 0;

        for idx in 0..12 {
            let largest_battery =
                largest(batteries[prev_battery_idx..batteries.len() - (11 - idx)].to_vec())?;
            highest_joltage.push(largest_battery);
            prev_battery_idx =
                bank[prev_battery_idx..].find(&largest_battery.to_string())? + prev_battery_idx + 1;
        }

        let max_jolts_str: String = highest_joltage.iter().map(|n| n.to_string()).collect();

        let max_jolts = max_jolts_str
            .parse::<u64>()
            .expect("Failed to parse string to u64");

        total_jolts += max_jolts;
    }

    Some(total_jolts)
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
