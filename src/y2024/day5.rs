use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input/day5.txt");

pub fn run() -> (u64, u64) {
    let (rules, mut updates) = parse(INPUT);
    return (
        sum_middle_valid_updates(&rules, &updates) as u64,
        sum_middle_fixed_updates(&rules, &mut updates) as u64,
    );
}

fn parse(input: &str) -> (Rules, Vec<Vec<u32>>) {
    let mut lines = input.split('\n');
    let mut line = lines.next();
    let mut rules = Rules::new();
    while line.is_some_and(|l| !l.trim().is_empty()) {
        rules.parse_and_add_rule(line.unwrap().trim());
        line = lines.next();
    }

    line = lines.next();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    while line.is_some() {
        let mut update: Vec<u32> = Vec::new();
        update.extend(
            line.unwrap()
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().expect("Failed to read page")),
        );
        updates.push(update);
        line = lines.next()
    }

    return (rules, updates);
}

fn find_middle(update: &Vec<u32>) -> u32 {
    update[update.len() / 2]
}

fn sum_middle_valid_updates(rules: &Rules, updates: &Vec<Vec<u32>>) -> u32 {
    return updates
        .iter()
        .map(|update| {
            if rules.validate(update) {
                find_middle(update)
            } else {
                0
            }
        })
        .sum();
}

fn sum_middle_fixed_updates(rules: &Rules, updates: &mut Vec<Vec<u32>>) -> u32 {
    return updates
        .iter_mut()
        .map(|update| {
            if rules.validate(update) {
                0
            } else {
                find_middle(&rules.fix(update))
            }
        })
        .sum();
}

struct Rules {
    rules_before: HashMap<u32, HashSet<u32>>,
    rules_after: HashMap<u32, HashSet<u32>>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            rules_before: HashMap::new(),
            rules_after: HashMap::new(),
        }
    }

    fn add_rule(&mut self, before: u32, after: u32) {
        self.rules_before
            .entry(before)
            .or_insert(HashSet::<u32>::new())
            .insert(after);
        self.rules_after
            .entry(after)
            .or_insert(HashSet::<u32>::new())
            .insert(before);
    }

    pub fn parse_and_add_rule(&mut self, input: &str) {
        let mut parts = input.split('|');
        let before: u32 = parts
            .next()
            .expect("Failed to parse line")
            .parse()
            .expect("Failed to parse line");
        let after: u32 = parts
            .next()
            .expect("Failed to parse line")
            .parse()
            .expect("Failed to parse line");
        self.add_rule(before, after);
    }

    pub fn validate(&self, input: &Vec<u32>) -> bool {
        let mut visited: HashSet<u32> = HashSet::new();
        for x in input.iter() {
            if self
                .rules_before
                .get(x)
                .is_some_and(|rule| !visited.is_disjoint(rule))
            {
                return false;
            }
            visited.insert(*x);
        }

        true
    }

    pub fn fix(&self, input: &mut Vec<u32>) -> Vec<u32> {
        let mut stack: Vec<u32> = Vec::new();
        let mut sorted: Vec<u32> = Vec::new();
        while input.len() > 0 {
            let current = match stack.pop() {
                Some(x) => x,
                None => input.pop().expect("Input is empty!"),
            };
            if self.rules_after.contains_key(&current) {
                let mut remainder: HashSet<u32> = HashSet::new();
                remainder.extend(input.iter());
                let rule = self.rules_after.get(&current).unwrap();
                remainder.retain(|x| rule.contains(x));
                if remainder.len() > 0 {
                    for x in remainder.iter() {
                        stack.push(*x);
                        input.pop_if(|y| *y == *x);
                    }
                    stack.push(current);
                } else {
                    sorted.push(current);
                }
            } else {
                sorted.push(current);
            }
        }

        sorted
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::y2024::day5::{INPUT, parse, sum_middle_valid_updates};

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[rstest]
    #[case(0, true)]
    #[case(1, true)]
    #[case(2, true)]
    #[case(3, false)]
    #[case(4, false)]
    #[case(5, false)]
    fn test_validate(#[case] index: usize, #[case] expected: bool) {
        let (rules, updates) = parse(TEST_INPUT);
        assert_eq!(expected, rules.validate(&updates[index]));
    }

    #[test]
    fn test_sum_middle_valid_updates() {
        let (rules, updates) = parse(TEST_INPUT);
        assert_eq!(143, sum_middle_valid_updates(&rules, &updates))
    }

    #[test]
    fn test_solution_part_1() {
        let (rules, updates) = parse(INPUT);
        assert_eq!(4766, sum_middle_valid_updates(&rules, &updates))
    }

    #[rstest]
    #[case(3, &[97, 75, 47, 61, 53])]
    #[case(4, &[61, 29, 13])]
    #[case(5, &[97, 75, 47, 29, 13])]
    fn test_fix(#[case] index: usize, #[case] expected: &[u32]) {
        let (rules, mut updates) = parse(TEST_INPUT);
        let expected = Vec::from(expected);
        assert_eq!(expected, rules.fix(&mut updates[index]));
    }
}
