const INPUT: &str = include_str!("input/day1.txt");

pub fn run() -> (u64, u64) {
    let rotations = parse(INPUT);
    (
        calculate_password(&rotations),
        calculate_password_secure(&rotations),
    )
}

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: u32,
    full_spins: u32,
}

impl Rotation {
    fn from(input: &str) -> Self {
        let direction = match &input[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unexpected direction"),
        };
        let full_distance: u32 = (&input[1..]).parse::<u32>().unwrap();
        let full_spins = full_distance / 100;
        let distance = full_distance % 100;
        Rotation {
            direction,
            distance,
            full_spins,
        }
    }

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

fn parse(input: &str) -> Vec<Rotation> {
    let mut rotations: Vec<Rotation> = Vec::new();
    rotations.extend(input.split('\n').map(|x| Rotation::from(x.trim())));
    rotations
}

fn calculate_password(rotations: &Vec<Rotation>) -> u64 {
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

fn calculate_password_secure(rotations: &Vec<Rotation>) -> u64 {
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
    use crate::y2025::day1::{calculate_password, calculate_password_secure, parse, run};

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
        let rotations = parse(TEST_INPUT);
        assert_eq!(3, calculate_password(&rotations));
    }

    #[test]
    fn test_calculate_password_secure() {
        let rotations = parse(TEST_INPUT);
        assert_eq!(6, calculate_password_secure(&rotations));
    }

    #[test]
    fn test_solution() {
        let (p1, p2) = run();
        assert_eq!(1118, p1);
        assert_eq!(6289, p2);
    }
}
