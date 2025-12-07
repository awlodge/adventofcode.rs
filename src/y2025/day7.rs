use std::collections::{HashMap, HashSet};

use crate::helpers::{
    grid::{Grid, Point},
    hashmap::CountMap,
};

const INPUT: &str = include_str!("input/day7.txt");

pub fn run() -> (u64, u64) {
    let manifold: Grid<char> = TachyonManifold::parse(INPUT);
    (
        manifold.count_beam_splits() as u64,
        manifold.count_quantum_timelines() as u64,
    )
}

trait TachyonManifold {
    fn parse(input: &str) -> Self;
    fn find_start(&self) -> Point;
    fn count_beam_splits(&self) -> usize;
    fn count_quantum_timelines(&self) -> usize;
}

impl TachyonManifold for Grid<char> {
    fn parse(input: &str) -> Self {
        input.parse().unwrap()
    }

    fn find_start(&self) -> Point {
        self.search('S').unwrap()
    }

    fn count_beam_splits(&self) -> usize {
        let start = self.find_start();
        let mut num_splits = 0;
        let mut beams: HashSet<Point> = HashSet::new();
        beams.insert(start);

        loop {
            let mut new_beams: HashSet<Point> = HashSet::new();
            for beam in beams {
                let next = beam.down();
                let v = self.get(next);
                if v.is_none() {
                    break;
                }
                let v = v.unwrap();
                if v == '^' {
                    num_splits += 1;
                    new_beams.insert(next.left());
                    new_beams.insert(next.right());
                } else {
                    new_beams.insert(next);
                }
            }

            if new_beams.is_empty() {
                break;
            }
            beams = new_beams;
        }

        num_splits
    }

    fn count_quantum_timelines(&self) -> usize {
        let start = self.find_start();
        let mut timelines: HashMap<Point, usize> = HashMap::new();
        timelines.insert(start, 1);
        // let mut dbg_grid = self.clone();

        loop {
            let mut new_timelines: HashMap<Point, usize> = HashMap::new();
            for (beam, count) in &timelines {
                let next = beam.down();
                let v = match self.get(next) {
                    Some(x) => x,
                    None => break,
                };
                if v == '^' {
                    new_timelines.insert_or_increment(next.left(), *count);
                    new_timelines.insert_or_increment(next.right(), *count);
                } else {
                    new_timelines.insert_or_increment(next, *count);
                }
            }

            if new_timelines.is_empty() {
                break;
            }

            timelines = new_timelines;
            // for (p, v) in &timelines {
            //     dbg_grid
            //         .update(
            //             *p,
            //             match *v {
            //                 0 => '0',
            //                 1 => '1',
            //                 2 => '2',
            //                 3 => '3',
            //                 4 => '4',
            //                 5 => '5',
            //                 6 => '6',
            //                 7 => '7',
            //                 8 => '8',
            //                 9 => '9',
            //                 10 => 'a',
            //                 11 => 'b',
            //                 12 => 'c',
            //                 13 => 'd',
            //                 14 => 'e',
            //                 15 => 'f',
            //                 _ => 'X',
            //             },
            //         )
            //         .unwrap();
            // }
        }

        // print!("{dbg_grid}");
        timelines.iter().map(|(_, v)| *v).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::{helpers::grid::Grid, y2025::day7::TachyonManifold};

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_count_beam_splits() {
        let manifold: Grid<char> = TachyonManifold::parse(TEST_INPUT);
        assert_eq!(21, manifold.count_beam_splits());
    }

    #[test]
    fn test_count_quantum_timelines() {
        let manifold: Grid<char> = TachyonManifold::parse(TEST_INPUT);
        assert_eq!(40, manifold.count_quantum_timelines());
    }
}
