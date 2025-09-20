pub struct Report {
    values: Vec<i32>,
}

impl Report {
    pub fn new(values: Vec<i32>) -> Report {
        Self { values }
    }

    fn is_slice_valid(values: &[i32]) -> bool {
        const THRESHOLD: i32 = 3;

        if values.len() < 2 {
            return true;
        }

        let mut is_ascending = true;
        let mut is_descending = true;

        for window in values.windows(2) {
            let v1 = window[0];
            let v2 = window[1];

            if (v1 - v2).abs() > THRESHOLD {
                return false;
            }

            if v1 == v2 {
                return false;
            }

            if v1 > v2 {
                is_ascending = false;
            }

            if v1 < v2 {
                is_descending = false;
            }

            if !is_ascending && !is_descending {
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
            let mut temp_values = self.values.clone();
            temp_values.remove(i);
            if Self::is_slice_valid(&temp_values) {
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
        let mut valid = 0;
        for report in &self.reports {
            valid += report.valid(false) as i32;
        }

        valid
    }

    pub fn part2(&self) -> i32 {
        let mut valid = 0;
        for report in &self.reports {
            valid += report.valid(true) as i32;
        }

        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ReportTestCase {
        values: Vec<i32>,
        expected: bool,
    }

    #[test]
    fn test_report_valid() {
        let test_cases = vec![
            ReportTestCase {
                values: vec![7, 6, 4, 2, 1],
                expected: true,
            },
            ReportTestCase {
                values: vec![1, 2, 7, 8, 9],
                expected: false,
            },
            ReportTestCase {
                values: vec![9, 7, 6, 2, 1],
                expected: false,
            },
            ReportTestCase {
                values: vec![1, 3, 2, 4, 5],
                expected: false,
            },
            ReportTestCase {
                values: vec![8, 6, 4, 4, 1],
                expected: false,
            },
            ReportTestCase {
                values: vec![1, 3, 6, 7, 9],
                expected: true,
            },
        ];

        let mut values: Vec<Report> = Vec::new();

        for case in test_cases {
            let report = Report::new(case.values.clone());

            assert_eq!(
                report.valid(false),
                case.expected,
                "Failed for {:?}",
                case.values,
            );

            values.push(report);
        }

        let solution = Day02::new(values);

        assert_eq!(solution.part1(), 2)
    }

    #[test]
    fn test_day02_part2() {
        let test_cases = vec![
            ReportTestCase {
                values: vec![7, 6, 4, 2, 1],
                expected: true,
            },
            ReportTestCase {
                values: vec![1, 2, 7, 8, 9],
                expected: false,
            },
            ReportTestCase {
                values: vec![9, 7, 6, 2, 1],
                expected: false,
            },
            ReportTestCase {
                values: vec![1, 3, 2, 4, 5],
                expected: true,
            },
            ReportTestCase {
                values: vec![8, 6, 4, 4, 1],
                expected: true,
            },
            ReportTestCase {
                values: vec![1, 3, 6, 7, 9],
                expected: true,
            },
        ];

        let mut reports: Vec<Report> = Vec::new();
        let mut expected_valid_count = 0;
        for case in test_cases {
            let report = Report::new(case.values.clone());
            assert_eq!(
                report.valid(true),
                case.expected,
                "Failed for {:?} with dampener",
                case.values,
            );
            if case.expected {
                expected_valid_count += 1;
            }
            reports.push(report);
        }

        let solution = Day02::new(reports);
        assert_eq!(solution.part2(), expected_valid_count);
    }
}
