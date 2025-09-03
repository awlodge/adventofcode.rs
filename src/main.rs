pub mod y2024;

fn main() {
    println!("2024 Day 1:");
    let (x, y) = y2024::day1::run();
    println!("  Part 1: {x}");
    println!("  Part 2: {y}");

    println!("2024 Day 2:");
    let (x, y) = y2024::day2::run();
    println!("  Part 1: {x}");
    println!("  Part 2: {y}");
}
