use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug)]
pub enum Cephaloperator {
    ADD,
    MULTIPLY,
}

impl FromStr for Cephaloperator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Cephaloperator::ADD),
            "*" => Ok(Cephaloperator::MULTIPLY),
            _ => Err("Invalid Cephaloperation"),
        }
    }
}

pub fn apply_cephaloperation(cephaloperands: &Vec<u64>, cephaloperator: &Cephaloperator) -> u64 {
    match cephaloperator {
        Cephaloperator::ADD => cephaloperands.iter().sum(),
        Cephaloperator::MULTIPLY => cephaloperands.iter().product(),
    }
}

pub fn parse_cephaloperations_by_row(input: &str) -> (Vec<Vec<u64>>, Vec<Cephaloperator>) {
    let mut input_lines = input.lines().peekable();

    let mut input_matrix: Vec<Vec<u64>> = vec![];
    let mut op_vec: Vec<Cephaloperator> = vec![];

    while let Some(line) = input_lines.next() {
        if input_lines.peek().is_none() {
            op_vec = line
                .split(" ")
                .filter_map(|op_str| op_str.parse::<Cephaloperator>().ok())
                .collect();
        } else {
            let nums: Vec<u64> = line
                .split(" ")
                .filter_map(|num_str| num_str.parse::<u64>().ok())
                .collect();

            if !input_matrix.is_empty() {
                for (idx, num) in nums.iter().enumerate() {
                    input_matrix[idx].push(*num);
                }
            } else {
                for num in nums.iter() {
                    input_matrix.push(vec![*num]);
                }
            }
        }
    }

    (input_matrix, op_vec)
}

pub fn parse_cephaloperations_by_col(input: &str) -> Option<(Vec<Vec<u64>>, Vec<Cephaloperator>)> {
    let input_mat_unparsed: Vec<&str> = input.lines().collect();
    let num_inputs_len = input_mat_unparsed.len() - 1;
    let ops_unparsed = input_mat_unparsed.last()?;

    let mut input_matrix: Vec<Vec<u64>> = vec![];
    let mut op_vec: Vec<Cephaloperator> = vec![];

    let mut curr_inputs: Vec<String> = vec![];

    for (rev_idx, op_char) in ops_unparsed.chars().rev().enumerate() {
        let c = ops_unparsed.len() - rev_idx;
        for r in 0..num_inputs_len {
            match curr_inputs.last_mut() {
                Some(curr_input) => {
                    *curr_input = format!(
                        "{}{}",
                        *curr_input,
                        input_mat_unparsed[r].chars().nth(c - 1)?
                    )
                }
                None => (),
            }
        }
        match op_char.to_string().parse::<Cephaloperator>() {
            Ok(op) => {
                op_vec.push(op);
                input_matrix.push(
                    curr_inputs
                        .iter()
                        .filter_map(|s| s.trim().parse::<u64>().ok())
                        .collect::<Vec<u64>>(),
                );
                curr_inputs.clear();
            }
            Err(_) => curr_inputs.push(String::new()),
        }
    }

    Some((input_matrix, op_vec))
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let (parsed_operands, parsed_operators) = parse_cephaloperations_by_row(input);

    if parsed_operands.len() != parsed_operators.len() {
        return None;
    }

    let op_results: Vec<u64> = parsed_operands
        .iter()
        .zip(parsed_operators.iter())
        .map(|(cephaloperands, cephaloperator)| {
            apply_cephaloperation(cephaloperands, cephaloperator)
        })
        .collect();

    Some(op_results.iter().fold(0, |total, res| total + res))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_str = input.trim();
    if input_str.is_empty() {
        return None;
    }

    let (parsed_operands, parsed_operators) = parse_cephaloperations_by_col(input)?;

    if parsed_operands.len() != parsed_operators.len() {
        return None;
    }

    let op_results: Vec<u64> = parsed_operands
        .iter()
        .zip(parsed_operators.iter())
        .map(|(cephaloperands, cephaloperator)| {
            apply_cephaloperation(cephaloperands, cephaloperator)
        })
        .collect();

    Some(op_results.iter().fold(0, |total, res| total + res))
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
