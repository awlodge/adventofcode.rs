use crate::helpers::grid::{Direction, Grid, Point};

const INPUT: &str = include_str!("input/day4.txt");
const XMAS: &str = "XMAS";

fn parse(input: &str) -> Grid<char> {
    return Grid::<char>::parse(input);
}

pub fn run() -> (u64, u64) {
    let wordsearch = parse(INPUT);
    (wordsearch.find_words(XMAS) as u64, 0)
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
}
