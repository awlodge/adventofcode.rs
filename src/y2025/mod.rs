mod day1;

pub fn run(day: u32) -> Result<(u64, u64), u32> {
    match day {
        1 => Ok(day1::run()),
        _ => Err(day),
    }
}
