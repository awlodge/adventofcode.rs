use crate::helpers::grid::{Direction, Grid, Point};

const INPUT: &str = include_str!("input/day4.txt");

pub fn run() -> (u64, u64) {
    let mut paper_trail = parse(INPUT);
    (
        paper_trail.count_accessible_rolls() as u64,
        paper_trail.remove_accessible_rolls() as u64,
    )
}

fn parse(input: &str) -> impl PaperTrail {
    input.parse::<Grid<char>>().unwrap()
}

trait PaperTrail {
    const ROLL: char = '@';
    fn iter_accessible_rolls(&self) -> impl Iterator<Item = Point>;
    fn count_accessible_rolls(&self) -> usize;
    fn is_accessible(&self, p: Point) -> bool;
    fn is_roll(&self, p: Point) -> bool;
    fn remove_roll(&mut self, p: Point);
    fn remove_accessible_rolls(&mut self) -> usize;
}

impl PaperTrail for Grid<char> {
    fn iter_accessible_rolls(&self) -> impl Iterator<Item = Point> {
        self.walk()
            .filter(|(p, _)| self.is_roll(*p) && self.is_accessible(*p))
            .map(|(p, _)| p)
    }

    fn count_accessible_rolls(&self) -> usize {
        self.iter_accessible_rolls().count()
    }

    fn is_accessible(&self, p: Point) -> bool {
        Direction::iter().filter(|d| self.is_roll(p + *d)).count() < 4
    }

    fn is_roll(&self, p: Point) -> bool {
        self.get(p).is_some_and(|x| x == Self::ROLL)
    }

    fn remove_roll(&mut self, p: Point) {
        self.update(p, '.').unwrap()
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut count = 0;
        loop {
            let rolls: Vec<Point> = self.iter_accessible_rolls().collect();
            let roll_count = rolls.len();
            if roll_count == 0 {
                break;
            }
            for roll in rolls {
                self.remove_roll(roll);
            }
            count += roll_count;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day4::{PaperTrail, parse, run};

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_count_accessible_rolls() {
        assert_eq!(13, parse(TEST_INPUT).count_accessible_rolls())
    }

    #[test]
    fn test_remove_accessible_rolls() {
        let mut paper_trail = parse(TEST_INPUT);
        assert_eq!(43, paper_trail.remove_accessible_rolls());
    }

    #[test]
    fn test_solution() {
        let (s1, s2) = run();
        assert_eq!(1578, s1);
        assert_eq!(10132, s2);
    }
}
