use std::{fs::File, io::Read};

type Report = Vec<i32>;

const INPUT_FILE: &str = "C:\\Users\\awlod\\code\\aoco3\\adventofcode\\src\\y2024\\input\\day2.txt";

fn is_safe(report: &Report) -> bool {
    let differences = report
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx > 0)
        .map(|(idx, x)| *x - report[idx - 1]);

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
    reports.filter(|r| is_safe(*r)).count()
}

fn is_safe_with_removal(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    for idx in 0..report.len() {
        let mut report_clone = report.clone();
        report_clone.remove(idx);
        if is_safe(&report_clone) {
            return true;
        }
    }

    return false;
}

fn count_safe_with_removal<'a>(reports: impl Iterator<Item = &'a Report>) -> usize {
    reports.filter(|r| is_safe_with_removal(*r)).count()
}

fn parse(input: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    reports.extend(input.split('\n').map(|x| parse_report(x)));
    reports
}

fn parse_file(filename: &str) -> Vec<Report> {
    let mut fp = File::open(filename).expect("File {filename} not found");
    let mut contents = String::new();
    fp.read_to_string(&mut contents).unwrap();
    return parse(&contents);
}

fn parse_report(input: &str) -> Report {
    let mut report: Vec<i32> = Vec::new();
    report.extend(
        input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap()),
    );
    report
}

pub fn run() -> (usize, usize) {
    let reports = parse_file(INPUT_FILE);
    return (
        count_safe(reports.iter()),
        count_safe_with_removal(reports.iter()),
    );
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::y2024::day2::{
        INPUT_FILE, Report, count_safe, count_safe_with_removal, is_safe, is_safe_with_removal,
        parse, parse_file, parse_report,
    };

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
        let report: Report = parse_report(input);
        assert_eq!(expected, is_safe(&report));
    }

    #[test]
    fn test_count_safe() {
        let reports = parse(TEST_INPUT);
        assert_eq!(2, count_safe(reports.iter()));
    }

    #[test]
    fn test_solution_part_1() {
        let reports = parse_file(INPUT_FILE);
        assert_eq!(356, count_safe(reports.iter()));
    }

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", true)]
    #[case("8 6 4 4 1", true)]
    #[case("1 3 6 7 9", true)]
    fn test_is_safe_with_removal(#[case] input: &str, #[case] expected: bool) {
        let report: Report = parse_report(input);
        assert_eq!(expected, is_safe_with_removal(&report));
    }

    #[test]
    fn test_count_safe_with_removal() {
        let reports = parse(TEST_INPUT);
        assert_eq!(4, count_safe_with_removal(reports.iter()));
    }

    #[test]
    fn test_solution_part_2() {
        let reports = parse_file(INPUT_FILE);
        assert_eq!(413, count_safe_with_removal(reports.iter()));
    }
}
