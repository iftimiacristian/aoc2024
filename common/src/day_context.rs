// common/src/day_context.rs

use std::path::PathBuf;
use super::enums::{Part, InputMode};

pub struct DayContext {
    pub day: u8,
    pub part: Part,
}

impl DayContext {
    pub fn new(day: u8) -> Self {
        DayContext {
            day,
            part: Part::One,
        }
    }

    pub fn set_part(&mut self, part: Part) {
        self.part = part;
    }

    pub fn next_part(&mut self) {
        self.part = Part::Two;
    }

    fn get_input_file_path(&self, mode: InputMode) -> PathBuf {
        let suffix = match mode {
            InputMode::Example => "example",
            InputMode::Full => "full",
        };
        let prefix = match self.part {
            Part::One => "1",
            Part::Two => "2",
        };

        let day_padded = format!("{:02}", self.day);
        let path = format!("inputs/day{day_padded}/{prefix}.{suffix}.txt");

        PathBuf::from(path)
    }

    pub fn load_input(&self, mode: InputMode) -> Result<String, std::io::Error> {
        let path = self.get_input_file_path(mode);
        std::fs::read_to_string(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct FilePathTestCase {
        day: u8,
        part: Part,
        mode: InputMode,
        expected: &'static str,
    }

    #[test]
    fn test_get_input_file_path_table_driven() {
        let test_cases = vec![
            FilePathTestCase {
                day: 1,
                part: Part::One,
                mode: InputMode::Example,
                expected: "inputs/day01/1.example.txt",
            },
            FilePathTestCase {
                day: 2,
                part: Part::Two,
                mode: InputMode::Full,
                expected: "inputs/day02/2.full.txt",
            },
            FilePathTestCase {
                day: 10,
                part: Part::One,
                mode: InputMode::Full,
                expected: "inputs/day10/1.full.txt",
            },
            FilePathTestCase {
                day: 20,
                part: Part::Two,
                mode: InputMode::Example,
                expected: "inputs/day20/2.example.txt",
            }
        ];

        for case in test_cases {
            let mut day_context = DayContext::new(case.day);
            day_context.set_part(case.part);
            let actual_path = day_context.get_input_file_path(case.mode);
            let expected_path_buf = PathBuf::from(case.expected);

            assert_eq!(
                actual_path, expected_path_buf,
                "Failed for day={}, part={:?}, mode={:?}", case.day, case.part, case.mode
            );
        }
    }
}
