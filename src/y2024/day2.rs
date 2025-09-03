const INPUT: &str = include_str!("input/day2.txt");

struct Report(Vec<i32>);

impl Report {
    fn from(input: &str) -> Report {
        let mut report: Vec<i32> = Vec::new();
        report.extend(
            input
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap()),
        );
        Report(report)
    }

    fn is_safe(&self) -> bool {
        let differences = self
            .0
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx > 0)
            .map(|(idx, x)| *x - self.0[idx - 1]);

        let mut is_increasing: Option<bool> = Option::None;
        for x in differences {
            let absx = x.abs();
            if absx < 1 || absx > 3 {
                return false;
            }
            match is_increasing {
                None => {
                    is_increasing = Option::Some(x > 0);
                    continue;
                }
                Some(k) => {
                    if (k && x < 0) || (!k && x > 0) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn count_safe<'a>(reports: impl Iterator<Item = &'a Report>) -> usize {
        reports.filter(|r| r.is_safe()).count()
    }

    fn is_safe_with_removal(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for idx in 0..self.0.len() {
            let mut report_clone = Report(self.0.clone());
            report_clone.0.remove(idx);
            if report_clone.is_safe() {
                return true;
            }
        }

        return false;
    }

    fn count_safe_with_removal<'a>(reports: impl Iterator<Item = &'a Report>) -> usize {
        reports.filter(|r| r.is_safe_with_removal()).count()
    }
}

fn parse(input: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    reports.extend(input.split('\n').map(|x| Report::from(x)));
    reports
}

pub fn run() -> (u64, u64) {
    let reports = parse(INPUT);
    return (
        Report::count_safe(reports.iter()) as u64,
        Report::count_safe_with_removal(reports.iter()) as u64,
    );
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::y2024::day2::{INPUT, Report, parse};

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", false)]
    #[case("8 6 4 4 1", false)]
    #[case("1 3 6 7 9", true)]
    fn test_is_safe(#[case] input: &str, #[case] expected: bool) {
        let report = Report::from(input);
        assert_eq!(expected, report.is_safe());
    }

    #[test]
    fn test_count_safe() {
        let reports = parse(TEST_INPUT);
        assert_eq!(2, Report::count_safe(reports.iter()));
    }

    #[test]
    fn test_solution_part_1() {
        let reports = parse(INPUT);
        assert_eq!(356, Report::count_safe(reports.iter()));
    }

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", true)]
    #[case("8 6 4 4 1", true)]
    #[case("1 3 6 7 9", true)]
    fn test_is_safe_with_removal(#[case] input: &str, #[case] expected: bool) {
        let report = Report::from(input);
        assert_eq!(expected, report.is_safe_with_removal());
    }

    #[test]
    fn test_count_safe_with_removal() {
        let reports = parse(TEST_INPUT);
        assert_eq!(4, Report::count_safe_with_removal(reports.iter()));
    }

    #[test]
    fn test_solution_part_2() {
        let reports = parse(INPUT);
        assert_eq!(413, Report::count_safe_with_removal(reports.iter()));
    }
}
