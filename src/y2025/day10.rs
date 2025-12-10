use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::helpers::parse::parse_lines;

const INPUT: &str = include_str!("input/day10.txt");

pub fn run() -> (u64, u64) {
    let mut machines: Vec<Machine> = Machines::parse(INPUT);
    (
        machines.sum_min_presses(),
        0, // machines.sum_min_presses_joltage(),
    )
}

trait Machines {
    fn parse(input: &str) -> Self;
    fn sum_min_presses(&mut self) -> u64;
    fn sum_min_presses_joltage(&mut self) -> u64;
}

impl Machines for Vec<Machine> {
    fn parse(input: &str) -> Vec<Machine> {
        parse_lines(input).collect()
    }

    fn sum_min_presses(&mut self) -> u64 {
        self.iter_mut()
            .map(|m| m.find_min_presses().unwrap() as u64)
            .sum()
    }

    fn sum_min_presses_joltage(&mut self) -> u64 {
        self.iter_mut()
            .map(|m| m.find_min_presses_joltage().unwrap() as u64)
            .sum()
    }
}

#[derive(Debug)]
struct Machine {
    lights: u16,
    target_lights: u16,
    buttons: Vec<u16>,
    joltages: Vec<u32>,
    target_joltages: Vec<u32>,
}

impl Machine {
    fn push_button(&mut self, idx: usize) {
        self.lights = self.lights ^ self.buttons[idx];
    }

    fn push_button_joltage(&mut self, idx: usize) {
        let mut button = self.buttons[idx];
        for i in 0..self.joltages.len() {
            if button & 1 == 1 {
                self.joltages[i] += 1
            }
            button = button >> 1;
        }
    }

    fn reset(&mut self) {
        self.lights = 0;
        let mut new_joltages: Vec<u32> = Vec::with_capacity(self.joltages.len());
        new_joltages.resize(self.joltages.len(), 0);
        self.joltages = new_joltages;
    }

    fn is_at_target(&self) -> bool {
        self.lights == self.target_lights
    }

    fn is_at_target_joltage(&self) -> bool {
        self.joltages == self.target_joltages
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

    fn find_min_presses_joltage(&mut self) -> Option<usize> {
        let mut idx: usize = 0;
        loop {
            for presses in (0..self.buttons.len()).combinations_with_replacement(idx) {
                for b in presses {
                    self.push_button_joltage(b);
                }
                if self.is_at_target_joltage() {
                    return Some(idx);
                }
                self.reset();
            }
            idx += 1;
        }
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

        let target_joltages: Vec<u32> = captures
            .name("joltages")
            .unwrap()
            .as_str()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut joltages: Vec<u32> = Vec::with_capacity(target_joltages.len());
        joltages.resize(target_joltages.len(), 0);

        Ok(Machine {
            lights: 0,
            target_lights: target,
            buttons,
            joltages,
            target_joltages,
        })
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::y2025::day10::{Machine, Machines};

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_parse() {
        let inp = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine: Machine = inp.parse().unwrap();
        assert_eq!(0, machine.lights);
        assert_eq!(0b0110, machine.target_lights);
        assert_eq!(
            Vec::from(&[0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011]),
            machine.buttons
        );
        assert_eq!(Vec::from(&[3, 5, 4, 7]), machine.target_joltages);
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
        let mut machines: Vec<Machine> = Machines::parse(TEST_INPUT);
        assert_eq!(7, machines.sum_min_presses());
    }

    #[test]
    fn test_push_button_joltage() {
        let inp = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let mut machine: Machine = inp.parse().unwrap();
        for b in &[0, 1, 1, 1, 3, 3, 3, 4, 5, 5] {
            machine.push_button_joltage(*b);
        }
        assert_eq!(machine.target_joltages, machine.joltages);
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 12)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 11)]
    fn test_find_min_presses_joltage(#[case] input: &str, #[case] min_presses: usize) {
        let mut machine: Machine = input.parse().unwrap();
        assert_eq!(min_presses, machine.find_min_presses_joltage().unwrap());
    }

    #[test]
    fn test_sum_min_presses_joltage() {
        let mut machines: Vec<Machine> = Machines::parse(TEST_INPUT);
        assert_eq!(33, machines.sum_min_presses_joltage());
    }
}
