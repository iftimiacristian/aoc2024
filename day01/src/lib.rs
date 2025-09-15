use common::{Day, DayContext, InputMode};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define a struct that will hold the parsed data and implement the Day trait
pub struct Day01Solution {
    column1_data: Vec<i32>,
    column2_data: Vec<i32>,
}

impl Day01Solution {
    // Constructor for Day01Solution, taking a DayContext
    pub fn new(context: DayContext, mode: InputMode) -> Result<Self, Box<dyn std::error::Error>> {
        let reader = context.load_input(mode)?;
        let (mut column1_data, mut column2_data) = Self::parse_input(reader)?;

        column1_data.sort_unstable();
        column2_data.sort_unstable();

        Ok(Day01Solution {
            column1_data,
            column2_data,
        })
    }

    fn parse_input(reader: BufReader<File>) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
        reader
            .lines()
            .map(|line| {
                let line = line?;
                let nums = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()?;

                if nums.len() != 2 {
                    return Err(format!("Expected 2 numbers, got {}", nums.len()).into());
                }

                Ok((nums[0], nums[1]))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|pairs| pairs.into_iter().unzip())
    }
}

// Implement the Day trait for Day01Solution
impl Day for Day01Solution {
    fn part1(&self) {
        println!("{}", self.distance_sum());
    }

    fn part2(&self) {
        println!("{}", self.similarity_score());
    }
}

impl Day01Solution {

    pub fn distance_sum(&self) -> i32 {
        self.column1_data
            .iter()
            .zip(self.column2_data.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    pub fn similarity_score(&self) -> i32 {
        // Step 1: Count frequencies in column2_data - O(n)
        let right_counts: HashMap<i32, i32> = self.column2_data
            .iter()
            .fold(HashMap::new(), |mut acc, &num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

        // Step 2: Calculate similarity using lookups - O(n)
        self.column1_data
            .iter()
            .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
            .sum()
    }

}

pub fn build_day_01_solution(context: common::DayContext) -> Box<dyn common::Day> {
    Box::new(Day01Solution::new(context, InputMode::Full)
        .expect("Failed to create Day 1 solution"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::DayContext;

    #[test]
    fn part1_example() {
        let day_context = DayContext::new(1); // Create a DayContext for Day 1
        let solution = Day01Solution::new(day_context, InputMode::Example)
            .expect("Failed to create solution for test"); // Instantiate your solution struct

        let expected_output = 11;
        assert_eq!(solution.distance_sum(), expected_output); // Call part1 on the solution instance
    }

    #[test]
    fn part2_example() {
        let mut day_context = DayContext::new(1);
        day_context.next_part();
        let solution = Day01Solution::new(day_context, InputMode::Example)
            .expect("Failed to create solution for test");
        let expected_output = 31;
        assert_eq!(solution.similarity_score(), expected_output);
    }
}
