use crate::days::AdventDay;

pub struct Day01;

#[derive(Debug)]
enum Command {
    L(usize),
    R(usize),
}

impl Command {
    fn from_str(s: &str) -> Self {
        if s.starts_with("L") {
            let v = s.trim_start_matches('L').parse::<usize>().unwrap();
            Command::L(v)
        } else {
            let v = s.trim_start_matches('R').parse::<usize>().unwrap();
            Command::R(v)
        }
    }
    fn extract_rotations(&mut self, max: usize) -> usize {
        let div = max + 1;
        match self {
            Command::L(v) | Command::R(v) => {
                let times = *v / div;
                *v = v.rem_euclid(div);
                times
            }
        }
    }
}

struct Dial {
    arrow: usize,
    max: usize,
}

fn wrapping_sub(a: usize, b: usize, m: usize) -> usize {
    (a as isize - b as isize).rem_euclid(m as isize) as usize
}

fn wrapping_add(a: usize, b: usize, m: usize) -> usize {
    (a + b).rem_euclid(m)
}

impl Dial {
    fn new(arrow: usize, max: usize) -> Self {
        Dial { arrow, max }
    }

    fn pass_through_zero(&self, command: &Command) -> bool {
        match *command {
            Command::L(v) => self.arrow != 0 && self.arrow < v,
            Command::R(v) => self.arrow + v > (self.max + 1),
        }
    }

    fn rotate(&mut self, command: &Command) {
        match *command {
            Command::L(v) => {
                self.arrow = wrapping_sub(self.arrow, v, self.max + 1);
            }
            Command::R(v) => {
                self.arrow = wrapping_add(self.arrow, v, self.max + 1);
            }
        }
    }
    fn arrow(&self) -> usize {
        self.arrow
    }
    fn max(&self) -> usize {
        self.max
    }
}

impl AdventDay for Day01 {
    fn solve_part1(&self, input: &str) -> String {
        let mut out = 0;
        let mut dial = Dial::new(50, 99);
        for line in input.lines() {
            let command = Command::from_str(line);
            dial.rotate(&command);
            out += (dial.arrow() == 0) as usize;
        }
        out.to_string()
    }
    fn solve_part2(&self, input: &str) -> String {
        let mut out = 0;
        let mut dial = Dial::new(50, 99);
        for line in input.lines() {
            let mut command = Command::from_str(line);
            out += command.extract_rotations(dial.max());
            out += (dial.pass_through_zero(&command)) as usize;
            dial.rotate(&command);
            out += (dial.arrow() == 0) as usize;
        }
        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day01.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day01;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day01;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "6");
    }
}
