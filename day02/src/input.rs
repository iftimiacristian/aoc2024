use crate::solution::Report;
use common::ParseError;
use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Result<Vec<Report>, ParseError> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Result<Vec<i32>, ParseIntError> =
                line.split_whitespace().map(|s| s.parse::<i32>()).collect();

            Ok(Report::new(nums?))
        })
        .collect::<Result<Vec<_>, _>>()
}
