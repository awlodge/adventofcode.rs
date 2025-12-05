use std::str::FromStr;

const INPUT: &str = include_str!("input/day5.txt");

pub fn run() -> (u64, u64) {
    let (ranges, ingredients) = parse(INPUT);
    (
        ranges.count_fresh(&ingredients) as u64,
        ranges.total_fresh(),
    )
}

fn parse(inp: &str) -> (impl FreshRanges, Vec<u64>) {
    let mut lines = inp.split('\n');
    let ranges: Vec<Range> = FreshRanges::parse(
        lines
            .by_ref()
            .take_while(|x| !x.trim().is_empty())
            .map(|line| line.trim()),
    );
    let ingredients: Vec<u64> = lines.map(|line| line.trim().parse().unwrap()).collect();
    (ranges, ingredients)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

#[derive(Debug)]
enum RangeParseError {
    InvalidRange,
    NotInteger,
}

impl FromStr for Range {
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

impl Range {
    fn contains(&self, x: u64) -> bool {
        self.min <= x && self.max >= x
    }

    fn count(&self) -> u64 {
        1 + self.max - self.min
    }
}

trait FreshRanges {
    fn parse<'a>(inp: impl Iterator<Item = &'a str>) -> Self;
    fn collapse(&mut self) -> Self;
    fn is_fresh(&self, x: u64) -> bool;
    fn count_fresh(&self, ingredients: &[u64]) -> usize;
    fn total_fresh(&self) -> u64;
}

impl FreshRanges for Vec<Range> {
    fn parse<'a>(inp: impl Iterator<Item = &'a str>) -> Self {
        let mut res: Vec<Range> = inp.map(|x| x.parse().unwrap()).collect();
        res.collapse()
    }

    fn collapse(&mut self) -> Self {
        let mut res: Vec<Range> = Vec::new();
        self.sort();
        for range in self {
            let prev = res.last();
            if prev.is_some_and(|x| range.min <= x.max) {
                let prev = prev.unwrap();
                let new_min = prev.min;
                let new_max = prev.max.max(range.max);
                res.remove(res.len() - 1);
                res.push(Range {
                    min: new_min,
                    max: new_max,
                });
            } else {
                res.push(range.clone());
            }
        }

        res
    }

    fn is_fresh(&self, x: u64) -> bool {
        // println!("Ingredient: {x}");
        let mut start = 0;
        let mut end = self.len();
        let mut mid = (start + end) / 2;
        while start < end {
            let candidate = self[mid];
            // println!("  Candidate: {candidate:?}");
            if candidate.contains(x) {
                // println!("  Fresh!");
                return true;
            }
            if x < candidate.min {
                end = mid;
            } else {
                start = mid + 1;
            }
            mid = (start + end) / 2;
        }

        // println!("  Not fresh");
        false
    }

    fn count_fresh(&self, ingredients: &[u64]) -> usize {
        ingredients.iter().filter(|x| self.is_fresh(**x)).count()
    }

    fn total_fresh(&self) -> u64 {
        self.iter().map(|range| range.count()).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day5::{FreshRanges, parse};

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_count_fresh() {
        let (ranges, ingredients) = parse(TEST_INPUT);
        assert_eq!(3, ranges.count_fresh(&ingredients));
    }

    #[test]
    fn test_total_fresh() {
        let (ranges, _) = parse(TEST_INPUT);
        assert_eq!(14, ranges.total_fresh());
    }
}
