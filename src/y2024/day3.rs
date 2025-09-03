use regex::Regex;

const INPUT: &str = include_str!("input/day3.txt");

pub fn run() -> (u64, u64) {
    (execute(INPUT), execute_with_conditions(INPUT))
}

fn execute(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    return re
        .captures_iter(input)
        .map(|c| c[1].parse::<u64>().unwrap() * c[2].parse::<u64>().unwrap())
        .sum();
}

fn execute_with_conditions(input: &str) -> u64 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let mut sum: u64 = 0;
    for c in re.captures_iter(input) {
        match &c[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum += c[2].parse::<u64>().expect("Failed to parse {c:#}")
                        * c[3].parse::<u64>().unwrap()
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::y2024::day3::{INPUT, execute, execute_with_conditions};

    #[test]
    fn test_execute() {
        const TEST_INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let output = execute(TEST_INPUT);
        assert_eq!(161, output);
    }

    #[test]
    fn test_solution_part_1() {
        let output = execute(INPUT);
        assert_eq!(178794710, output);
    }

    #[test]
    fn test_execute_with_conditions() {
        const TEST_INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let output = execute_with_conditions(TEST_INPUT);
        assert_eq!(48, output);
    }

    #[test]
    fn test_solution_part_2() {
        let output = execute_with_conditions(INPUT);
        assert_eq!(76729637, output);
    }
}
