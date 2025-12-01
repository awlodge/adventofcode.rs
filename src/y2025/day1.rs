use std::{path::Iter, str::FromStr};

use crate::helpers::parse::parse_lines;

const INPUT: &str = include_str!("input/day1.txt");

pub fn run() -> (u64, u64) {
    let rotations: Vec<Rotation> = parse_lines(INPUT).collect();
    (
        calculate_password(rotations.iter()),
        calculate_password_secure(rotations.iter()),
    )
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: u32,
    full_spins: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseRotationError {
    InvalidDirection,
    UnparseableDistance,
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match &s[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseRotationError::InvalidDirection),
        };
        let full_distance: u32 = match (&s[1..]).parse::<u32>() {
            Ok(r) => r,
            Err(_) => return Err(ParseRotationError::UnparseableDistance),
        };
        let full_spins = full_distance / 100;
        let distance = full_distance % 100;

        Ok(Rotation {
            direction,
            distance,
            full_spins,
        })
    }
}

impl Rotation {
    fn rotate(&self, start: u32) -> u32 {
        match &self.direction {
            Direction::Left => (start + (100 - self.distance)) % 100,
            Direction::Right => (start + self.distance) % 100,
        }
    }

    fn rotate_count_zeroes(&self, start: u32) -> (u32, u32) {
        if start == 0 {
            return (self.rotate(start), self.full_spins);
        }

        let mut pos: i32 = start as i32;
        let mut count: u32 = self.full_spins;
        pos = match &self.direction {
            Direction::Left => pos - (self.distance as i32),
            Direction::Right => pos + (self.distance as i32),
        };
        if pos >= 100 || pos <= 0 {
            count += 1;
        }
        pos = pos % 100;
        if pos < 0 {
            pos = pos + 100;
        }
        (pos.try_into().unwrap(), count)
    }
}

fn calculate_password<'a>(rotations: impl Iterator<Item = &'a Rotation>) -> u64 {
    let mut pos: u32 = 50;
    let mut password: u64 = 0;
    for r in rotations {
        pos = r.rotate(pos);
        if pos == 0 {
            password += 1;
        }
    }
    password
}

fn calculate_password_secure<'a>(rotations: impl Iterator<Item = &'a Rotation>) -> u64 {
    let mut password: u64 = 0;
    let mut pos: u32 = 50;
    for r in rotations {
        let (p, c) = r.rotate_count_zeroes(pos);
        pos = p;
        password += c as u64;
    }
    password
}

#[cfg(test)]
mod test {
    use crate::y2025::day1::{
        Rotation, calculate_password, calculate_password_secure, parse_lines, run,
    };

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_calculate_password() {
        let rotations: Vec<Rotation> = parse_lines(TEST_INPUT).collect();
        assert_eq!(3, calculate_password(rotations.iter()));
    }

    #[test]
    fn test_calculate_password_secure() {
        let rotations: Vec<Rotation> = parse_lines(TEST_INPUT).collect();
        assert_eq!(6, calculate_password_secure(rotations.iter()));
    }

    #[test]
    fn test_solution() {
        let (p1, p2) = run();
        assert_eq!(1118, p1);
        assert_eq!(6289, p2);
    }
}
