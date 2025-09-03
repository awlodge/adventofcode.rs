pub mod y2024;

fn main() {
    for day in 1..26 {
        match y2024::run(day) {
            Ok((x, y)) => {
                println!("2024 Day {day}");
                println!("  Part 1: {x}");
                println!("  Part 2: {y}");
            }
            Err(_) => {
                println!("No implementation found for 2024 Day {day}. Stopping");
                break;
            }
        }
    }
}
