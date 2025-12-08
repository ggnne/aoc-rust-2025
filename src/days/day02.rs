use crate::days::AdventDay;

pub struct Day02;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }
    fn from_str(s: &str) -> Self {
        let mut it = s.trim().split("-").filter_map(|n| n.parse::<usize>().ok());
        let (start, end) = (it.next().unwrap(), it.next().unwrap());
        Range::new(start, end)        
    }
    fn sum_invalid<F>(&self, f: F) -> usize 
    where 
        F: Fn(&usize) -> bool
    {
        (self.start..=self.end).filter(f).sum()
    }
}


fn repeats_twice(number: &usize) -> bool {
    let s = number.to_string();
    let len = s.len();
    if len % 2 == 0 {
        let half_len = len / 2;
        let div = 10usize.pow(half_len as u32);
        (*number / div) == (*number % div)
    } else {
        false
    }
}

fn repeats_at_least_twice(number: &usize) -> bool {
    let s = number.to_string();
    let len = s.len();

    for k in 1..=(len / 2) { 
        
        if len % k != 0 {
            continue;
        }

        let base_sequence = &s[0..k];
        let num_repetitions = len / k;
        
        let mut matches = true;        
        for r in 1..num_repetitions {
            let start_index = r * k;
            let end_index = (r + 1) * k;
            let current_segment = &s[start_index..end_index];
            
            if current_segment != base_sequence {
                matches = false;
                break;
            }
        }

        if matches {
            return true;
        }
    }
    
    false
}

impl AdventDay for Day02 {
    fn solve_part1(&self, input: &str) -> String {
        input.split(",").map(|s| Range::from_str(s).sum_invalid(repeats_twice)).sum::<usize>().to_string()
    }
    fn solve_part2(&self, input: &str) -> String {
        input.split(",").map(|s| Range::from_str(s).sum_invalid(repeats_at_least_twice)).sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/day02.txt"));

    #[test]
    fn solve_example_part1() {
        let day = Day02;
        let result = day.solve_part1(INPUT);
        assert_eq!(result, "1227775554");
    }

    #[test]
    fn solve_example_part2() {
        let day = Day02;
        let result = day.solve_part2(INPUT);
        assert_eq!(result, "4174379265");
    }
}
