use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
    NumberParse(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::NumberParse(err)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
            ParseError::NumberParse(err) => write!(f, "Number parse error: {err}"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), ParseError> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Result<Vec<i32>, ParseIntError> =
                line.split_whitespace().map(|s| s.parse::<i32>()).collect();

            let nums = nums?;

            if nums.len() != 2 {
                return Err(ParseError::InvalidFormat(format!(
                    "Expected 2 numbers per line, got {}: '{line}'",
                    nums.len()
                )));
            }

            Ok((nums[0], nums[1]))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|pairs| pairs.into_iter().unzip())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_input() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        let (left, right) = parse_input(input).unwrap();
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let input = "3   4\n\n4   3\n\n";
        let (left, right) = parse_input(input).unwrap();
        assert_eq!(left, vec![3, 4]);
        assert_eq!(right, vec![4, 3]);
    }

    #[test]
    fn test_parse_invalid_format() {
        let input = "1 2 3"; // Too many numbers
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        let input = "1 abc";
        assert!(parse_input(input).is_err());
    }
}
