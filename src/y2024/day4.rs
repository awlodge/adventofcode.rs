use crate::helpers::grid::{Direction, Grid, Point};

const INPUT: &str = include_str!("input/day4.txt");
const XMAS: &str = "XMAS";

fn parse(input: &str) -> Grid<char> {
    input.parse().unwrap()
}

pub fn run() -> (u64, u64) {
    let wordsearch = parse(INPUT);
    (
        wordsearch.find_words(XMAS) as u64,
        wordsearch.find_x_mas() as u64,
    )
}

impl Grid<char> {
    fn find_words(&self, word: &str) -> u32 {
        let start = word.chars().next().expect("Empty word");
        return self
            .walk()
            .map(|(p, v)| {
                if v == start {
                    self.find_words_at_point(&p, &word[1..])
                } else {
                    0
                }
            })
            .sum();
    }

    fn find_words_at_point(&self, p: &Point, word: &str) -> u32 {
        return Direction::iter()
            .filter(|d| self.direction_is_word(p, &d, word))
            .count() as u32;
    }

    fn direction_is_word(&self, p: &Point, d: &Point, word: &str) -> bool {
        let start = match word.chars().next() {
            Some(c) => c,
            None => return true,
        };

        let mut place = *p + *d;
        if !self.get(place).is_some_and(|x| x == start) {
            return false;
        }

        for c in word[1..].chars() {
            place += *d;
            if !self.get(place).is_some_and(|x| x == c) {
                return false;
            }
        }

        true
    }

    fn find_x_mas(&self) -> u32 {
        return self.walk().filter(|(p, _)| self.is_x_mas(p)).count() as u32;
    }

    fn is_x_mas(&self, p: &Point) -> bool {
        if self.get(*p).is_none_or(|v| v != 'A') {
            return false;
        }

        let corners = [
            self.get(p + Direction::NorthWest.point()),
            self.get(p + Direction::NorthEast.point()),
            self.get(p + Direction::SouthEast.point()),
            self.get(p + Direction::SouthWest.point()),
        ];

        if corners.iter().any(|v| v.is_none()) {
            return false;
        }

        let corners = [
            corners[0].unwrap(),
            corners[1].unwrap(),
            corners[2].unwrap(),
            corners[3].unwrap(),
        ];

        return ((corners[0] == 'M' && corners[2] == 'S')
            || (corners[0] == 'S' && corners[2] == 'M'))
            && ((corners[1] == 'M' && corners[3] == 'S')
                || (corners[1] == 'S' && corners[3] == 'M'));
    }
}

#[cfg(test)]
mod test {
    use crate::y2024::day4::{INPUT, XMAS, parse};

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_find_xmas_words() {
        let wordsearch = parse(TEST_INPUT);
        assert_eq!(18, wordsearch.find_words(XMAS));
    }

    #[test]
    fn test_solution_part_1() {
        let wordsearch = parse(INPUT);
        assert_eq!(2496, wordsearch.find_words(XMAS));
    }

    #[test]
    fn test_find_x_mas() {
        let wordsearch = parse(TEST_INPUT);
        assert_eq!(9, wordsearch.find_x_mas());
    }

    #[test]
    fn test_solution_part_2() {
        let wordsearch = parse(INPUT);
        assert_eq!(1967, wordsearch.find_x_mas());
    }
}
