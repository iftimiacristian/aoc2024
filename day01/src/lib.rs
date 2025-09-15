mod input;
mod solution;

use common::{Day, DayContext, InputMode};
use std::io::Read;

pub use solution::Day01;

/// Infrastructure adapter that bridges the pure solution logic with the AoC framework
pub struct Day01Solution {
    solution: Day01,
}

impl Day01Solution {
    /// Create a new Day01Solution by loading and parsing input
    pub fn new(context: DayContext, mode: InputMode) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = context.load_input(mode)?;
        let mut input_content = String::new();
        reader.read_to_string(&mut input_content)?;

        let (left, right) = input::parse_input(&input_content)?;

        let solution = Day01::new(left, right);

        Ok(Day01Solution { solution })
    }
}

impl Day for Day01Solution {
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

pub fn build_day_01_solution(context: common::DayContext) -> Box<dyn common::Day> {
    Box::new(Day01Solution::new(context, InputMode::Full).expect("Failed to create Day 1 solution"))
}
