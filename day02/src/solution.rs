#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending,
    Mixed,
}

pub struct Report {
    values: Vec<i32>,
}

impl Report {
    pub fn new(values: Vec<i32>) -> Report {
        Self { values }
    }

    fn determine_direction(values: &[i32]) -> Direction {
        let mut has_ascending = false;
        let mut has_descending = false;

        for window in values.windows(2) {
            let v1 = window[0];
            let v2 = window[1];

            if v1 < v2 {
                has_ascending = true;
            } else if v1 > v2 {
                has_descending = true;
            }

            if has_ascending && has_descending {
                return Direction::Mixed;
            }
        }

        if has_ascending {
            Direction::Ascending
        } else if has_descending {
            Direction::Descending
        } else {
            Direction::Ascending // All equal values, but handled separately
        }
    }

    fn is_slice_valid(values: &[i32]) -> bool {
        const THRESHOLD: i32 = 3;

        if values.len() < 2 {
            return true;
        }

        let direction = Self::determine_direction(values);
        if direction == Direction::Mixed {
            return false;
        }

        for window in values.windows(2) {
            let v1 = window[0];
            let v2 = window[1];

            if (v1 - v2).abs() > THRESHOLD || v1 == v2 {
                return false;
            }
        }

        true
    }

    pub fn valid(&self, dampener: bool) -> bool {
        if !dampener {
            return Self::is_slice_valid(&self.values);
        }

        if self.values.len() <= 1 {
            return true;
        }

        for i in 0..self.values.len() {
            let slice = [&self.values[..i], &self.values[i+1..]].concat();
            if Self::is_slice_valid(&slice) {
                return true;
            }
        }

        false
    }
}

pub struct Day02 {
    reports: Vec<Report>,
}

impl Day02 {
    pub fn new(reports: Vec<Report>) -> Self {
        Self { reports }
    }

    pub fn input_size(&self) -> usize {
        self.reports.len()
    }

    pub fn total_elements(&self) -> usize {
        self.reports.iter().map(|report| report.values.len()).sum()
    }

    pub fn part1(&self) -> i32 {
        self.reports.iter().filter(|r| r.valid(false)).count() as i32
    }

    pub fn part2(&self) -> i32 {
        self.reports.iter().filter(|r| r.valid(true)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ReportTestCase {
        values: Vec<i32>,
        expected_part1: bool,
        expected_part2: bool,
    }

    fn get_test_cases() -> Vec<ReportTestCase> {
        vec![
            ReportTestCase {
                values: vec![7, 6, 4, 2, 1],
                expected_part1: true,
                expected_part2: true,
            },
            ReportTestCase {
                values: vec![1, 2, 7, 8, 9],
                expected_part1: false,
                expected_part2: false,
            },
            ReportTestCase {
                values: vec![9, 7, 6, 2, 1],
                expected_part1: false,
                expected_part2: false,
            },
            ReportTestCase {
                values: vec![1, 3, 2, 4, 5],
                expected_part1: false,
                expected_part2: true,
            },
            ReportTestCase {
                values: vec![8, 6, 4, 4, 1],
                expected_part1: false,
                expected_part2: true,
            },
            ReportTestCase {
                values: vec![1, 3, 6, 7, 9],
                expected_part1: true,
                expected_part2: true,
            },
        ]
    }

    #[test]
    fn test_report_valid() {
        let test_cases = get_test_cases();
        let mut reports: Vec<Report> = Vec::new();

        for case in &test_cases {
            let report = Report::new(case.values.clone());

            assert_eq!(
                report.valid(false),
                case.expected_part1,
                "Failed for {:?}",
                case.values,
            );

            reports.push(report);
        }

        let solution = Day02::new(reports);
        let expected_part1_count = test_cases.iter().filter(|c| c.expected_part1).count() as i32;
        assert_eq!(solution.part1(), expected_part1_count);
    }

    #[test]
    fn test_day02_part2() {
        let test_cases = get_test_cases();
        let mut reports: Vec<Report> = Vec::new();

        for case in &test_cases {
            let report = Report::new(case.values.clone());
            assert_eq!(
                report.valid(true),
                case.expected_part2,
                "Failed for {:?} with dampener",
                case.values,
            );
            reports.push(report);
        }

        let solution = Day02::new(reports);
        let expected_part2_count = test_cases.iter().filter(|c| c.expected_part2).count() as i32;
        assert_eq!(solution.part2(), expected_part2_count);
    }
}
