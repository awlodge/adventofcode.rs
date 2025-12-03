const INPUT: &str = include_str!("input/day3.txt");
const SAFE_NUM_BATTERIES: usize = 2;
const UNSAFE_NUM_BATTERIES: usize = 12;

pub fn run() -> (u64, u64) {
    (sum_joltage_safe(INPUT), sum_joltage_unsafe(INPUT))
}

fn sum_joltage_safe(input: &str) -> u64 {
    sum_joltage(input, SAFE_NUM_BATTERIES)
}

fn sum_joltage_unsafe(input: &str) -> u64 {
    sum_joltage(input, UNSAFE_NUM_BATTERIES)
}

fn sum_joltage(input: &str, num_batteries: usize) -> u64 {
    input
        .split('\n')
        .map(|l| parse_bank(l.trim()))
        .map(|r| {
            let bank: Vec<u32> = r.collect();
            calculate_joltage(&bank, num_batteries)
        })
        .sum()
}

fn calculate_joltage(bank: &[u32], num_batteries: usize) -> u64 {
    let mut digits: Vec<u32> = Vec::new();
    let bank_len = bank.len();

    for (idx, d) in bank.iter().enumerate() {
        let distance = bank_len - idx;
        let mut place = 0;
        if distance < num_batteries {
            place = num_batteries - distance;
        }

        let mut placed = false;
        while place < digits.len() {
            if *d > digits[place] {
                digits[place] = *d;
                digits.truncate(place + 1);
                placed = true;
            }
            place += 1
        }
        if !placed && digits.len() < num_batteries {
            digits.push(*d);
        }
    }

    concat_digits(&mut digits)
}

fn concat_digits(digits: &mut Vec<u32>) -> u64 {
    let mut base: u64 = 1;
    let mut res: u64 = 0;
    while digits.len() > 0 {
        res += (digits.pop().unwrap() as u64) * base;
        base *= 10;
    }

    res
}

fn parse_bank(input: &str) -> impl Iterator<Item = u32> {
    input.chars().map(|c| c.to_digit(10).unwrap())
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::y2025::day3::{
        SAFE_NUM_BATTERIES, UNSAFE_NUM_BATTERIES, calculate_joltage, parse_bank, run,
        sum_joltage_safe, sum_joltage_unsafe,
    };

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_calculate_joltage_safe(#[case] inp: &str, #[case] expected: u64) {
        let bank: Vec<u32> = parse_bank(inp).collect();
        assert_eq!(expected, calculate_joltage(&bank, SAFE_NUM_BATTERIES))
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_calculate_joltage_unsafe(#[case] inp: &str, #[case] expected: u64) {
        let bank: Vec<u32> = parse_bank(inp).collect();
        assert_eq!(expected, calculate_joltage(&bank, UNSAFE_NUM_BATTERIES))
    }

    #[test]
    fn test_sum_joltage_safe() {
        assert_eq!(357, sum_joltage_safe(TEST_INPUT))
    }

    #[test]
    fn test_sum_joltage_unsafe() {
        assert_eq!(3121910778619, sum_joltage_unsafe(TEST_INPUT))
    }

    #[test]
    fn test_solution() {
        let (s1, s2) = run();
        assert_eq!(17074, s1);
        assert_eq!(169512729575727, s2);
    }
}
