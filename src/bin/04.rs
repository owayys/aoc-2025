advent_of_code::solution!(4);

const ADJACENTS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adjacent(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    ADJACENTS
        .iter()
        .filter(|&&(dr, dc)| {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            (0..height).contains(&nr)
                && (0..width).contains(&nc)
                && (grid[nr as usize][nc as usize] == '@' || grid[nr as usize][nc as usize] == 'r')
        })
        .count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let roll_grid: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = roll_grid.len();
    let width = roll_grid[0].len();

    let mut valid_rolls = 0;

    for row in 0..height {
        for col in 0..width {
            if roll_grid[row][col] != '@' {
                continue;
            }

            let adjacent_rolls = count_adjacent(&roll_grid, row, col);

            if adjacent_rolls < 4 {
                valid_rolls += 1;
            }
        }
    }

    Some(valid_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let mut roll_grid: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = roll_grid.len();
    let width = roll_grid[0].len();

    let mut rolls_removable = true;

    let mut rolls_removed = 0;

    while rolls_removable {
        let mut rolls_removed_in_turn = 0;

        for row in roll_grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 'r' {
                    *cell = 'x';
                }
            }
        }

        for row in 0..height {
            for col in 0..width {
                if roll_grid[row][col] != '@' {
                    continue;
                }

                let adjacent_rolls = count_adjacent(&roll_grid, row, col);

                if adjacent_rolls < 4 {
                    roll_grid[row][col] = 'r';
                    rolls_removed_in_turn += 1;
                }
            }
        }

        if rolls_removed_in_turn == 0 {
            rolls_removable = false;
        } else {
            rolls_removed += rolls_removed_in_turn;
        }
    }

    Some(rolls_removed)
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
