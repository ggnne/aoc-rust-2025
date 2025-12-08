mod day01;
mod day03;

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One,
    Two,
}

pub trait AdventDay {
    fn solve_part1(&self, input: &str) -> String;
    fn solve_part2(&self, input: &str) -> String;
    fn solve(&self, part: Part, input: &str) -> String {
        match part {
            Part::One => self.solve_part1(input),
            Part::Two => self.solve_part2(input),
        }
    }
}

pub fn get_day(day: u8) -> Option<Box<dyn AdventDay>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        3 => Some(Box::new(day03::Day03)),
        _ => None,
    }
}
