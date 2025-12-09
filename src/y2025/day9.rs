use itertools::Itertools;

use crate::helpers::{grid::Point, parse::parse_lines};

const INPUT: &str = include_str!("input/day9.txt");

pub fn run() -> (u64, u64) {
    let floor = parse(INPUT);
    (floor.max_rectangle(), 0)
}

fn parse(input: &str) -> impl TheaterFloor {
    let x: Vec<Point> = parse_lines(input).collect();
    x
}

trait Rectangle {
    fn area(&self, other: &Self) -> u64;
}

impl Rectangle for Point {
    fn area(&self, other: &Self) -> u64 {
        let a = ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1);
        a.try_into().unwrap()
    }
}

trait TheaterFloor {
    fn max_rectangle(&self) -> u64;
}

impl TheaterFloor for Vec<Point> {
    fn max_rectangle(&self) -> u64 {
        self.iter()
            .combinations(2)
            .map(|pair| pair[0].area(pair[1]))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day9::{TheaterFloor, parse};

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_max_rectangle() {
        let t = parse(TEST_INPUT);
        assert_eq!(50, t.max_rectangle());
    }
}
