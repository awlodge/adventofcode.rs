pub mod y2024;
use crate::y2024::day1::run;

fn main() {
    println!("2024 Day 1:");
    let (x, y) = run();
    println!("  Part 1: {x}");
    println!("  Part 2: {y}");
}
