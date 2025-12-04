use std::{
    fmt,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new<T>(x: T, y: T) -> Point
    where
        T: TryInto<i64>,
        <T as TryInto<i64>>::Error: fmt::Debug,
    {
        Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        *self + *rhs
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn point(&self) -> &Point {
        const NORTH: Point = Point { x: 0, y: -1 };
        const SOUTH: Point = Point { x: 0, y: 1 };
        const EAST: Point = Point { x: 1, y: 0 };
        const WEST: Point = Point { x: -1, y: 0 };
        const NORTH_EAST: Point = Point { x: 1, y: -1 };
        const NORTH_WEST: Point = Point { x: -1, y: -1 };
        const SOUTH_EAST: Point = Point { x: 1, y: 1 };
        const SOUTH_WEST: Point = Point { x: -1, y: 1 };

        match self {
            Direction::North => &NORTH,
            Direction::NorthEast => &NORTH_EAST,
            Direction::East => &EAST,
            Direction::SouthEast => &SOUTH_EAST,
            Direction::South => &SOUTH,
            Direction::SouthWest => &SOUTH_WEST,
            Direction::West => &WEST,
            Direction::NorthWest => &NORTH_WEST,
        }
    }

    pub fn iter() -> impl Iterator<Item = Point> {
        [
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
        .iter()
        .map(|d| d.point())
        .copied()
    }
}

#[derive(Debug)]
pub struct Grid<T: Copy> {
    grid: Vec<Vec<T>>,
}

impl<T: Copy> fmt::Display for Grid<T> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub enum GridError {
    MismatchedColumns,
    PointNotInGrid,
}

impl<T: Copy> Grid<T> {
    fn new(rows: Vec<Vec<T>>) -> Result<Self, GridError> {
        let col_num = match rows.get(0) {
            Some(r) => r.len(),
            None => 0,
        };
        for r in rows.iter() {
            if r.len() != col_num {
                return Err(GridError::MismatchedColumns);
            }
        }

        return Ok(Self { grid: rows });
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        match self.grid.get(0) {
            Some(r) => r.len(),
            None => 0,
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= 0
            && p.x < self.cols().try_into().unwrap()
            && p.y >= 0
            && p.y < self.rows().try_into().unwrap()
    }

    pub fn get(&self, p: Point) -> Option<T> {
        if !self.contains(p) {
            return None;
        }

        let row_num: usize = p.y.try_into().unwrap();
        let col_num: usize = p.x.try_into().unwrap();
        Some(self.grid[row_num][col_num])
    }

    pub fn walk(&self) -> impl Iterator<Item = (Point, T)> {
        (0..self.rows()).flat_map(move |r| {
            (0..self.cols()).map(move |c| {
                let p = Point::new(c, r);
                let v = self.get(p).unwrap();
                (p, v)
            })
        })
    }

    pub fn iter_adjacent(&self, p: Point) -> impl Iterator<Item = (Point, T)> {
        Direction::iter()
            .map(move |d| (p + d, self.get(p + d)))
            .filter(|(_, v)| v.is_some())
            .map(|(q, v)| (q, v.unwrap()))
    }

    pub fn update(&mut self, p: Point, v: T) -> Result<(), GridError> {
        if !self.contains(p) {
            return Err(GridError::PointNotInGrid);
        }

        let row_num: usize = p.y.try_into().unwrap();
        let col_num: usize = p.x.try_into().unwrap();
        self.grid[row_num][col_num] = v;
        Ok(())
    }

    pub fn swap(&mut self, p: Point, q: Point) -> Result<(), GridError> {
        let a = match self.get(p) {
            Some(v) => v,
            None => return Err(GridError::PointNotInGrid),
        };

        let b = match self.get(q) {
            Some(v) => v,
            None => return Err(GridError::PointNotInGrid),
        };

        self.update(p, b).unwrap();
        self.update(q, a).unwrap();
        Ok(())
    }
}

impl FromStr for Grid<char> {
    type Err = GridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<char>> = Vec::new();
        rows.extend(input.split('\n').map(|line| {
            let line = line.trim();
            let mut row: Vec<char> = Vec::new();
            row.extend(line.chars());
            row
        }));
        Grid::new(rows)
    }
}

impl FromStr for Grid<u32> {
    type Err = GridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        rows.extend(input.split('\n').map(|line| {
            let mut row: Vec<u32> = Vec::new();
            row.extend(
                line.chars()
                    .map(|c| c.to_digit(10).expect("Failed to read digit")),
            );
            row
        }));
        Grid::new(rows)
    }
}
