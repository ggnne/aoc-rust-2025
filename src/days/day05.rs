use crate::days::AdventDay;
use std::collections::BTreeMap;

pub struct Day05;

fn parse_input(input: &str) -> (&str, &str) {
    let mut parts = input.split("\n\n");
    (parts.next().unwrap(), parts.next().unwrap())
}

type Range = (usize, usize);

fn merge_ranges(r1: Range, r2: Range) -> Vec<Range> {
    let (r1, r2) = if r1.0 <= r2.0 { (r1, r2) } else { (r2, r1) };
    if r1.1 >= r2.0 {
        vec![(r1.0.min(r2.0), r1.1.max(r2.1))]
    } else {
        vec![r1, r2]
    }
}

#[derive(Debug)]
struct Database {
    map: BTreeMap<usize, usize>,
}

impl Database {
    fn from_str(s: &str) -> Self {
        let mut ranges = vec![];
        for line in s.lines().map(|l| l.trim()) {
            let mut lb_ub = line.split("-").filter_map(|n| n.parse::<usize>().ok());
            if let (Some(lb), Some(ub)) = (lb_ub.next(), lb_ub.next()) {
                ranges.push((lb, ub));
            }
        }
        ranges.sort();
        let mut stack = vec![];
        for rng in ranges.into_iter() {
            if stack.is_empty() {
                stack.push(rng);
            } else {
                let last = stack.pop().unwrap();
                stack.extend(merge_ranges(last, rng));
            }
        }

        let mut map = BTreeMap::new();
        for (lb, ub) in stack.into_iter() {
            map.insert(lb, ub);
        }

        Database { map }
    }

    fn is_fresh(&self, item: usize) -> bool {
        self.map.range(..=item).any(|(_, &v)| v >= item)
    }

    fn how_many_fresh(&self) -> usize {
        self.map.iter().map(|(&k, &v)| v - k + 1).sum()
    }
}

impl AdventDay for Day05 {
    fn solve_part1(&self, input: &str) -> String {
        let (db_str, items_str) = parse_input(input);
        let db = Database::from_str(db_str);
        let items: Vec<usize> = items_str
            .lines()
            .filter_map(|l| l.trim().parse::<usize>().ok())
            .collect();
        items
            .into_iter()
            .filter(|&item| db.is_fresh(item))
            .count()
            .to_string()
    }
    fn solve_part2(&self, input: &str) -> String {
        let (db_str, _) = parse_input(input);
        let db = Database::from_str(db_str);
        db.how_many_fresh().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day05.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day05;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day05;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "14");
    }
}
