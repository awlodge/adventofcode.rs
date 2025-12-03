use std::str::FromStr;

use crate::helpers::parse::parse_split;

const INPUT: &str = include_str!("input/day2.txt");

pub fn run() -> (u64, u64) {
    (sum_invalid_ids(INPUT), 0)
}

fn parse(input: &str) -> impl Iterator<Item = ProductIdRange> {
    parse_split(input.split(','))
}

fn sum_invalid_ids(input: &str) -> u64 {
    parse(input).map(|r| r.list_invalid_ids()).flatten().sum()
}

fn count_digits(inp: u64) -> u32 {
    let mut c: u32 = 1;
    let mut x = inp / 10;
    while x > 0 {
        c += 1;
        x = x / 10;
    }
    c
}

#[derive(Debug)]
struct ProductIdRange {
    min: u64,
    max: u64,
}

#[derive(Debug)]
enum RangeParseError {
    InvalidRange,
    NotInteger,
}

impl FromStr for ProductIdRange {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s.split('-');
        let min: u64 = match ranges.next() {
            Some(r) => match r.parse() {
                Ok(s) => s,
                Err(_) => return Err(RangeParseError::NotInteger),
            },
            None => return Err(RangeParseError::InvalidRange),
        };
        let max: u64 = match ranges.next() {
            Some(r) => match r.parse() {
                Ok(s) => s,
                Err(_) => return Err(RangeParseError::NotInteger),
            },
            None => return Err(RangeParseError::InvalidRange),
        };
        Ok(Self { min, max })
    }
}

impl ProductIdRange {
    const BASE: u64 = 10;

    fn contains(&self, x: u64) -> bool {
        x >= self.min && x <= self.max
    }

    fn nearest_candidate(x: u64) -> (u64, u32) {
        let d = count_digits(x);
        if d % 2 == 0 {
            let exp = d / 2;
            return (x / (Self::BASE.pow(exp)), exp);
        }
        let exp = (d / 2) + 1;
        return (Self::BASE.pow(exp - 1), exp);
    }

    fn min_candidate(&self) -> (u64, u32) {
        ProductIdRange::nearest_candidate(self.min)
    }

    fn max_candidate(&self) -> (u64, u32) {
        ProductIdRange::nearest_candidate(self.max)
    }

    fn list_invalid_ids(&self) -> Vec<u64> {
        let (min_base, min_exp) = self.min_candidate();
        let (max_base, max_exp) = self.max_candidate();
        let mut res: Vec<u64> = Vec::new();
        let mut base = min_base;
        let mut exp = min_exp;
        while exp <= max_exp {
            let y = Self::BASE.pow(exp);
            let mut z = y;
            if exp == max_exp {
                z = max_base + 1;
            }

            while base < z {
                let candidate = (base * y) + base;
                if self.contains(candidate) {
                    res.push(candidate);
                }
                base += 1;
            }
            exp += 1;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::y2025::day2::{ProductIdRange, parse, run, sum_invalid_ids};

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[rstest]
    #[case("11-22", &[11, 22])]
    #[case("95-115", &[99])]
    #[case("998-1012", &[1010])]
    #[case("1188511880-1188511890", &[1188511885])]
    #[case("222220-222224", &[222222])]
    #[case("1698522-1698528", &[])]
    #[case("446443-446449", &[446446])]
    #[case("38593856-38593862", &[38593859])]
    fn test_list_invalid_ids(#[case] range: ProductIdRange, #[case] expected: &[u64]) {
        let expected: Vec<u64> = expected.to_vec();
        assert_eq!(expected, range.list_invalid_ids());
    }

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(1227775554, sum_invalid_ids(TEST_INPUT));
    }

    #[test]
    fn test_solution() {
        let (s1, _) = run();
        assert_eq!(13919717792, s1);
    }
}
