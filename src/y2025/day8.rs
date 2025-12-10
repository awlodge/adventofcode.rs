use std::{cmp::Reverse, collections::BinaryHeap, num::ParseIntError, str::FromStr};

use itertools::Itertools;

use crate::helpers::{disjointset::DisjointSet, parse::parse_lines};

const INPUT: &str = include_str!("input/day8.txt");

pub fn run() -> (u64, u64) {
    let circuits = parse(INPUT);
    let mut connections = circuits.all_pairs();
    let last_join = connections.join_all_circuits(&circuits).unwrap();
    let mut connections = circuits.all_pairs();

    (
        connections.join_circuits(1000).product_top_circuits(3) as u64,
        last_join.0.wall_distance(&last_join.1),
    )
}

fn parse(inp: &str) -> impl JunctionBoxes {
    parse_lines(inp).collect::<Vec<JunctionBox>>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
            .try_into()
            .unwrap()
    }

    fn wall_distance(&self, other: &JunctionBox) -> u64 {
        (self.x * other.x).try_into().unwrap()
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionBoxConnection {
    distance: u64,
    c1: JunctionBox,
    c2: JunctionBox,
}

impl JunctionBoxConnection {
    fn new(c1: &JunctionBox, c2: &JunctionBox) -> Self {
        let distance = c1.distance(c2);
        JunctionBoxConnection {
            distance,
            c1: *c1,
            c2: *c2,
        }
    }
}

trait JunctionBoxes {
    fn len(&self) -> usize;
    fn all_pairs(&self) -> impl Connections;
}

trait Connections {
    fn join_circuits(&mut self, num_circuits: usize) -> impl Circuits;
    fn join_all_circuits(
        &mut self,
        boxes: &impl JunctionBoxes,
    ) -> Option<(JunctionBox, JunctionBox)>;
}

trait Circuits {
    fn add_connection(&mut self, conn: &JunctionBoxConnection);
    fn product_top_circuits(&mut self, take: usize) -> usize;
}

impl JunctionBoxes for Vec<JunctionBox> {
    fn len(&self) -> usize {
        self.len()
    }

    fn all_pairs(&self) -> impl Connections {
        let mut connections = BinaryHeap::new();
        for c in self
            .iter()
            .combinations(2)
            .map(|pair| JunctionBoxConnection::new(pair[0], pair[1]))
        {
            connections.push(Reverse(c));
        }
        connections
    }
}

impl Connections for BinaryHeap<Reverse<JunctionBoxConnection>> {
    fn join_circuits(&mut self, num_circuits: usize) -> impl Circuits {
        // let mut pairs = self.iter();
        let mut circuits: DisjointSet<JunctionBox> = DisjointSet::new();
        for _ in 0..num_circuits {
            let conn = self.pop().unwrap().0;
            circuits.add_connection(&conn);
        }

        circuits
    }

    fn join_all_circuits(
        &mut self,
        boxes: &impl JunctionBoxes,
    ) -> Option<(JunctionBox, JunctionBox)> {
        let mut circuits: DisjointSet<JunctionBox> = DisjointSet::new();
        loop {
            let conn = self.pop()?.0;
            circuits.add_connection(&conn);

            if circuits.len() == 1 && circuits.iter().next().unwrap().len() == boxes.len() {
                return Some((conn.c1, conn.c2));
            }
        }
    }
}

impl Circuits for DisjointSet<JunctionBox> {
    fn add_connection(&mut self, conn: &JunctionBoxConnection) {
        self.insert(&conn.c1, &conn.c2)
    }

    fn product_top_circuits(&mut self, take: usize) -> usize {
        self.sort();
        self.iter().take(take).map(|c| c.len()).product()
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day8::{Circuits, Connections, JunctionBox, JunctionBoxes, parse};

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
        assert_eq!(103401, p.distance(&q));
    }

    #[test]
    fn test_join_circuits() {
        let circuits = parse(TEST_INPUT);
        let prod: usize = circuits
            .all_pairs()
            .join_circuits(10)
            .product_top_circuits(3);
        assert_eq!(40, prod);
    }

    #[test]
    fn test_join_all_circuits() {
        let circuits = parse(TEST_INPUT);
        let last_join = circuits.all_pairs().join_all_circuits(&circuits).unwrap();
        assert_eq!(25272, last_join.0.wall_distance(&last_join.1));
    }
}
