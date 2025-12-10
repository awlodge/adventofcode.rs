mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(day: u32) -> Result<(u64, u64), u32> {
    match day {
        1 => Ok(day1::run()),
        2 => Ok(day2::run()),
        3 => Ok(day3::run()),
        4 => Ok(day4::run()),
        5 => Ok(day5::run()),
        6 => Ok(day6::run()),
        7 => Ok(day7::run()),
        8 => Ok(day8::run()),
        9 => Ok(day9::run()),
        10 => Ok(day10::run()),
        _ => Err(day),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use rstest::{fixture, rstest};

    use crate::y2025::run;

    #[fixture]
    fn solutions() -> HashMap<u32, (u64, u64)> {
        const SOLUTIONS: &str = include_str!("solutions.txt");
        let mut s = HashMap::new();
        for line in SOLUTIONS.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let cols: Vec<u64> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
            let day: u32 = cols[0].try_into().unwrap();
            s.insert(day, (cols[1], cols[2]));
        }

        s
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    #[case(5)]
    #[case(6)]
    #[case(7)]
    #[case(8)]
    #[case(9)]
    #[case(10)]
    fn test_solution(solutions: HashMap<u32, (u64, u64)>, #[case] day: u32) {
        let (exp_s1, exp_s2) = solutions.get(&day).unwrap();
        let (s1, s2) = run(day).unwrap();
        assert_eq!(*exp_s1, s1);
        assert_eq!(*exp_s2, s2);
    }
}
