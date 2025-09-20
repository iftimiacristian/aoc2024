mod input;
mod solution;

use common::{Day, DayContext, InputMode, ParseError};

pub use solution::Day01;

/// Infrastructure adapter that bridges the pure solution logic with the AoC framework
pub struct Day01Solution {
    solution: Day01,
}

impl Day for Day01Solution {
    fn parse_input(input: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let (left, right) = input::parse_input(input)?;
        let solution = Day01::new(left, right);
        Ok(Day01Solution { solution })
    }

    fn part1(&self) -> String {
        self.solution.part1().to_string()
    }

    fn part2(&self) -> String {
        self.solution.part2().to_string()
    }

    fn day_number(&self) -> u8 {
        1
    }

    fn memory_usage_estimate(&self) -> usize {
        let input_lines = self.solution.input_size();
        std::mem::size_of::<i32>() * input_lines * 3
    }
}

pub fn build_day_01_solution(context: DayContext) -> Box<dyn Day> {
    Box::new(Day01Solution::new(context, InputMode::Full).expect("Failed to create Day 1 solution"))
}
