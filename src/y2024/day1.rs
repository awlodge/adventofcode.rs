use std::collections::HashMap;

const INPUT: &str = include_str!("input/day1.txt");

fn distance(a: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    a.iter()
        .enumerate()
        .map(|(idx, x)| (x - b[idx]).abs())
        .sum()
}

fn similarity(a: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    a.iter()
        .map(|x| {
            *x * (*counts
                .entry(*x)
                .or_insert(b.iter().filter(|y| **y == *x).count()) as i32)
        })
        .sum()
}

fn parse(input: &str, col1: &mut Vec<i32>, col2: &mut Vec<i32>) {
    for l in input.split('\n') {
        for (idx, x) in l.split_ascii_whitespace().enumerate() {
            match idx {
                0 => col1.push(x.parse().expect("Could not parse {x}")),
                1 => col2.push(x.parse().expect("Could not parse {x}")),
                _ => panic!("Unexpected input!"),
            }
        }
    }
}

pub fn run() -> (i32, i32) {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    parse(INPUT, &mut a, &mut b);
    let x = distance(&mut a, &mut b);

    let y = similarity(&mut a, &mut b);
    (x, y)
}

#[cfg(test)]
mod tests {
    use crate::y2024::day1::{INPUT, distance, parse, similarity};

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_distance() {
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        parse(TEST_INPUT, &mut a, &mut b);
        assert_eq!(11, distance(&mut a, &mut b))
    }

    #[test]
    fn test_similarity() {
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        parse(TEST_INPUT, &mut a, &mut b);
        assert_eq!(31, similarity(&mut a, &mut b))
    }

    #[test]
    fn test_solution_part_1() {
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        parse(INPUT, &mut a, &mut b);
        assert_eq!(1506483, distance(&mut a, &mut b));
    }

    #[test]
    fn test_solution_part_2() {
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        parse(INPUT, &mut a, &mut b);
        assert_eq!(23126924, similarity(&mut a, &mut b));
    }
}
