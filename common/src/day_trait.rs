// common/src/day_trait.rs

use crate::{DayContext, InputMode, ParseError};
use std::io::Read;

pub trait Day {
    fn new(context: DayContext, mode: InputMode) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut reader = context.load_input(mode)?;
        let mut input_content = String::new();
        reader.read_to_string(&mut input_content)?;

        let solution = Self::parse_input(&input_content)?;

        Ok(solution)
    }

    fn parse_input(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn part1(&self) -> String;

    fn part2(&self) -> String;

    fn day_number(&self) -> u8;

    /// Optional: Estimated memory usage in bytes
    /// Default implementation returns 1MB
    fn memory_usage_estimate(&self) -> usize {
        1024 * 1024
    }
}
