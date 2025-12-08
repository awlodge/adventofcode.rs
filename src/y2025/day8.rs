use std::{cmp::Reverse, collections::HashSet, num::ParseIntError, str::FromStr};

use itertools::Itertools;

use crate::helpers::parse::parse_lines;

const INPUT: &str = include_str!("input/day8.txt");

pub fn run() -> (u64, u64) {
    let circuits = parse(INPUT);
    let circuit_counts = circuits.join_circuits(1000);
    (
        circuit_counts.iter().take(3).map(|c| *c as u64).product(),
        0,
    )
}

fn parse(inp: &str) -> impl Circuits {
    parse_lines(inp).collect::<Vec<JunctionBox>>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> f64 {
        f64::sqrt(
            ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
                as f64,
        )
    }

    fn from_slice(vals: &[i64]) -> Result<Self, JunctionBoxError> {
        if vals.len() != 3 {
            return Err(JunctionBoxError::WrongDimension);
        }

        Ok(JunctionBox {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        })
    }
}

#[derive(Debug)]
enum JunctionBoxError {
    NotAnInteger,
    WrongDimension,
}

impl FromStr for JunctionBox {
    type Err = JunctionBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<Result<i64, ParseIntError>> =
            s.split(',').map(|x| x.parse::<i64>()).collect();
        if vals.iter().any(|x| x.is_err()) {
            return Err(JunctionBoxError::NotAnInteger);
        }
        let vals: Vec<i64> = vals.iter().map(|x| x.clone().unwrap()).collect();
        Self::from_slice(&vals)
    }
}

trait Circuits {
    fn join_circuits(&self, num_circuits: usize) -> Vec<usize>;
}

impl Circuits for Vec<JunctionBox> {
    fn join_circuits(&self, num_circuits: usize) -> Vec<usize> {
        let mut pairs: Vec<Vec<&JunctionBox>> = self.iter().combinations(2).collect();
        pairs.sort_by(|p, q| {
            p[0].distance(&p[1])
                .partial_cmp(&(q[0].distance(&q[1])))
                .unwrap()
        });
        // for p in &pairs {
        //     println!("{p:?} distance {}", p[0].distance(&p[1]));
        // }
        let mut pairs = pairs.iter();
        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let mut num_connections = 0;
        while num_connections < num_circuits {
            // let clen: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
            // println!("{clen:?}");
            let pair = pairs.next();
            if pair.is_none() {
                break;
            }
            let pair = pair.unwrap();
            // println!("Connect pair {pair:?}");
            num_connections += 1;
            let mut existing_circuits = circuits
                .iter_mut()
                .enumerate()
                .filter(|(_, c)| c.contains(pair[0]) || c.contains(pair[1]));

            let c1 = existing_circuits.next();
            if c1.is_none() {
                // println!("  New circuit!");
                circuits.push(HashSet::from([*pair[0], *pair[1]]));
                continue;
            }

            let (_, c1) = c1.unwrap();
            if c1.contains(pair[0]) && c1.contains(pair[1]) {
                // println!("  Already in {c1:?}");
                continue;
            }

            let c2 = existing_circuits.next();
            if c2.is_none() {
                // println!("  Add to {c1:?}");
                c1.insert(*pair[0]);
                c1.insert(*pair[1]);
                continue;
            }

            let (idx2, c2) = c2.unwrap();
            // println!("  Combine {c1:?} with {c2:?}");
            c1.extend(c2.iter());
            circuits.remove(idx2);
        }

        let mut circuit_lens: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
        circuit_lens.sort_by_key(|x| Reverse(*x));
        circuit_lens
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day8::{Circuits, JunctionBox, parse};

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_distance() {
        let p = JunctionBox::from_slice(&[162, 817, 812]).unwrap();
        let q = JunctionBox::from_slice(&[431, 825, 988]).unwrap();
        assert!(p.distance(&q) < 373.41132);
    }

    #[test]
    fn test_join_circuits() {
        let circuits = parse(TEST_INPUT);
        let circuit_counts = circuits.join_circuits(10);
        let prod: usize = circuit_counts.iter().take(3).product();
        assert_eq!(40, prod);
    }
}
