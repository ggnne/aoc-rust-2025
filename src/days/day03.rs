use crate::days::AdventDay;
use std::fmt;

pub struct Day03;

struct BatterySection {
    value: u8,
    digits: usize,
}

impl fmt::Display for BatterySection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.value, self.digits)
    }
}

impl BatterySection {
    fn new(value: u8, digits: usize) -> Self {
        BatterySection { value, digits }
    }

    fn up(&mut self, times: usize) {
        self.digits += times;
    }

    fn down(&mut self, times: usize) {
        self.digits = self.digits.saturating_sub(times);
    }
    fn to_number(&self) -> usize {
        let mut number = 0usize;
        for i in 0..self.digits {
            number += self.value as usize * 10usize.pow(i as u32);
        }
        number
    }
}

struct Battery {
    sections: Vec<BatterySection>,
    digits: usize,
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for bs in self.sections.iter() {
            write!(f, "{bs}")?;
        }
        Ok(())
    }
}

impl Battery {
    fn new() -> Self {
        Battery {
            sections: vec![],
            digits: 0,
        }
    }

    fn add(&mut self, section: BatterySection) {
        self.digits += section.digits;
        self.sections.push(section);
    }

    fn up(&mut self, index: usize, times: usize) {
        if let Some(bs) = self.sections.get_mut(index) {
            self.digits += times;
            bs.up(times);
        }
    }

    fn get_at(&self, index: usize) -> Option<&BatterySection> {
        self.sections.get(index)
    }

    fn get_value_at(&self, index: usize) -> Option<&u8> {
        self.get_at(index).map(|bs| &bs.value)
    }

    fn from_str(s: &str) -> Self {
        let mut battery = Battery::new();
        for n in s.chars().filter_map(|c| c.to_digit(10).map(|n| n as u8)) {
            let index = battery.sections.len().saturating_sub(1);
            if battery.get_value_at(index) == Some(&n) {
                battery.up(index, 1);
            } else {
                battery.add(BatterySection::new(n, 1));
            }
        }
        battery
    }
    fn normalize(&mut self) {
        let mut new_sections: Vec<BatterySection> = vec![];
        let mut new_digits = 0;
        for bs in self.sections.iter().filter(|&bs| bs.digits > 0) {
            if let Some(last) = new_sections.last_mut()
                && last.value == bs.value
            {
                last.up(bs.digits);
            } else {
                new_sections.push(BatterySection::new(bs.value, bs.digits));
            }
            new_digits += bs.digits;
        }
        self.sections = new_sections;
        self.digits = new_digits;
    }

    fn highest_lower(&mut self, digits: usize) {
        while self.digits > digits {
            let mut changed = false;
            for i in 1..self.sections.len() {
                if self.sections[i - 1].value < self.sections[i].value {
                    self.sections[i - 1].down(1);
                    changed = true;
                    break;
                }
            }
            if !changed && let Some(last) = self.sections.last_mut() {
                last.down(1);
            }
            self.normalize();
        }
    }

    fn to_number(&self) -> usize {
        let mut number = 0;
        let mut digits = 0;
        for bs in self.sections.iter().rev() {
            number += bs.to_number() * 10usize.pow(digits);
            digits += bs.digits as u32;
        }
        number
    }
}

impl AdventDay for Day03 {
    fn solve_part1(&self, input: &str) -> String {
        let mut out = 0;
        for line in input.lines() {
            let mut battery = Battery::from_str(line);
            battery.highest_lower(2);
            out += battery.to_number();
        }
        out.to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut out = 0;
        for line in input.lines() {
            let mut battery = Battery::from_str(line);
            battery.highest_lower(12);
            out += battery.to_number();
        }
        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day03.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day03;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "357");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day03;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "3121910778619");
    }
}
