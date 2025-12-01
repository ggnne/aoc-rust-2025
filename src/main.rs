use clap::Parser;
use std::time::Instant;
use std::{fs, path::PathBuf};

mod days;

fn validate_day(s: &str) -> Result<u8, String> {
    let day: u8 = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid number"))?;
    if (1..=12).contains(&day) {
        Ok(day)
    } else {
        Err(format!("Day must be between 1 and 12, but got {day}"))
    }
}

fn validate_part(s: &str) -> Result<u8, String> {
    let part: u8 = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid number"))?;
    if part == 1 || part == 2 {
        Ok(part)
    } else {
        Err(format!("Part must be 1 or 2, but got {part}"))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = validate_day)]
    day: u8,
    #[arg(short, long, default_value_t = 1, value_parser = validate_part)]
    part: u8,
    #[arg(short, long)]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let part = {
        if args.part == 1 {
            days::Part::One
        } else {
            days::Part::Two
        }
    };
    let path = args.input;

    if let Some(solver) = days::get_day(day) {
        match fs::read_to_string(&path) {
            Ok(input) => {
                let start = Instant::now();
                let solution = solver.solve(part, &input);
                let duration = start.elapsed();
                println!("========================================");
                println!("[ Day {day:02} | Part {part:?} ]");
                println!(" Solution: {solution}");
                println!(" Time:     {:.2} ms", duration.as_micros() as f64 / 1000.0);
                println!(" μs:       {} µs", duration.as_micros());
                println!("========================================");
            }
            Err(e) => {
                eprintln!("Error: {e}!")
            }
        }
    } else {
        eprintln!("Error: Day{day:02} not implemented yet!")
    }
}
