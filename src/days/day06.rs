use std::vec;

use crate::days::AdventDay;

pub struct Day06;

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s.trim() {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => unreachable!(),
        }
    }
}

fn parse_input_part1(input: &str) -> (Vec<Operation>, Vec<Vec<usize>>) {
    let mut lines: Vec<&str> = input.lines().collect();
    let operations: Vec<Operation> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(Operation::from_str)
        .collect();
    let interim: Vec<Vec<usize>> = lines
        .iter()
        .map(|&l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect()
        })
        .collect();
    let n = interim.len();
    let m = interim[0].len();
    let numbers: Vec<Vec<usize>> = (0..m)
        .map(|j| (0..n).map(|i| interim[i][j]).collect())
        .collect();
    (operations, numbers)
}

fn parse_input_part2(input: &str) -> (Vec<Operation>, Vec<Vec<usize>>) {
    let mut lines: Vec<&str> = input.lines().collect();
    let operations: Vec<Operation> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(Operation::from_str)
        .collect();
    let chars: Vec<Vec<char>> = lines.iter().map(|&l| l.chars().collect()).collect();
    let mut numbers: Vec<Vec<usize>> = vec![vec![]];
    let n = chars.len();
    let m = chars[0].len();
    for j in 0..m {
        let s: String = (0..n)
            .filter_map(|i| {
                if chars[i][j] != ' ' {
                    Some(chars[i][j])
                } else {
                    None
                }
            })
            .collect();
        if s.is_empty() {
            numbers.push(vec![]);
            continue;
        }
        if let Some(last) = numbers.last_mut() {
            last.push(s.parse::<usize>().unwrap());
        }
    }
    (operations, numbers)
}

fn calculate(operations: &[Operation], numbers: &[Vec<usize>]) -> usize {
    let outputs: Vec<usize> = operations
        .iter()
        .enumerate()
        .map(|(j, op)| match &op {
            Operation::Add => numbers[j].iter().sum(),
            Operation::Mul => numbers[j].iter().product(),
        })
        .collect();
    outputs.into_iter().sum()
}

impl AdventDay for Day06 {
    fn solve_part1(&self, input: &str) -> String {
        let (operations, numbers) = parse_input_part1(input);
        calculate(&operations, &numbers).to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let (operations, numbers) = parse_input_part2(input);
        calculate(&operations, &numbers).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day06.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day06;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "4277556");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day06;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "3263827");
    }
}
