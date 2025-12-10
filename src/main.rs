use std::{fs::File, io::Write};

use clap::{Parser, command};

pub mod helpers;
pub mod y2024;
pub mod y2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2024..2026), default_value_t = 2025)]
    year: u16,

    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..26))]
    day: Option<u16>,

    #[arg(short, long)]
    build_solution_file: bool,
}

fn main() {
    let args = Args::parse();
    let days = match args.day {
        Some(d) => d..(d + 1),
        None => 1..26,
    };

    let run = match args.year {
        2024 => y2024::run,
        2025 => y2025::run,
        _ => panic!("Year {} not found", args.year),
    };

    let mut solutions: Vec<(u16, u64, u64)> = Vec::new();
    for day in days {
        match run(day as u32) {
            Ok((x, y)) => {
                solutions.push((day, x, y));
                println!("{} Day {day}", args.year);
                println!("  Part 1: {x}");
                println!("  Part 2: {y}");
            }
            Err(_) => {
                println!(
                    "No implementation found for {} Day {day}. Stopping",
                    args.year
                );
                break;
            }
        }
    }

    if args.build_solution_file {
        let mut buffer = File::create(format!("src/y{}/solutions.txt", args.year)).unwrap();
        for (d, s1, s2) in solutions {
            writeln!(buffer, "{d},{s1},{s2}").unwrap();
        }
    }
}
