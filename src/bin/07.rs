use std::collections::{BTreeSet, HashMap};

advent_of_code::solution!(7);

pub fn find_char_indices(row: &str, target: char) -> Vec<usize> {
    let mut indices = vec![];

    for (i, c) in row.char_indices() {
        if c == target {
            indices.push(i);
        }
    }

    indices
}

fn count_timelines(
    pos: usize,
    row: usize,
    splitters: &[Vec<usize>],
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if row >= splitters.len() {
        return 1;
    }

    if let Some(&cached) = cache.get(&(row, pos)) {
        return cached;
    }

    let result = if splitters[row].contains(&pos) {
        let left = count_timelines(pos - 1, row + 1, splitters, cache);
        let right = count_timelines(pos + 1, row + 1, splitters, cache);
        left + right
    } else {
        count_timelines(pos, row + 1, splitters, cache)
    };

    cache.insert((row, pos), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let mut lines = input_str.lines();
    let mut split_count = 0;
    let mut beam_indices: Vec<usize> = find_char_indices(lines.next()?, 'S');

    for line in lines {
        let splitter_indices = find_char_indices(line, '^');

        if splitter_indices.is_empty() {
            continue;
        }

        let mut updated_beam_indices: BTreeSet<usize> = BTreeSet::new();

        for b in beam_indices.clone().iter() {
            if splitter_indices.contains(b) {
                split_count += 1;
                updated_beam_indices.insert(b - 1);
                updated_beam_indices.insert(b + 1);
            } else {
                updated_beam_indices.insert(*b);
            }
        }

        beam_indices = updated_beam_indices.iter().cloned().collect::<Vec<usize>>();
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let lines: Vec<&str> = input_str.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let start_pos = find_char_indices(lines[0], 'S')[0];

    let splitters: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| find_char_indices(line, '^'))
        .collect();

    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

    Some(count_timelines(start_pos, 0, &splitters, &mut cache))
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
