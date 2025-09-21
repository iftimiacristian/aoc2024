mod input;
mod solution;

use common::{Day, DayContext, InputMode, ParseError};

pub use solution::Day03;

/// Infrastructure adapter that bridges the pure solution logic with the AoC framework
pub struct Day03Solution {
    solution: Day03,
}

impl Day for Day03Solution {
    fn parse_input(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let parsed_input = input::parse_input(input)?;
        let solution = Day03::new(parsed_input);
        Ok(Day03Solution { solution })
    }

    fn part1(&self) -> String {
        self.solution.part1().to_string()
    }

    fn part2(&self) -> String {
        self.solution.part2().to_string()
    }

    fn day_number(&self) -> u8 {
        3
    }

    fn memory_usage_estimate(&self) -> usize {
        // TODO: Implement memory usage estimate
        0
    }
}

pub fn build_day_03_solution(context: DayContext) -> Box<dyn Day> {
    Box::new(Day03Solution::new(context, InputMode::Full).expect("Failed to create Day 03 solution"))
}
