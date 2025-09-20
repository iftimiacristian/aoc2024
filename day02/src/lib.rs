mod input;
mod solution;

use common::{Day, DayContext, InputMode, ParseError};
pub use solution::Day02;
use std::mem::size_of;

pub struct Day02Solution {
    solution: Day02,
}

impl Day for Day02Solution {
    fn parse_input(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let reports = input::parse_input(input)?;
        let solution = Day02::new(reports);
        Ok(Day02Solution { solution })
    }

    fn part1(&self) -> String {
        self.solution.part1().to_string()
    }

    fn part2(&self) -> String {
        self.solution.part2().to_string()
    }

    fn day_number(&self) -> u8 {
        2
    }

    fn memory_usage_estimate(&self) -> usize {
        let total_elements = self.solution.total_elements();
        size_of::<i32>() * total_elements
    }
}

pub fn build_day_02_solution(context: DayContext) -> Box<dyn Day> {
    Box::new(Day02Solution::new(context, InputMode::Full).expect("Failed to create Day 2 solution"))
}
