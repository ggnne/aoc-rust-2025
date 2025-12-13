use crate::days::AdventDay;
use std::collections::{HashMap, HashSet};

pub struct Day07;


struct Manifold {
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    length: usize,
    width: usize,
}

impl Manifold {
    fn from_str(s: &str) -> Self {
        let mut start = (0, 0);
        let mut splitters: HashSet<(usize, usize)> = HashSet::new();
        let mut length = 0;
        let mut width = 0;
        for (i, line) in s.lines().enumerate() {
            length += 1;
            for (j, c) in line.char_indices() {
                width += 1;
                if c == 'S' {
                    start = (i, j);
                } else if c == '^' {
                    splitters.insert((i, j));
                }
            }
        }
        width /= length;
        Manifold {
            start,
            splitters,
            length,
            width,
        }
    }

    fn simulate(&self) -> (usize, usize) {
        let mut curr: HashMap<usize, usize> = HashMap::from([(self.start.1, 1)]);
        let mut splitters_count= 0;
        for i in 0..self.length {
            let mut next: HashMap<usize, usize> = HashMap::new();
            for (j, p) in curr.into_iter() {
                let mut v = Vec::with_capacity(2);
                if self.splitters.contains(&(i + 1, j)) {
                    splitters_count += 1;
                    if let Some(lj) = j.checked_sub(1) {
                        v.push(lj);
                    }
                    if j < self.width - 1 {
                        v.push(j + 1);
                    }
                } else {
                    v.push(j)
                }
                for j in v.into_iter() {
                    next.entry(j).and_modify(|e| *e += p).or_insert(p);
                }
            }
            curr = next;
        }
        (splitters_count, curr.into_values().sum())
    }
}

impl AdventDay for Day07 {
    fn solve_part1(&self, input: &str) -> String {
        let manifold = Manifold::from_str(input);
        let (splitters_count, _) = manifold.simulate();
        splitters_count.to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let manifold = Manifold::from_str(input);
        let (_, total_paths) = manifold.simulate();
        total_paths.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day07.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day07;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day07;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "40");
    }
}
