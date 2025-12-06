use std::str::FromStr;

use crate::helpers::grid::Grid;

const INPUT: &str = include_str!("input/day6.txt");

pub fn run() -> (u64, u64) {
    let worksheet: Worksheet = INPUT.parse().unwrap();
    let worksheet2 = Worksheet::from_grid(&(INPUT.parse().unwrap()));
    (worksheet.caclulate(), worksheet2.caclulate())
}

#[derive(Debug)]
struct Worksheet {
    cols: Vec<Vec<u64>>,
    operations: Vec<char>,
}

#[derive(Debug)]
enum WorksheetParseError {
    EmptyInput,
}

impl FromStr for Worksheet {
    type Err = WorksheetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<&str> = s.split('\n').collect();
        let operations: Vec<char> = match rows.pop() {
            Some(s) => s
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.chars().next().unwrap())
                .collect(),
            None => return Err(WorksheetParseError::EmptyInput),
        };
        let count = operations.len();
        let rows: Vec<Vec<u64>> = rows
            .iter()
            .map(|s| {
                s.split(' ')
                    .filter(|x| !x.trim().is_empty())
                    .map(|x| x.trim().parse().unwrap())
                    .collect()
            })
            .collect();

        let cols: Vec<Vec<u64>> = (0..count)
            .map(|col| rows.iter().map(|r| r[col]).collect())
            .collect();
        Ok(Worksheet { cols, operations })
    }
}

impl Worksheet {
    fn from_grid(grid: &Grid<char>) -> Self {
        let mut operations: Vec<char> = Vec::new();
        let mut cols: Vec<Vec<u64>> = Vec::new();
        let max_row = grid.rows() - 1;
        for (p, v) in grid.walk_row(max_row) {
            if v != ' ' && v != '.' {
                operations.push(v);
                cols.push(Vec::new());
            }

            let col = cols.last_mut().unwrap();
            let mut q = p.up();
            let mut base = 1;
            let mut val: u64 = 0;
            while grid.contains(q) {
                let v = grid.get(q).unwrap();
                if v != ' ' && v != '.' {
                    val += (v.to_digit(10).unwrap() as u64) * base;
                    base *= 10;
                }
                q = q.up();
            }
            if val != 0 {
                col.push(val);
            }
        }

        Worksheet { cols, operations }
    }

    fn len(&self) -> usize {
        self.operations.len()
    }

    fn calculate_column(&self, col: usize) -> u64 {
        let operands = self.cols[col].iter();
        let operation = self.operations[col];
        match operation {
            '+' => operands.sum(),
            '*' => operands.product(),
            _ => panic!("Unexpected operation {operation}"),
        }
    }

    fn caclulate(&self) -> u64 {
        (0..self.len()).map(|x| self.calculate_column(x)).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day6::Worksheet;

    const TEST_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_calculate() {
        let worksheet: Worksheet = TEST_INPUT.parse().unwrap();
        assert_eq!(4277556, worksheet.caclulate());
    }

    #[test]
    fn test_calculate_from_grid() {
        let grid = TEST_INPUT.parse().unwrap();
        let worksheet = Worksheet::from_grid(&grid);
        println!("{worksheet:?}");
        assert_eq!(3263827, worksheet.caclulate());
    }
}
