use crate::days::AdventDay;
use core::fmt;
use std::collections::{HashMap, HashSet};

pub struct Day04;

fn neighbours(coord: &(usize, usize)) -> HashSet<(usize, usize)> {
    let (i, j) = *coord;
    let directions: [(i8, i8); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut set = HashSet::with_capacity(8);

    let start_i = i as i32;
    let start_j = j as i32;

    for (di, dj) in directions {
        let ni = start_i + di as i32;
        let nj = start_j + dj as i32;

        if ni >= 0 && nj >= 0 {
            set.insert((ni as usize, nj as usize));
        }
    }
    set
}

type AdjList = HashMap<(usize, usize), HashSet<(usize, usize)>>;

#[derive(Debug)]
struct Grid {
    n: usize,
    m: usize,
    papers: AdjList,
}

impl Grid {

    fn from_str(s: &str) -> Self {
        let mut papers_set = HashSet::new();
        let mut n = 0;
        let mut m = 0;
        for (i, line) in s.lines().enumerate() {
            n += 1;
            for (j, c) in line.char_indices() {
                if c == '@' {
                    papers_set.insert((i, j));
                }
                m += 1;
            }
        }
        n += 1;
        m += 1;

        let mut papers: AdjList = HashMap::with_capacity(papers_set.len());
        for paper in papers_set.iter() {
            let neighbours: HashSet<(usize, usize)> = neighbours(paper)
                .iter()
                .filter(|p| papers_set.contains(p))
                .cloned()
                .collect();
            papers.insert(*paper, neighbours);
        }
        Grid { n, m, papers }
    }

    fn removable(&self) -> HashSet<(usize, usize)> {
        self.papers
            .iter()
            .filter(|&(_, v)| v.len() < 4)
            .map(|(k, _)| *k)
            .collect()
    }

    fn remove(&mut self, paper: &(usize, usize)) {
        if let Some(neighbours) = self.papers.get(paper).cloned() {
            for neighbour in neighbours.iter() {
                if let Some(e) = self.papers.get_mut(neighbour) {
                    e.remove(paper);
                }
            }
            self.papers.remove(paper);
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                let c = if self.papers.contains_key(&(i, j)) {
                    '@'
                } else {
                    '.'
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl AdventDay for Day04 {
    fn solve_part1(&self, input: &str) -> String {
        let grid = Grid::from_str(input);
        grid.removable().len().to_string()
    }
    fn solve_part2(&self, input: &str) -> String {
        let mut out = 0;
        let mut grid = Grid::from_str(input);
        let mut removable = grid.removable();
        loop {
            if removable.is_empty() {
                break;
            }
            out += removable.len();
            for r in removable {
                grid.remove(&r);
            }
            removable = grid.removable();
        }
        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day04.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day04;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day04;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "43");
    }
}
