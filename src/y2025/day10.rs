use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::helpers::parse::parse_lines;

const INPUT: &str = include_str!("input/day10.txt");

pub fn run() -> (u64, u64) {
    (sum_min_presses(INPUT), 0)
}

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    parse_lines(input)
}

fn sum_min_presses(input: &str) -> u64 {
    let mut machines: Vec<Machine> = parse(input).collect();
    let mut s: u64 = 0;
    for machine in machines.iter_mut() {
        s += machine.find_min_presses().unwrap() as u64;
    }
    s
}

#[derive(Debug)]
struct Machine {
    lights: u16,
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u32>,
}

impl Machine {
    fn push_button(&mut self, idx: usize) {
        self.lights = self.lights ^ self.buttons[idx];
    }

    fn reset(&mut self) {
        self.lights = 0;
    }

    fn is_at_target(&self) -> bool {
        self.lights == self.target
    }

    fn find_min_presses(&mut self) -> Option<usize> {
        for idx in 1..self.buttons.len() {
            for presses in (0..self.buttons.len()).combinations(idx) {
                for b in presses {
                    self.push_button(b);
                }
                if self.is_at_target() {
                    return Some(idx);
                }
                self.reset();
            }
        }
        None
    }
}

#[derive(Debug)]
enum MachineParseErr {
    InvalidMachine,
}

impl FromStr for Machine {
    type Err = MachineParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^\[(?<lights>[\.#]+)\] (?<buttons>[\(\)\d, ]+\) )+\{(?<joltages>[\d,]+)\}$",
        )
        .unwrap();
        let captures = re.captures(s);
        if captures.is_none() {
            return Err(MachineParseErr::InvalidMachine);
        }
        let captures = captures.unwrap();
        let target_lights = captures.name("lights").unwrap().as_str().chars();
        let mut target: u16 = 0;
        for (idx, c) in target_lights.enumerate() {
            if c == '#' {
                target = target | (1 << idx);
            }
        }

        let buttons_inp = captures.name("buttons").unwrap().as_str();
        let buttons_re = Regex::new(r"\((?<button>[\d,]+)\)").unwrap();
        let mut buttons: Vec<u16> = Vec::new();
        for cap in buttons_re.captures_iter(buttons_inp) {
            let mut b: u16 = 0;
            for c in cap
                .name("button")
                .unwrap()
                .as_str()
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
            {
                b = b | (1 << c);
            }
            buttons.push(b);
        }

        let joltages: Vec<u32> = captures
            .name("joltages")
            .unwrap()
            .as_str()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Ok(Machine {
            lights: 0,
            target,
            buttons,
            joltages,
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::y2025::day10::{Machine, sum_min_presses};

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_parse() {
        let inp = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine: Machine = inp.parse().unwrap();
        assert_eq!(0, machine.lights);
        assert_eq!(0b0110, machine.target);
        assert_eq!(
            Vec::from(&[0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]),
            machine.buttons
        );
        assert_eq!(Vec::from(&[3, 5, 4, 7]), machine.joltages);
    }

    #[test]
    fn test_push_button() {
        let inp = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let mut machine: Machine = inp.parse().unwrap();
        machine.push_button(0);
        assert_eq!(0b1000, machine.lights);
        machine.push_button(1);
        assert_eq!(0b0010, machine.lights);
        machine.push_button(2);
        assert_eq!(0b0110, machine.lights);
        machine.push_button(3);
        assert_eq!(0b1010, machine.lights);
        machine.push_button(4);
        assert_eq!(0b1111, machine.lights);
        machine.push_button(5);
        assert_eq!(0b1100, machine.lights);
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 2)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 3)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 2)]
    fn test_find_min_presses(#[case] input: &str, #[case] min_presses: usize) {
        let mut machine: Machine = input.parse().unwrap();
        assert_eq!(min_presses, machine.find_min_presses().unwrap());
    }

    #[test]
    fn test_sum_min_presses() {
        assert_eq!(7, sum_min_presses(TEST_INPUT))
    }
}
